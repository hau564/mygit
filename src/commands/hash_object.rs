use crate::objects::Object;

pub(crate) fn invoke(write: bool, file: &str) -> std::io::Result<()> {    
    let object = Object::blob_from_file(file)?;
    println!("{}", object.hash(write));    
    Ok(())
}