mod utils;

use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

// Because a tuple doesn't work in wasm-bindgen.
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Resolution {
    pub x: u32,
    pub y: u32,
}

/// The base settings for one or multiple renders.
///
/// This struct is mostly used for caching. Allowing for faster renders and
/// *possibly* "realtime" editing.
#[wasm_bindgen]
pub struct RenderSetup {
    resolution: Resolution,
}

#[allow(unused)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(unused)]
pub struct SortablePixel {
    pixel: Pixel,
    sort_value: f32,
}

#[wasm_bindgen]
pub struct RustValueRef {}

trait Referencable {
    fn reference_name() -> String;
}

impl Ord for SortablePixel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_value.total_cmp(&other.sort_value)
    }
}

impl PartialOrd for SortablePixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SortablePixel {
    fn eq(&self, other: &Self) -> bool {
        self.sort_value == other.sort_value
    }
}

impl Eq for SortablePixel {}

#[allow(unused)]
pub struct Render {
    render_setup: RenderSetup,
    buffer: Vec<Pixel>,
}

// TODO(Julius): Make a macro for this
impl Referencable for Render {
    fn reference_name() -> String {
        "shatter::render".into()
    }
}

impl RenderSetup {
    pub fn with_resolution(mut self, resolution: Resolution) {
        self.resolution = resolution;
    }

    /// Loads in a new image as the beginning of a new render. This will be the
    /// starting point of every new render.
    ///
    /// [NOTE]: When a new image is loaded with this function the cache will be
    /// cleared to save up on memory.
    pub fn load_image() {
        todo!()
    }

    pub fn new_render() -> Render {
        todo!()
    }

    /// Explicitly clear image cache.
    pub fn clear_cache() {
        todo!()
    }
}

#[wasm_bindgen]
impl Render {
    pub fn step() {
        todo!()
    }

    pub fn to_canvas() {
        todo!()
    }
}
