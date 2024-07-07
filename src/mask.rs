use image::GrayImage;
use wasm_bindgen::prelude::*;

use crate::manager::{StepAttributes, MASK_MANAGER, STEP_MANAGER};

/// A simple step to display the result of a mask. Mosty for debugging
#[wasm_bindgen(js_name = showMask)]
pub fn show_mask(mask: usize) -> usize {
    let mut binding = STEP_MANAGER.lock();
    let manager = binding.as_mut().unwrap();

    manager.register(Box::from(move |attr: &mut StepAttributes| {
        let mut binding = MASK_MANAGER.lock();
        let mask_manager = binding.as_mut().unwrap();

        attr.image_buffer = mask_manager.run(&mask, attr).unwrap();
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

        // The to_luma32f function is slow. Maybe write your own implementation
        let mut luma = attr.image_buffer.to_luma32f();
        let pixels = luma.iter_mut().map(|p| {
            if *p >= lower_bounds && *p <= higher_bounds {
                return 255u8;
            }

            0u8
        });
        image::DynamicImage::ImageLuma8(
            GrayImage::from_vec(width, height, pixels.collect())
                .expect("Could not convert back into image. Array might not be long enough"),
        )
    }))
}
