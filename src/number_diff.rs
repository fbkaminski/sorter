use std::collections::hash_set::SymmetricDifference;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub struct NumberDiff;

impl NumberDiff {

    pub fn create() -> NumberDiff {
        NumberDiff {}
    }

    pub fn compare<'a, 'b>(&mut self, input_file: &'a str, output_file: &'b str) -> Option<Vec<u32>> {
        let mut icontents = String::new();
        let mut ifile = File::open(input_file).expect("open failed");
        let _ = ifile.read_to_string(&mut icontents).expect("fail reading");
        let inumbers: Vec<u32> = icontents
            .lines()
            .map(|line| line.trim().parse::<u32>())
            .filter_map(Result::ok)
            .collect();

        let mut ocontents = String::new();
        let mut ofile = File::open(output_file).expect("open failed");
        let _ = ofile.read_to_string(&mut ocontents).expect("fail reading");
        let onumbers: Vec<u32> = ocontents
            .lines()
            .map(|line| line.trim().parse::<u32>())
            .filter_map(Result::ok)
            .collect();

        let iset: HashSet<_> = inumbers.iter().cloned().collect();
        let oset: HashSet<_> = onumbers.iter().cloned().collect();

        let sdiff: SymmetricDifference<'_, u32, std::hash::RandomState> = iset.symmetric_difference(&oset);
        let vec: Vec<u32> = sdiff.cloned().collect();
        return Some(vec.iter().map(|&x| x).collect());
    }

}