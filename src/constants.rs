
// The size of the buffer for read and write
pub const BUFFER_SIZE: usize = 16384;
// the buffer size for the number array
// this is an estimation of how many numbers
// we get within a 16k buffer range
// (we are expecting at most a 6 digit number + newline)
pub const NBUFFER_SIZE: usize = BUFFER_SIZE / 7;