use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

const RTK_MAGIC: &[u8; 26] = b"(c) 1998 PyroTechnix,Inc.";

#[derive(Debug)]
pub enum DefParserError {
    InvalidHeader(&'static str),
    FormatNotSupported,
}

pub fn decompress(input: &str, output: &str) -> io::Result<()> {
    let f = File::open(&input).unwrap();
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(f);

    reader.read_to_end(&mut buffer).unwrap();

    let s = decompress_data(buffer.as_slice());

    let mut f = File::create(&output)?;
    f.write_all(s.unwrap().as_bytes())?;

    Ok(())
}

pub fn decompress_data(src_data: &[u8]) -> io::Result<String> {
    let def_data = parse_def_header(src_data);

    assert_eq!(def_data.is_ok(), true);

    let mut gz = GzDecoder::new(def_data.unwrap());
    let mut s = String::new();
    gz.read_to_string(&mut s)?;

    Ok(s)
}

pub fn parse_def_header(src_data: &[u8]) -> Result<&[u8], DefParserError> {
    let len_magic = RTK_MAGIC.len();

    if src_data[..len_magic].starts_with(RTK_MAGIC) {
        return Ok(&src_data[len_magic..]);
    }

    Err(DefParserError::InvalidHeader(
        "Failed to validate file signature",
    ))
}
