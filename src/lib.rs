#![feature(fn_traits)]

#[macro_use]
extern crate lazy_static;

mod image_loading;
mod mask;
mod manager;
mod utils;

use image::DynamicImage;
use manager::{StepAttributes, STEP_MANAGER};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

#[cfg(debug_assertions)]
use utils::set_panic_hook;

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

#[cfg(debug_assertions)]
#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}

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
            manager.run(step, &mut attributes);
        }

        let image_buffer = attributes.image_buffer;

        let output = image_buffer.clone().into_rgba8();

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
    }

    pub fn step(mut self, handle: usize) -> Self {
        self.steps.push(handle);
        self
    }
}
