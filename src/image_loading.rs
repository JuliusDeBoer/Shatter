use image::{DynamicImage, ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

use crate::steps::{StepAttributes, STEP_MANAGER};

#[wasm_bindgen(js_name = loadImage)]
pub fn load_image(image_data: ImageData) -> usize {
    let mut binding = STEP_MANAGER.lock();
    let manager = binding.as_mut().unwrap();

    let width = image_data.width();
    let height = image_data.height();
    let data = image_data.data().to_vec();

    manager.register(Box::from(move |attr: &mut StepAttributes| {
        let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, data)
            .map(DynamicImage::ImageRgba8);
        attr.image_buffer = image.expect("Could not parse image");
    }))
}
