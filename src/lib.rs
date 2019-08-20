//! Easy way to read UTF16 encoded files

#![feature(test)]
extern crate test;

use std::io::Read;

/// Decodes a Reader with UTF16 data to a String
/// 
/// # Examples
/// 
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// 
/// let f = File::open("test_files/test_be.txt").unwrap();
/// let r = BufReader::new(f);
/// let s = utf16_reader::read_to_string(r);
/// 
/// println!("{}", s);
/// ```
pub fn read_to_string<R: Read>(source: R) -> String {
    let mut bytes = source.bytes();
    
    let x: Vec<u8> = bytes.by_ref().take(2).map(|x|x.unwrap()).collect();
    
    let mut i = true;
    if ((x[0] as u16) << 8) + x[1] as u16 == 0xFEFF { i = !i };
    
    let (hs, ls): (Vec<u8>, Vec<u8>) = bytes.map(|x| x.unwrap()).partition(|_| {i=!i; i});
    let c: Vec<u16> = ls.iter().zip(hs.iter()).map(|(ls, hs)| ((*hs as u16)<<8) + *ls as u16).collect();

    String::from_utf16(&c).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::fs;
    use std::fs::File;

    #[test]
    fn read_be_test_file() {
        let f = File::open("test_files/test_be.txt").unwrap();
        let s = read_to_string(f);
        assert_eq!("This is a test", s);
    }

    #[test]
    fn read_le_test_file() {
        let f = File::open("test_files/test_le.txt").unwrap();
        let s = read_to_string(f);
        assert_eq!("This is a test", s);
    }

    #[bench]
    fn bench_small_text(b: &mut Bencher) {
        b.iter(|| read_to_string("This is a test".as_bytes()));
    }

    #[bench]
    fn bench_medium_text(b: &mut Bencher) {
        let f = fs::read("test_files/bench_text_medium.txt").unwrap();
        b.iter(|| read_to_string(f.as_slice()));
    }

    #[bench]
    fn bench_large_text(b: &mut Bencher) {
        let f = fs::read("test_files/bench_text_large.txt").unwrap();
        b.iter(|| read_to_string(f.as_slice()));
    }
}