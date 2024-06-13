use std::sync::Mutex;

use image::DynamicImage;

pub struct StepAttributes {
    pub image_buffer: DynamicImage,
}

#[derive(Default)]
pub struct StepManager {
    // TODO: Make this a dictionary so that the indices dont "decay"
    steps: Vec<Box<dyn FnOnce(&mut StepAttributes) + Send + 'static>>,
}

impl StepManager {
    pub fn register(
        &mut self,
        step: Box<dyn FnOnce(&mut StepAttributes) + Send + 'static>,
    ) -> usize {
        self.steps.push(step);
        self.steps.len() - 1
    }

    pub fn run(&mut self, handle: usize, attr: &mut StepAttributes) {
        let func = self.steps.remove(handle);
        func.call_once((attr,));
    }
}

pub static STEP_MANAGER: Mutex<StepManager> = Mutex::new(StepManager { steps: Vec::new() });
