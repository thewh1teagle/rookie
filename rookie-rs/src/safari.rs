use std::collections::btree_map::Cursor;
use std::{path::PathBuf, error::Error};
use crate::enums::*;

static APPLE_TO_UNIX_TIME: i32 = 978307200;
use std::fs::File;
use std::io::{self, Read, Seek, BufReader};
use std::path::Path;
use std::vec::Vec;

enum Endianness {
    LittleEndian,
    BigEndian,
}

trait CustomRead {
    fn read_u32(&mut self, endianness: Endianness) -> io::Result<u32>;
    fn read_f64(&mut self, endianness: Endianness) -> io::Result<f64>;
    fn read_bytes(&mut self, length: usize) -> io::Result<Vec<u8>>;
}

impl<R: Read> CustomRead for BufReader<R> {
    fn read_u32(&mut self, endianness: Endianness) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        
        let value = match endianness {
            Endianness::LittleEndian => u32::from_le_bytes(buffer),
            Endianness::BigEndian => u32::from_be_bytes(buffer),
        };
        
        Ok(value)
    }

    fn read_f64(&mut self, endianness: Endianness) -> io::Result<f64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        
        let value = match endianness {
            Endianness::LittleEndian => f64::from_le_bytes(buffer),
            Endianness::BigEndian => f64::from_be_bytes(buffer),
        };
        
        Ok(value)
    }

    fn read_bytes(&mut self, length: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; length];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    
}



pub fn safari_based(db_path: PathBuf, domains: Option<Vec<&str>>) -> Result<Vec<Cookie>, Box<dyn Error>> {
    // 1. open cookies file
    // 2. parse headers
    // 3. parse pages (total from headers)
    // 4. get N cookies from each page, iterate
    // 5. parse each cookie
    // 6. add each cookie based on domain filter



    // init
    let offset = 0;
    // open file
    let file = File::open(db_path)?;
    let mut buf = BufReader::new(file);
    


    // parse header
    assert_eq!(buf.read_bytes(4)?, b"cook");
    let mut total_pages_buf = [0u8, 4];
    buf.read_exact(&mut total_pages_buf)?;
    let total_pages = total_pages_buf.first().unwrap_or(&0);
    let mut page_sizes = vec![];
    

    
    for _ in 0..*total_pages {
        let mut size_buf = [0u8; 4];
        buf.read_exact(&mut size_buf)?;
        let first = size_buf[0]; // Clone the first element
        page_sizes.push(first);
    }

    // load
    
    Err(("can't find any cookies file").into())
}



