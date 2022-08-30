use std::ops::AddAssign;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SaveResult {
    pub save: u32,
    pub value: f32,
}
impl SaveResult {
    #[allow(unused)]
    pub fn new(save: u32, value: f32) -> Self {
        Self { save, value }
    }

    pub fn initial(save: u32) -> Self {
        Self { save, value: 0.0 }
    }
}

#[derive(Debug, Serialize)]
pub struct ProcessorResults {
    pub save_results: [SaveResult; 7],
}
impl ProcessorResults {
    pub fn new() -> Self {
        Self {
            save_results: [
                SaveResult::initial(1),
                SaveResult::initial(2),
                SaveResult::initial(3),
                SaveResult::initial(4),
                SaveResult::initial(5),
                SaveResult::initial(6),
                SaveResult::initial(7),
            ],
        }
    }

    pub fn add(&mut self, save: u32, addition: f32) {
        for result in self.save_results.iter_mut() {
            if result.save == save {
                result.value += addition;
            }
        }
    }

    pub fn add_all(&mut self, addition: f32) {
        for mut save_result in self.save_results.iter_mut() {
            save_result.value += addition;
        }
    }

    pub fn merge(&mut self, other: ProcessorResults) {
        for (index, result) in self.save_results.iter_mut().enumerate() {
            result.value += other.save_results[index].value;
        }
    }
}

impl Default for ProcessorResults {
    fn default() -> Self {
        Self::new()
    }
}

impl From<[SaveResult; 7]> for ProcessorResults {
    fn from(save_results: [SaveResult; 7]) -> Self {
        Self { save_results }
    }
}

impl AddAssign<ProcessorResults> for ProcessorResults {
    fn add_assign(&mut self, rhs: ProcessorResults) {
        self.merge(rhs);
    }
}
