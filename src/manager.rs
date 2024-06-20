use std::{collections::HashMap, sync::Mutex};

use image::DynamicImage;

#[derive(Default)]
pub struct StepAttributes {
    pub image_buffer: DynamicImage,
}

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

    pub fn register(&mut self, step: Box<dyn FnOnce(&mut A) -> R + Send + 'static>) -> usize {
        let key = self.gen_unique_key();
        self.hash_map.insert(key, step);
        key
    }

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
