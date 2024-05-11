use crate::objects::Object;

pub(crate) fn invoke(tree_hash: &str) -> std::io::Result<()> {
    let object = Object::read(tree_hash)?;
    println!("{}", object.pretty_tree());
    Ok(())
}