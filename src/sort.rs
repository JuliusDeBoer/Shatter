use image::{DynamicImage, RgbImage};
use wasm_bindgen::prelude::*;

use crate::manager::{StepAttributes, MASK_MANAGER, STEP_MANAGER};

/// You guessed it! It sorts pixels.
///
/// Its sort of the entire reason for the project.
#[wasm_bindgen(js_name = pixelSort)]
pub fn pixel_sort(mask: usize) -> usize {
    let mut binding = STEP_MANAGER.lock();
    let manager = binding
        .as_mut()
        .expect("Could not get lock for step manager");

    manager.register(Box::from(move |attr: &mut StepAttributes| {
        let mut binding = MASK_MANAGER.lock();
        let mask_manager = binding
            .as_mut()
            .expect("Could not get lock for mask manager");

        let width = attr.image_buffer.width() as usize;
        let height = attr.image_buffer.height() as usize;

        let buffer_binding = attr.image_buffer.clone().into_rgb8();
        let mut image_chunks = buffer_binding
            .chunks(3)
            .collect::<Vec<&[u8]>>()
            .chunks(width)
            .map(Vec::from)
            .collect::<Vec<_>>();

        let binding = mask_manager
            .run(&mask, attr)
            .expect("Could not create mask");

        let mask_chunks = binding
            .as_luma8()
            .expect("Could not handle mask")
            .chunks(width)
            .map(Vec::from)
            .collect::<Vec<_>>();

        assert_eq!(
            image_chunks.len(),
            mask_chunks.len(),
            "Image and mask are not the same size"
        );

        for i in 0..height {
            let mut begin = Option::<usize>::None;
            let mut current = 0usize;
            mask_chunks[i].clone().iter().for_each(|p| {
                if *p > 128 && current != width - 1 {
                    // Pixel light
                    if begin.is_none() {
                        begin = Some(current);
                    }
                } else {
                    // Pixel dark
                    if let Some(b) = begin {
                        let slice = image_chunks[i][b..current].as_mut();
                        slice.sort_by(|a, b| a[0].cmp(&b[0]));
                        begin = None;
                    }
                }
                current += 1;
            })
        }

        attr.image_buffer = DynamicImage::from(
            RgbImage::from_vec(
                width as u32,
                height as u32,
                image_chunks
                    .concat()
                    .iter()
                    .copied()
                    .flatten()
                    .copied()
                    .collect(),
            )
            .unwrap(),
        );
    }))
}
