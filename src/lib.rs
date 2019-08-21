//! Easy way to read UTF16 encoded files
//!
//! # Examples
//!
//! ```
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! let f = File::open("test_files/test_be.txt").unwrap();
//! let r = BufReader::new(f);
//! let s = utf16_reader::read_to_string(r);
//!
//! println!("{}", s);
//! ```

#![feature(test)]
extern crate test;

use std::io::Read;
use std::io::Result;
use std::cmp::min;

struct UTF16Reader<R> {
    source: R,
    buf: [u8; 256],
    pos: usize,
    len: usize,
}

impl<R: Read + Sized> UTF16Reader<R> {
    fn new(source: R) -> Self {
        UTF16Reader {
            source,
            buf: [0u8; 256],
            pos: 0,
            len: 0,
        }
    }
}

impl<R: Read + Sized> Read for UTF16Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut ipos = 0;
        loop {
            let m = min(self.len - self.pos, buf.len() - ipos);
            buf[ipos..ipos+m].copy_from_slice(&self.buf[self.pos..self.pos+m]);
            ipos += m;
            self.pos += m;
            if self.pos == self.len {
                self.pos = 0;
                self.len = 0;
                match self.source.read(&mut self.buf) {
                    Ok(0) => { return Ok(ipos); },
                    Ok(s) => { self.len = s; },
                    Err(e) => { return Err(e); },
                }
            } else  {
                break;
            }
        }
        Ok(ipos)
    }
}

/// Takes a Reader with UTF16 data and returns a String
pub fn read_to_string<R: Read>(source: R) -> String {
    let mut source = UTF16Reader::new(source);
    let mut buf = [0u8; 2];
    match source.read(&mut buf) {
        Ok(2) => (),
        Ok(s) => { panic!(s); },
        Err(e) => { panic!(e); },
    }

    let mut i = true; // find endian and use it for tracking
    if buf == [0xFE, 0xFF] {
        i = !i
    };

    let mut c = Vec::new();
    loop {
        let mut d = Vec::new();
        source.read_to_end(&mut d).unwrap();
        let mut b = 0u16;
        d.iter().for_each(|x| {
            if !i {
                b = (*x as u16) << 8;
            } else {
                c.push(b + (*x as u16));
            }
            i = !i;
        });

        match source.read(&mut buf) {
            Ok(0) => { break; },
            Ok(_) => (),
            Err(e) => { panic!(e); },
        }
    }
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
    fn read_be_test_file_large() {
        let f = File::open("test_files/test_large_utf16BE.txt").unwrap();
        let s = read_to_string(f);
        let r = fs::read_to_string("test_files/test_large_utf8.txt").unwrap();
        assert_eq!(r, s);
    }

    #[test]
    fn read_le_test_file() {
        let f = File::open("test_files/test_le.txt").unwrap();
        let s = read_to_string(f);
        assert_eq!("This is a test", s);
    }

    #[test]
    fn read_le_test_file_large() {
        let f = File::open("test_files/test_large_utf16LE.txt").unwrap();
        let s = read_to_string(f);
        let r = fs::read_to_string("test_files/test_large_utf8.txt").unwrap();
        assert_eq!(r, s);
    }

    #[bench]
    fn bench_small_text(b: &mut Bencher) {
        b.iter(|| read_to_string("This is a test".as_bytes()));
    }

    #[bench]
    fn bench_medium_text(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open("test_files/bench_text_medium.txt").unwrap();
            read_to_string(f);
            }
        );
    }

    #[bench]
    fn bench_large_text(b: &mut Bencher) {
        b.iter(|| {
            let f = File::open("test_files/bench_text_large.txt").unwrap();
            read_to_string(f);
            }
        );
    }
}
