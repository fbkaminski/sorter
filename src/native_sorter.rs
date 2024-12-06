use crate::number_file::NumberFile;
use crate::number_sorter::SortAlgorithm;

pub struct NativeSorter {
    numbers: Vec<u32>,
    infile: NumberFile,
    outfile: NumberFile,
}

impl SortAlgorithm for NativeSorter {
    fn create(size: usize, input_file: &str, output_file: &str) -> Self {
        NativeSorter {
            numbers: Vec::with_capacity(size),
            infile: NumberFile::open(&input_file),
            outfile: NumberFile::create(&output_file)
        }
    }
    fn sort(&mut self) {
        self.load_input();
        self.numbers.sort();
        self.write_output();
    }
}

impl NativeSorter {

    fn load_input(&mut self) {
        while self.infile.have_numbers() {
            let mut partial = self.infile.read_numbers().unwrap();
            self.numbers.append(&mut partial);
        }
    }

    fn write_output(&mut self) {
        self.outfile.write_numbers(&self.numbers.as_slice(), self.numbers.len());
    }

}
