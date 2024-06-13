#![feature(fn_traits)]

mod utils;

use std::sync::Mutex;

use image::{DynamicImage, ImageBuffer, Rgba};
use utils::set_panic_hook;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);

    #[wasm_bindgen(extends = CanvasRenderingContext2d)]
    #[derive(Clone)]
    pub type CanvasRenderingContext2D;

    #[wasm_bindgen(method)]
    pub fn fillRect(this: &CanvasRenderingContext2D, x: u16, y: u16, width: u16, height: u16);
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}

pub struct StepAttributes {
    pub image_buffer: DynamicImage,
}

#[derive(Default)]
pub struct StepManager {
    // TODO: Make this a dictionary so that the indices dont "decay"
    steps: Vec<Box<dyn FnOnce(&mut StepAttributes) + Send + 'static>>,
}

impl StepManager {
    pub fn register(
        &mut self,
        step: Box<dyn FnOnce(&mut StepAttributes) + Send + 'static>,
    ) -> usize {
        self.steps.push(step);
        self.steps.len() - 1
    }

    pub fn run(&mut self, handle: usize, attr: &mut StepAttributes) {
        let func = self.steps.remove(handle);
        func.call_once((attr,));
    }
}

static STEP_MANAGER: Mutex<StepManager> = Mutex::new(StepManager { steps: Vec::new() });

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct RenderSettings {
    context: Option<CanvasRenderingContext2d>,
    steps: Vec<usize>,
    dimensions: Option<(u32, u32)>,
}

#[wasm_bindgen]
impl RenderSettings {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RenderSettings {
        RenderSettings::default()
    }

    #[wasm_bindgen(js_name = withContext)]
    pub fn with_context(mut self, context: CanvasRenderingContext2d) -> Self {
        self.context = Some(context);
        self
    }

    #[wasm_bindgen(js_name = withDimensions)]
    pub fn with_dimensions(mut self, x: u32, y: u32) -> Self {
        self.dimensions = Some((x, y));
        self
    }

    pub fn render(self) {
        self.context.clone().expect("Context where not provided");
        self.dimensions.expect("Dimensions where not provided");

        let mut attributes = StepAttributes {
            image_buffer: DynamicImage::new(
                self.dimensions.unwrap().0,
                self.dimensions.unwrap().1,
                image::ColorType::Rgba8,
            ),
        };

        let mut binding = STEP_MANAGER.lock();
        let manager = binding.as_mut().unwrap();

        for step in &self.steps {
            manager.run(*step, &mut attributes);
        }

        let image_buffer = attributes.image_buffer;
        if let DynamicImage::ImageRgba8(output) = image_buffer.clone() {
            let data_again = ImageData::new_with_u8_clamped_array_and_sh(
                Clamped(&output.into_raw()[..]),
                image_buffer.width(),
                image_buffer.height(),
            )
            .unwrap();

            self.context
                .as_ref()
                .unwrap()
                .put_image_data(&data_again, 0., 0.)
                .unwrap();
        } else {
            panic!("Unexpected image format");
        }
    }

    pub fn step(mut self, handle: usize) -> Self {
        self.steps.push(handle);
        self
    }
}

#[wasm_bindgen(js_name = loadImage)]
pub fn load_image(image_data: ImageData) -> usize {
    let mut binding = STEP_MANAGER.lock();
    let manager = binding.as_mut().unwrap();

    let width = image_data.width();
    let height = image_data.height();
    let data = image_data.data().to_vec();

    debug("Registering the closure");

    manager.register(Box::from(move |attr: &mut StepAttributes| {
        debug("Running the closure");
        let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, data)
            .map(DynamicImage::ImageRgba8);
        attr.image_buffer = image.expect("Could not parse image");
    }))
}
