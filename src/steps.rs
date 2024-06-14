use std::{cell::LazyCell, collections::HashMap, sync::Mutex};

use image::DynamicImage;

pub struct StepAttributes {
    pub image_buffer: DynamicImage,
}

pub type StepFunc = dyn FnOnce(&mut StepAttributes) + Send + 'static;

#[derive(Default)]
pub struct StepManager {
    steps: HashMap<usize, Box<StepFunc>>,
}

impl StepManager {
    fn gen_unique_key(&self) -> usize {
        let mut keys = self.steps.keys();
        let mut key = keys.len();
        while keys.any(|k| *k == key) {
            key += 1;
        }
        key
    }

    pub fn register(&mut self, step: Box<StepFunc>) -> usize {
        let key = self.gen_unique_key();
        self.steps.insert(key, step);
        key
    }

    pub fn run(&mut self, handle: &usize, attr: &mut StepAttributes) {
        let func = self.steps.remove(handle).expect("Could not get closure");
        func.call_once((attr,));
    }
}

lazy_static! {
    pub static ref STEP_MANAGER: Mutex<StepManager> = Mutex::new(StepManager::default());
}
