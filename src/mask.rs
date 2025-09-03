
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

// Hue = 60 * ((G - B) / (max - min)) + 360 (if max is R),
// Hue = 60 * ((B - R) / (max - min)) + 120 (if max is G),
// Hue = 60 * ((R - G) / (max - min)) + 240 (if max is B).
fn rgb_to_hue(r: f32, g: f32, b: f32) -> f32 {
    let max = if r > g && r > b {
        r
    } else if g > b {
        g
    } else {
        b
    };
    let min = if r < g && r < b {
        r
    } else if g < b {
        g
    } else {
        b
    };

    let mut out = 0.0;
    if max == r {
        out = 60. * ((g - b) / (max - min)) + 360.;
    }
    if max == g {
        out = 60. * ((b - r) / (max - min)) + 120.;
    }
    if max == b {
        out = 60. * ((r - g) / (max - min)) + 240.;
    }

    if out <= 0. {
        return out + 360.;
    }

    out
}

#[wasm_bindgen(js_name = hueMask)]
pub fn hue_mask(lower_bounds: f32, higher_bounds: f32) -> usize {
    let mut binding = MASK_MANAGER.lock();
    let manager = binding.as_mut().expect("Could to get mask manager lock");

    let real_lower_bounds = lower_bounds * 360.0;
    let real_higher_bounds = higher_bounds * 360.0;

    manager.register(Box::from(move |attr: &mut StepAttributes| {
        let width = attr.image_buffer.width();
        let height = attr.image_buffer.height();

        let image = attr.image_buffer.to_rgb8();
        let hue = image
            .chunks(3)
            .map(|p| rgb_to_hue(p[0] as f32 / 255., p[1] as f32 / 255., p[2] as f32 / 255.));

        // assert!(!hue.any(|p| p < 0.), "One or more pixels are less then 0");
        // assert!(
        //     !hue.any(|p| p > 360.),
        //     "One or more pixels are more than 360"
        // );

        let pixels = hue.map(
            |p| match p >= real_lower_bounds && p <= real_higher_bounds {
                true => 255u8,
                false => 0u8,
            },
        );
        image::DynamicImage::ImageLuma8(
            GrayImage::from_vec(width, height, pixels.collect())
                .expect("Could not convert back into image"),
        )
    }))
}
