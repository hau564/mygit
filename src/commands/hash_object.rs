// use crate::objects::Object;
use std::fs::File;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::{Read, BufReader, Write, BufWriter};
use sha1::{Sha1, Digest};

pub(crate) fn invoke(write: bool, file: &str) -> std::io::Result<()> {    
    let f = File::open(file)?;
    let mut reader = BufReader::new(f);
    let mut file_content = Vec::new();
    reader.read_to_end(&mut file_content)?;

    let mut content = Vec::from(b"blob ");
    content.append(&mut file_content.len().to_string().into_bytes());
    content.push(0);
    content.append(&mut file_content);

    let print = content.to_vec();

    let mut hasher = Sha1::new();
    hasher.update(content);
    let hash = format!("{:x}", hasher.finalize());

    if write {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write(&print)?;
        let compressed = e.finish()?;
        let mut file = File::create(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))?;
        file.write_all(&compressed)?;
    }

    println!("{hash}");
    
    Ok(())
}