use std::io::BufReader;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::fs;
use std::fs::File;
use sha1::{Sha1, Digest};

pub(crate) struct Object<T> {
    pub size: i64,
    pub kind: String,
    pub content: T
}

impl Object<()> {
    pub(crate) fn read(object_hash: &str) -> std::io::Result<Object<impl Read>> {
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

        Ok(Object{kind: kind, size: size, content: z})
    }

    pub(crate) fn blob_from_file(file: &str) -> std::io::Result<Object<impl Read>> {
        let f = File::open(file)?;
        let reader = BufReader::new(f);
        let size = fs::metadata(file)?.len();
        Ok(Object{
            kind: "blob".to_string(),
            size: size as i64,
            content: reader
        })
    }

}

impl<R> Object<R> 
where R: Read
{
    pub(crate) fn pretty(mut self) -> String {
        let mut bytes = Vec::new();
        self.content.read_to_end(&mut bytes).unwrap();
        let mut bytes_fix = Vec::new();
        for i in &bytes {
            let c = char::from(*i);
            if c.is_ascii() {
                bytes_fix.push(*i);
            }
        }
        let pretty = String::from_utf8(bytes_fix).expect("Convert content bytes to string");
        pretty
    }

    pub(crate) fn hash(mut self, write: bool) -> String {        
        let mut bytes = Vec::new();
        self.content.read_to_end(&mut bytes).unwrap();

        let mut content = Vec::from(self.kind[..].as_bytes());
        content.push(b' ');
        content.append(&mut self.size.to_string().into_bytes());
        content.push(0);
        content.append(&mut bytes);

        let print = content.to_vec();
        
        let mut hasher = Sha1::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());

        if write {
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write(&print).unwrap();
            let compressed = e.finish().unwrap();
            let mut file = File::create(format!(".git/objects/{}/{}", &hash[..2], &hash[2..])).unwrap();
            file.write_all(&compressed).unwrap();
        }

        hash
    }
}