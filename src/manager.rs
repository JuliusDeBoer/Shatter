use std::{collections::HashMap, sync::Mutex};

use image::DynamicImage;

/// Everything a step needs to know about the current state of the image
#[derive(Default)]
pub struct StepAttributes {
    pub image_buffer: DynamicImage,
}

/// Manages functions that need to be kept underwater.
///
/// A couple of structs that this library uses are not able to be translated
/// into JS. The manager allows you to register and call functions using a
/// handle. This makes sure that JS never knows about these functions and
/// therefore they dont have to be translated.
#[derive(Default)]
pub struct Manager<A, R> {
    #[allow(clippy::type_complexity)]
    hash_map: HashMap<usize, Box<dyn FnOnce(&mut A) -> R + Send + 'static>>,
}

impl<A, R> Manager<A, R> {
    fn gen_unique_key(&self) -> usize {
        let mut keys = self.hash_map.keys();
        let mut key = keys.len();
        while keys.any(|k| *k == key) {
            key += 1;
        }
        key
    }

    /// Register a new function into the manager.
    /// 
    /// Returns a handle that can then be fed into the [run][Self::run]
    /// function.
    pub fn register(&mut self, step: Box<dyn FnOnce(&mut A) -> R + Send + 'static>) -> usize {
        let key = self.gen_unique_key();
        self.hash_map.insert(key, step);
        key
    }

    /// Runs the registered function.
    pub fn run(&mut self, handle: &usize, attr: &mut A) -> R {
        let func = self.hash_map.remove(handle).expect("Could not get closure");
        func.call_once((attr,))
    }
}

lazy_static! {
    pub static ref STEP_MANAGER: Mutex<Manager<StepAttributes, ()>> =
        Mutex::new(Manager::default());
    pub static ref MASK_MANAGER: Mutex<Manager<StepAttributes, DynamicImage>> =
        Mutex::new(Manager::default());
}
