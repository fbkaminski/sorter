use core::str;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use memmap::Mmap;
use crate::util::atoi;
use crate::constants;

pub struct NumberFile {
    file: File,
    len: u64,
    mem: Option<Mmap>,
    rest: [u8; 255],
    rest_len: usize,
    readed: usize
}

impl NumberFile {

    pub fn create<'a>(path: &'a str) -> NumberFile {
        let file = File::create(path).expect("creation failed");
        let len = file.metadata().unwrap().len();
        NumberFile {
            file: file,
            len: len,
            mem: None,
            rest: [0; 255],
            rest_len: 0,
            readed: 0
        }
    }

    pub fn open<'a>(path: &'a str) -> NumberFile {
        let file = File::open(path).expect("open failed");
        let len = file.metadata().unwrap().len();
        let mem = unsafe { Mmap::map(&file).unwrap() };
        NumberFile {
            file: file,
            len: len,
            mem: Some(mem),
            rest: [0; 255],
            rest_len: 0,
            readed: 0
        }
    }

    pub fn have_numbers(&self) -> bool {
        self.readed < self.len as usize
    }

    pub fn read_numbers(&mut self) -> Option<Vec<u32>> {
        let start = self.readed;
        let rest = self.len as usize - self.readed;
        let end = min(self.readed + constants::BUFFER_SIZE, self.readed + rest);
        if start < end {
            let mut numbers = self.read_numbers_from_buffer(start, end);
            self.readed += end - start;
            // if this is the last batch, check if we still have something in the rest buffer
            if rest < constants::BUFFER_SIZE && self.rest_len > 0 {
                let n = self.as_number(&self.rest[0..self.rest_len]);
                numbers.push(n as u32);
            }
            return Some(numbers);
        }
        return None;
    }

    fn read_numbers_from_buffer(&mut self, start: usize, end: usize) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        let mut line: [u8; 255] = [0; 255];
        let mut offset = 0;
        let readed = end - start;

        // if we have rest, copy to line and advance offset accordingly
        for i in 0..self.rest.len() {
            match self.rest[i] {
                b'\0' => {
                    line[offset] = b'\0';
                    self.rest.fill(0);
                    self.rest_len = 0;
                    break;
                }
                _ => {
                    line[offset] = self.rest[i];
                    offset += 1;
                }
            }
        }

        let buf = &self.mem.as_ref().unwrap()[start..end];

        for i in 0..buf.len() {
            let is_eof = readed < constants::BUFFER_SIZE && i == readed;
            if buf[i] == b'\n' || is_eof {
                let n = self.as_number(&line[0..offset]);
                result.push(n as u32);
                offset = 0;
                line.fill(0);
                if is_eof {
                    break;
                }
            } else {
                line[offset] = buf[i];
                offset += 1;
            }
        }
        if offset > 0 {
            // copy the rest which is on line to rest
            self.rest.clone_from_slice(&line);
            self.rest_len = offset;
            self.rest[offset] = b'\0';
        }
        return result;
    }

    pub fn write_numbers(&mut self, numbers: &[u32], len: usize) {
        let mut content = String::new();
        for i in 0..len {
            content.push_str(numbers[i].to_string().as_str());
            content.push('\n');
        }
        let _ = self.file.write_all(content.as_bytes());
        // fancy but inneficient

        // let content: String = numbers
        // .iter()
        // .map(|&num| (num as u32).to_string())
        // .collect::<Vec<String>>()
        // .join("\n");
    }

    #[inline]
    fn as_number(&self, buf: &[u8]) -> i32 {
        return atoi(unsafe {str::from_utf8_unchecked(buf)}.as_bytes());
        //return chars.parse::<u32>().unwrap();
    }

}
