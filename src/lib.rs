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

    let mut i = true; // find endian and use it for tracking
    if let [Some(Ok(0xFE)), Some(Ok(0xFF))] = [bytes.next(), bytes.next()] {
        i = !i
    };

    let mut c = Vec::new();
    let mut b = 0u16;
    bytes.map(|x| x.unwrap() as u16).for_each(|x| {
        if !i {
            b = x << 8;
        } else {
            c.push(b + x);
        }
        i = !i;
    });

    String::from_utf16(&c).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use test::Bencher;

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
