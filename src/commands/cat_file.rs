use crate::objects::Object;

pub(crate) fn invoke(pretty_print: bool, object_hash: &str) -> std::io::Result<()> {
    assert!(pretty_print, "unsupported command, try -p");
    let object = Object::read(object_hash)?;
    println!("{}",object.pretty());
    Ok(())
}