mod utils;

use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

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

#[wasm_bindgen]
pub struct StepAttributes {
    #[wasm_bindgen(skip)]
    pub image_buffer: RgbaImage,
}

#[wasm_bindgen(getter_with_clone)]
pub struct RenderSettings {
    context: Option<CanvasRenderingContext2d>,
    steps: Vec<js_sys::Function>,
    dimensions: Option<(u32, u32)>,
}

#[wasm_bindgen]
impl RenderSettings {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RenderSettings {
        RenderSettings {
            context: None,
            steps: Vec::new(),
            dimensions: None,
        }
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

    pub fn render(&mut self) {
        self.context.clone().expect("Context where not provided");
        self.dimensions.expect("Dimensions where not provided");

        let image_buffer = RgbaImage::new(self.dimensions.unwrap().0, self.dimensions.unwrap().1);
        let attributes = JsValue::from(StepAttributes {
            image_buffer,
        });


        for step in &self.steps {
            step.call0(&attributes).unwrap();
        }


        // if let DynamicImage::ImageRgba8(output) = attributes.into::<StepAttributes>().unwrap.image_buffer.into() {
        //     let mut imageData = ImageData::new_with_sw(image_buffer.width(), image_buffer.height()).unwrap();
        //     imageData.data().clear();
        //     imageData.data().append(&mut output.into_raw())
        // } else {
        //     panic!("Unexpected image format.");
        // }
    }

    pub fn step(mut self, step: js_sys::Function) -> Self {
        self.steps.push(step);
        self
    }
}

#[wasm_bindgen(js_name = loadImage)]
pub fn load_image(image_data: ImageData) -> js_sys::Function {
    Closure::once_into_js(move |mut attr: StepAttributes| {
        let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            image_data.width(),
            image_data.height(),
            image_data.data().to_vec(),
        )
        .map(DynamicImage::ImageRgba8);
        attr.image_buffer = image.expect("Could not parse image").into();
    })
    .into()
}
