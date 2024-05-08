use std::io::BufReader;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;

pub(crate) struct Object<T> {
    pub size: i64,
    pub kind: String,
    pub content: T
}

impl Object<()> {
    pub(crate) fn read(object_hash: &str) -> std::io::Result<Object<String>> {
        let f = std::fs::File::open(format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..]))?;
        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);
        
        let mut kind = Vec::new();
        z.read_until(b' ', &mut kind)?;
        kind.pop();
        let kind = String::from_utf8(kind).expect("Convert bytes to string");

        let mut size = Vec::new();
        z.read_until(0, &mut size)?;
        size.pop();
        let size = String::from_utf8(size).expect("Convert bytes to string");
        let size = size.parse::<i64>().unwrap();

        let mut content = Vec::new();
        z.read_to_end(&mut content)?;
        let mut content_fix = Vec::new();
        for i in &content {
            let c = char::from(*i);
            if c.is_ascii() {
                content_fix.push(*i);
            }
        }
        let content = String::from_utf8(content_fix).expect("Convert content bytes to string");

        Ok(Object{kind: kind, size: size, content: content})
    }
}