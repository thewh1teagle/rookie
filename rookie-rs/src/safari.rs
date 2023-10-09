use crate::enums::*;
use crate::{utils, date};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use std::io::Error;
use std::path::PathBuf;

use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::vec::Vec;
use std::time::Duration;

fn parse_page(bs: &[u8]) -> Result<Vec<Cookie>, Box<dyn std::error::Error>> {
    if slice(bs, 0, 4)? != [0x00, 0x00, 0x01, 0x00] {
        return Err("bad page header".into());
    }

    let count = slice(bs, 4, 4).map(LittleEndian::read_u32)? as usize;
    let parsed_table = parse_table::<LittleEndian>(&bs[8..], count)?;
    let mut cookies: Vec<Cookie> = vec![];
    for off in parsed_table {
        let slice_result = slice(bs, off, 4);

        // Read a little-endian unsigned 32-bit integer from the slice
        let u32_value = slice_result.map(LittleEndian::read_u32);

        // If 'u32_value' is Some(len), create a new slice of length 'len'
        let parsed_slice = u32_value.and_then(|len| slice(bs, off, len as usize));

        // Parse the sliced data into a Cookie struct using LittleEndian encoding
        let cookie = parsed_slice
            .and_then(|u| parse_cookie::<LittleEndian>(u))?;
        cookies.push(cookie);

        // Return the parsed Cookie struct, or propagate an error if any step fails
    }

    if slice(bs, count * 4 + 8, 4)? != [0x00, 0x00, 0x00, 0x00] {
        return Err("bad page trailer".into());
    }
    Ok(cookies)
}

fn parse_cookie<T: ByteOrder>(bs: &[u8]) -> io::Result<Cookie> {
    if bs.len() < 0x30 {
        return Err(Error::new(ErrorKind::InvalidData, "cookie data underflow"));
    }
    let flags = T::read_u32(&bs[0x08..0x0C]);

    let url_off = T::read_u32(&bs[0x10..0x14]) as usize;
    let name_off = T::read_u32(&bs[0x14..0x18]) as usize;
    let path_off = T::read_u32(&bs[0x18..0x1C]) as usize;
    let value_off = T::read_u32(&bs[0x1C..0x20]) as usize;

    // i/OS/X to Unix timestamp +(1 Jan 2001 epoch seconds).
    let expires = T::read_u64(&bs[0x28..0x30]);
    let expires = date::safari_timestamp(expires);

    let url = slice_to(bs, url_off, name_off).and_then(&c_str)?;
    let name = slice_to(bs, name_off, path_off).and_then(&c_str)?;
    let path = slice_to(bs, path_off, value_off).and_then(&c_str)?;
    let value = slice_to(bs, value_off, bs.len()).and_then(&c_str)?;

    let is_secure = flags & 0x01 == 0x01;
    let is_http_only = flags & 0x04 == 0x04;

    let cookie = Cookie {
        expires,
        domain: url,
        http_only: is_http_only,
        name,
        path,
        value,
        same_site: 0,
        secure: is_secure,
    };
    Ok(cookie)
}

pub fn parse_content(bs: &[u8]) -> Result<Vec<Cookie>, Box<dyn std::error::Error>> {
    // Magic bytes: "COOK" = 0x636F6F6B
    if slice(bs, 0, 4)? != [0x63, 0x6F, 0x6F, 0x6B] {
        return Err("not a cookie file".into());
    }

    let count = slice(bs, 4, 4).map(BigEndian::read_u32)? as usize;
    let table_iter = parse_table::<BigEndian>(&bs[8..], count)?;
    let table_iter = table_iter.iter();
    let mut pages = Vec::new();
    let mut off = count * 4 + 8;
    
    for &len in table_iter {
        let page_slice = match slice(bs, off, len) {
            Ok(slice) => slice,
            Err(_) => {
                // Handle the error here, e.g., by returning the error.
                return Err("cant get slice from page".into());
            }
        };
    
        pages.push(page_slice.to_vec());
        off += len;
    }

    let mut cookies: Vec<Cookie> = vec![];
    for page in pages {
        let cookie = parse_page(page.as_slice())?;
        cookies.extend(cookie);
    }
    Ok(cookies)
}

fn slice(bs: &[u8], off: usize, len: usize) -> io::Result<&[u8]> {
    if off + len > bs.len() {
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("data underflow: {}", off + len - bs.len()),
        ))
    } else {
        Ok(&bs[off..off + len])
    }
}

fn parse_table<T: ByteOrder>(bs: &[u8], count: usize) -> io::Result<Vec<usize>> {
    let end = count * 4;
    if end > bs.len() {
        return Err(Error::new(ErrorKind::InvalidData, "table data underflow"));
    }
    let data = (&bs[..end])
        .chunks(4)
        .map(|u| T::read_u32(u) as usize)
        .collect();
    Ok(data)
}

fn slice_to(bs: &[u8], off: usize, to: usize) -> io::Result<&[u8]> {
    if to < off {
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("negative data length: {}", to - off),
        ))
    } else {
        slice(bs, off, to - off)
    }
}

fn c_str(bs: &[u8]) -> io::Result<String> {
    bs.split_last()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "null c string"))
        .and_then(|(&last, elements)| {
            if last == 0x00 {
                Ok(elements)
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "c string non null terminator",
                ))
            }
        })
        .and_then(|elements| {
            String::from_utf8(elements.to_vec())
                .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))
        })
}

pub fn safari_based(
    db_path: PathBuf,
    domains: Option<Vec<&str>>,
) -> Result<Vec<Cookie>, Box<dyn std::error::Error>> {
    // 1. open cookies file
    // 2. parse headers
    // 3. parse pages (total from headers)
    // 4. get N cookies from each page, iterate
    // 5. parse each cookie
    // 6. add each cookie based on domain filter
    let mut file = File::open(db_path)?;
    let mut bs: Vec<u8> = Vec::new();
    file.read_to_end(&mut bs)?;
    let cookies = parse_content(&bs)?;



    // Filter cookies by domain if domains are specified
    if let Some(domain_filters) = domains {
        let filtered_cookies: Vec<Cookie> = cookies
            .into_iter()
            .filter(|cookie| {
                // Check if the cookie's domain matches any of the specified domains
                domain_filters.iter().any(|&domain| {
                    // Implement your domain matching logic here
                    // For example, you can use the `.ends_with` method to check if the cookie's domain ends with the specified domain.
                    cookie.domain.ends_with(domain)
                })
            })
            .collect();

        Ok(filtered_cookies)
    } else {
        Ok(cookies)
    }
}
