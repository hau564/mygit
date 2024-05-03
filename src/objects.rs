use std::io::BufReader;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;

// pub(crate) mod kind;
// use kind::Kind;

pub(crate) struct Object {
    kind: String,
    content: String
}

impl Object {
    fn new(object_hash: &str) -> std::io::Result<Object> {
        let f = std::fs::File::open(format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..]))?;
        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);
        let mut bytes = Vec::new();
        z.read_to_end(&mut bytes)?;
        
        let mut kind_bytes = Vec::new();
        let mut content_bytes = Vec::new();
        let mut started = false;
        for i in &bytes {
            if started {
                if *i < 254 {content_bytes.push(*i);}
            }
            else {
                if *i == 0 {started = true;}
                else {kind_bytes.push(*i);}
            }
        }

        let kind = String::from_utf8(kind_bytes).expect("Convert bytes to string");
        let content = String::from_utf8(content_bytes).expect("Convert bytes to string");
        Ok(Object{kind: kind, content: content})
    }
}