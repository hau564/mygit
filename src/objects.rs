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
    pub(crate) fn read(object_hash: &str) -> std::io::Result<Object<impl BufRead>> {
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

    pub(crate) fn blob_from_file(file: &str) -> std::io::Result<Object<impl BufRead>> {
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
where R: BufRead
{
    pub(crate) fn pretty(mut self) -> String {
        if self.kind == "tree" {
            return self.pretty_tree();
        }
        let mut bytes = Vec::new();
        self.content.read_to_end(&mut bytes).unwrap();
        let mut bytes_fix = Vec::new();
        for i in &bytes {
            let c = char::from(*i);
            if c.is_ascii() {
                bytes_fix.push(*i);
            }
        }
        let pretty = String::from_utf8(bytes_fix).expect("Convert bytes to string");
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
    
    pub(crate) fn pretty_tree(mut self) -> String {
        assert!(self.kind == "tree", "object is not a tree");

        let mut pretty = String::new();

        loop {
            let mut mode = Vec::new();
            let n = self.content.read_until(b' ', &mut mode).unwrap_or(0);
            if n == 0 {
                break;
            }
            if !pretty.is_empty() {
                pretty.push('\n');
            }
            mode.pop();
    
            let mut file_name = Vec::new();
            self.content.read_until(b'\0', &mut file_name).unwrap();
            file_name.pop();
    
            let mut hash = vec![0; 20];
            self.content.read_exact(&mut hash).unwrap();        
            
            pretty.push_str(get_entry(mode, file_name, hash).as_mut_str());
        }

        pretty
    }
}

fn get_entry(mode: Vec<u8>, file_name: Vec<u8>, hash_bytes: Vec<u8>) -> String {
    let hash = hex::encode(hash_bytes);
    let object = Object::read(&hash).expect("reading object from hash");

    let mut entry = String::new();
    if mode.len() < 6 {entry.push('0');}
    entry.push_str(String::from_utf8(mode).expect("coverting mode from bytes to string").as_mut_str());
    
    entry.push(' ');
    entry.push_str(&object.kind[..]);
    entry.push(' ');
    entry.push_str(&hash[..]);
    entry.push_str("    ");
    entry.push_str(String::from_utf8(file_name).expect("coverting file name from bytes to string").as_mut_str());

    entry
}
