use crate::number_file::NumberFile;
use crate::number_sorter::SortAlgorithm;

pub struct RadixSorter {
    size: usize,
    infile: NumberFile,
    outfile: NumberFile,
}

impl SortAlgorithm for RadixSorter {
    fn create(size: usize, input_file: &str, output_file: &str) -> Self {
        RadixSorter {
            size,
            infile: NumberFile::open(input_file),
            outfile: NumberFile::create(output_file),
        }
    }
    fn sort(&mut self) {
        self.load_and_sort();
    }
}

impl RadixSorter {

    fn load_and_sort(&mut self) {
        let mut numbers: Vec<u32> = Vec::with_capacity(self.size);
        while self.infile.have_numbers() {
            let mut partial = self.infile.read_numbers().unwrap();
            numbers.append(&mut partial);
        }
        self.radix_sort(&mut numbers);
        self.write_output(&numbers);
    }

    fn write_output(&mut self, arr: &Vec<u32>) {
        self.outfile
            .write_numbers(arr.as_slice(), arr.len());
    }

    fn radix_sort(&self, arr: &mut Vec<u32>) {
        let max = self.get_max(arr);
        let mut exp = 1;
        while max / exp > 0 {
            let sorted = self.sort_by(arr, exp);
            arr[..sorted.len()].copy_from_slice(&sorted[..sorted.len()]);
            exp *= 10;
        }
    }

    // fixme: we can use iter.map() for this
    fn get_max(&self, arr: &Vec<u32>) -> u32 {
        *arr.iter().max().unwrap()
    }

    fn sort_by(&self, arr: &mut Vec<u32>, exp: u32) -> Vec<u32> {
        // Output array
        let mut output: Vec<u32> = vec![0; arr.len()];
        let mut count: [u32; 10] = [0; 10];

        // Store count of occurrences
        // in count[]
        for i in 0..arr.len() {
            count[((arr[i] / exp) % 10) as usize] += 1;
        }

        // Change count[i] so that count[i]
        // now contains actual position
        // of this digit in output[]
        for i in 1..10 {
            count[i] += count[i - 1];
        }

        // Build the output array
        for i in (0..arr.len()).rev() {
            output[(count[((arr[i] / exp) % 10) as usize] - 1) as usize] = arr[i];
            count[((arr[i] / exp) % 10) as usize] -= 1;
        }

        return output;
    }
}
