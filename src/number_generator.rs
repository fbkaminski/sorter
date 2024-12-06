use core::num;

use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::IteratorRandom;
use crate::number_file::NumberFile;

pub struct NumberGenerator {
    file: NumberFile
}

impl NumberGenerator {

    pub fn create(output_file: &str) -> NumberGenerator {
        NumberGenerator{
            file: NumberFile::create(output_file)
        }
    }

    pub fn generate(&mut self, start: u32, end: u32, holes: u32) {
        let numbers = self.generate_numbers(start, end, holes);
        self.file.write_numbers(numbers.as_slice(), numbers.len());
    }

    fn generate_numbers(&self, start: u32, end: u32, holes: u32) -> Vec<u32> {
        let mut rng = thread_rng();
        let mut numbers: Vec<u32> = (start..=end).collect();
        numbers.shuffle(&mut rng);
        let indices_to_remove: Vec<usize> = (0..numbers.len())
            .choose_multiple(&mut rng, holes as usize);
        // Sort indices in reverse to remove them efficiently
        let mut indices_to_remove = indices_to_remove;
        indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for index in indices_to_remove {
            numbers.remove(index);
        }
        return numbers
    }

}