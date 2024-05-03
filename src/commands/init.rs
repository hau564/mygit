use std::fs;

pub(crate) fn invoke() {
    let mut existed = false;
    fs::create_dir(".git").unwrap_or_else(|_| -> (){existed = true; ()});
    fs::create_dir(".git/objects").unwrap_or_else(|_| -> (){existed = true; ()});
    fs::create_dir(".git/refs").unwrap_or_else(|_| -> (){existed = true; ()});
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap_or_else(|_| -> (){existed = true; ()});
    if existed {
        println!("Reinitialized existing git repository: existed items kept");
    }
    else {
        println!("Initialized git directory")
    }
}