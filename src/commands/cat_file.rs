use std::io::BufReader;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;

use crate::objects::Object;

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> std::io::Result<()> {
    println!("cat-file: pretty_print = {pretty_print}, object = {object_hash}");
    let f = std::fs::File::open(format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..]))?;
    let z = ZlibDecoder::new(f);
    let mut z = BufReader::new(z);
    let mut bytes = Vec::new();
    z.read_to_end(&mut bytes)?;
    
    let mut content = Vec::new();
    let mut started = false;
    for i in &bytes {
        if started {
            if *i < 254 {
                content.push(*i);
            }
        }
        if *i == 0 {
            started = true;
        }
    }

    let content = String::from_utf8(content).expect("Convert bytes to string");
    println!("{}", content);
    Ok(())
}