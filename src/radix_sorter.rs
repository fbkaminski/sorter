use crate::number_file::NumberFile;
use crate::number_sorter::SortAlgorithm;

pub struct RadixSorter {
    numbers: Vec<u32>,
    infile: NumberFile,
    outfile: NumberFile,
}

impl SortAlgorithm for RadixSorter {
    fn create(size: usize, input_file: &str, output_file: &str) -> Self {
        RadixSorter {
            numbers: Vec::with_capacity(size),
            infile: NumberFile::open(&input_file),
            outfile: NumberFile::create(&output_file),
        }
    }
    fn sort(&mut self) {
        self.load_input();
        self.radix_sort();
        self.write_output();
    }
}

impl RadixSorter {
    fn load_input(&mut self) {
        while self.infile.have_numbers() {
            let mut partial = self.infile.read_numbers().unwrap();
            self.numbers.append(&mut partial);
        }
    }

    fn write_output(&mut self) {
        self.outfile
            .write_numbers(&self.numbers.as_slice(), self.numbers.len());
    }

    fn radix_sort(&mut self) {
        let max = self.get_max();
        let mut exp = 1;
        while max / exp > 0 {
            self.count_sort(exp);
            exp *= 10;
        }
    }

    fn get_max(&self) -> u32 {
        let mut max = self.numbers[0];
        for i in 0..self.numbers.len() {
            if self.numbers[i] > max {
                max = self.numbers[i];
            }
        }
        return max;
    }

    fn count_sort(&mut self, exp: u32) {
        // Output array
        let n = self.numbers.len();
        let mut output: Vec<u32> = vec![0; n];
        let mut count: [u32; 10] = [0; 10];

        // Store count of occurrences
        // in count[]
        for i in 0..n {
            count[((self.numbers[i] / exp) % 10) as usize] += 1;
        }

        // Change count[i] so that count[i]
        // now contains actual position
        // of this digit in output[]
        for i in 1..10 {
            count[i] += count[i - 1];
        }

        // Build the output array
        for i in (0..n).rev() {
            output[(count[((self.numbers[i] / exp) % 10) as usize] - 1) as usize] = self.numbers[i];
            count[((self.numbers[i] / exp) % 10) as usize] -= 1;
        }

        // Copy the output array to arr[],
        // so that arr[] now contains sorted
        // numbers according to current digit
        for i in 0..n {
            self.numbers[i] = output[i];
        }
    }
}
