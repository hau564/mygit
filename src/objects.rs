use std::io::BufReader;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;

// pub(crate) mod kind;
// use kind::Kind;

pub(crate) struct Object {
    pub kind: String,
    pub content: String,
    pub size: i32
}

impl Object {
    pub fn new(object_hash: &str) -> std::io::Result<Object> {
        let f = std::fs::File::open(format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..]))?;
        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);
        let mut bytes = Vec::new();
        z.read_to_end(&mut bytes)?;
        
        let mut kind_bytes = Vec::new();
        let mut size_bytes = Vec::new();
        let mut content_bytes = Vec::new();
        let mut started = false;
        let mut space = false;
        for i in &bytes {
            if started {
                if *i < 254 {content_bytes.push(*i);}
            }
            else {
                if *i == 0 {started = true;}
                else {
                    if (*i) == b' ' {space = true; continue;}
                    if space == false {
                        kind_bytes.push(*i);                        
                    }
                    else {
                        size_bytes.push(*i);
                    }
                }
            }
        }

        let kind = String::from_utf8(kind_bytes).expect("Convert bytes to string");
        let content = String::from_utf8(content_bytes).expect("Convert bytes to string");
        let size = String::from_utf8(size_bytes).expect("Convert bytes to string");
        let size = size.parse::<i32>().unwrap();
        Ok(Object{kind: kind, content: content, size: size})
    }
}