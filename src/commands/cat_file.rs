use crate::objects::Object;

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> std::io::Result<()> {
    println!("cat-file: pretty_print = {pretty_print}, object = {object_hash}");
    let object = Object::new(object_hash)?;
    println!("{}",object.kind);
    println!("{}",object.size);
    println!("{}",object.content);
    Ok(())
}