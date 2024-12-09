pub trait SortAlgorithm {
    fn create(size: usize, input_file: &str, output_file: &str) -> Self;
    fn sort(&mut self);
}

pub struct NumberSorter<T: SortAlgorithm> {
    algorithm: Box<T>,
}

impl<T: SortAlgorithm> NumberSorter<T> {
    pub fn create(size: usize, input_file: &str, output_file: &str) -> NumberSorter<T> {
        NumberSorter {
            algorithm: Box::new(T::create(size, input_file, output_file)),
        }
    }

    pub fn sort(&mut self) {
        self.algorithm.sort();
    }
}
