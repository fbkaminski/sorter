use crate::constants;
use crate::number_file::NumberFile;
use bitvec::prelude::*;
use crate::number_sorter::SortAlgorithm;

pub struct BitSorter {
    bits: BitVec,
    infile: NumberFile,
    outfile: NumberFile,
}

impl SortAlgorithm for BitSorter {
    fn create(size: usize, input_file: &str, output_file: &str) -> Self {
        BitSorter {
            bits: bitvec![0; size],
            infile: NumberFile::open(&input_file),
            outfile: NumberFile::create(&output_file)
        }
    }
    fn sort(&mut self) {
        self.load_input();
        self.write_output();
    }
}

impl BitSorter {

    fn load_input(&mut self) {
        while self.infile.have_numbers() {
            let numbers = self.infile.read_numbers().unwrap();
            for n in numbers.iter() {
                self.bits.set(*n as usize, true);
            }
        }
    }

    fn write_output(&mut self) {
        let mut numbers: [u32; constants::NBUFFER_SIZE] = [0; constants::NBUFFER_SIZE];
        let mut offset: usize = 0;
        for i in 0..self.bits.len() {
            if self.bits[i] {
                numbers[offset] = i as u32;
                offset += 1;
                if offset >= constants::NBUFFER_SIZE {
                    self.outfile.write_numbers(&numbers, offset);
                    numbers.fill(0);
                    offset = 0;
                }
            }
        }
        // we might have some rest left, so write the last batch
        if offset > 0 {
            self.outfile.write_numbers(&numbers, offset);
        }
    }

}
