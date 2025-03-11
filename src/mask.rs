use image::GrayImage;
use wasm_bindgen::prelude::*;

use crate::manager::{StepAttributes, MASK_MANAGER, STEP_MANAGER};

/// A simple step to display the result of a mask. Mosty for debugging
#[wasm_bindgen(js_name = showMask)]
pub fn show_mask(mask: usize) -> usize {
    let mut binding = STEP_MANAGER.lock();
    let manager = binding
        .as_mut()
        .expect("Could not get lock for step manager");

    manager.register(Box::from(move |attr: &mut StepAttributes| {
        let mut binding = MASK_MANAGER.lock();
        let mask_manager = binding
            .as_mut()
            .expect("Could not get lock for mask manager");

        attr.image_buffer = mask_manager
            .run(&mask, attr)
            .expect("Could not create mask");
    }))
}

/// Creates a contrast mask
#[wasm_bindgen(js_name = contrastMask)]
pub fn contrast_mask(lower_bounds: f32, higher_bounds: f32) -> usize {
    let mut binding = MASK_MANAGER.lock();
    let manager = binding.as_mut().expect("Could to get mask manager lock");

    manager.register(Box::from(move |attr: &mut StepAttributes| {
        let width = attr.image_buffer.width();
        let height = attr.image_buffer.height();

        let mut luma = attr.image_buffer.to_luma32f();
        let pixels = luma
            .iter_mut()
            .map(|p| match *p >= lower_bounds && *p <= higher_bounds {
                true => 255u8,
                false => 0u8,
            });
        image::DynamicImage::ImageLuma8(
            GrayImage::from_vec(width, height, pixels.collect())
                .expect("Could not convert back into image. Array might not be long enough"),
        )
    }))
}
