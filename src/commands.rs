use crate::number_generator::NumberGenerator;
use crate::number_sorter::NumberSorter;
use crate::bit_sorter::BitSorter;
use crate::native_sorter::NativeSorter;
use crate::number_diff::NumberDiff;

pub enum SortStrategy {
    BITSORT,
    NATIVE
}

pub trait Command {
    fn run(&mut self);
}

pub struct GenerateCommand {
    generator: NumberGenerator,
    start: u32,
    end: u32,
}

impl GenerateCommand {

    pub fn create(output_file: &str, start: u32, end: u32) -> GenerateCommand {
        GenerateCommand {
            generator: NumberGenerator::create(&output_file),
            start: start,
            end: end
        }
    }

}

impl Command for GenerateCommand {

    fn run(&mut self) {
        // take 1/4 of numbers to make them more sparse (and less sequential)
        let holes = self.end - self.start / 4;
        self.generator.generate(self.start, self.end, holes);
    }
}

pub struct BitSortCommand {
    sorter: NumberSorter<BitSorter>
}

impl BitSortCommand {
    pub fn new(input_file: &str, output_file: &str) -> BitSortCommand {
        BitSortCommand {
            sorter: NumberSorter::create(
                    1_000_000,
                    &input_file,
                    &output_file)
        }
    }
}

impl Command for BitSortCommand {

    fn run(&mut self) {
        self.sorter.sort();
    }

}

pub struct NativeSortCommand {
    sorter: NumberSorter<NativeSorter>
}

impl NativeSortCommand {
    pub fn new(input_file: &str, output_file: &str) -> NativeSortCommand {
        NativeSortCommand {
            sorter: NumberSorter::create(
                    1_000_000,
                    &input_file,
                    &output_file)
        }
    }
}

impl Command for NativeSortCommand {

    fn run(&mut self) {
        self.sorter.sort();
    }

}

pub struct CompareCommand {
    comparator: NumberDiff,
    input_file: String,
    output_file: String
}

impl CompareCommand {

    pub fn new(input_file: &str, output_file: &str) -> CompareCommand {
        CompareCommand {
            comparator: NumberDiff::create(),
            input_file: input_file.to_string(),
            output_file: output_file.to_string()
        }
    }
}

impl Command for CompareCommand {

    fn run(&mut self) {
        let diff = self.comparator.compare(self.input_file.as_str(), self.output_file.as_str()).unwrap();
        if diff.len() > 0 {
            println!("elements missing: {:?}", diff);
        } else {
            println!("no elements missing");
        }
    }

}

pub struct Commands;

impl Commands {

    pub fn sort(strategy: SortStrategy, input_file: &str, output_file: &str) -> Box<dyn Command> {
        return match strategy {
            SortStrategy::BITSORT => { Box::new(BitSortCommand::new(input_file, output_file))}
            SortStrategy::NATIVE => { Box::new(NativeSortCommand::new(input_file, output_file)) }
        }
    }

    pub fn compare(input_file: &str, output_file: &str) -> Box<dyn Command> {
        Box::new(CompareCommand::new(input_file, output_file))
    }

    pub fn generate(output_file: &str, start: u32, end: u32) -> Box<dyn Command> {
        Box::new(GenerateCommand::create(output_file, start, end))
    }

}
