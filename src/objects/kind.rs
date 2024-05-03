use std::fmt;

pub(crate) enum Kind {
    Blob,
    Tree,
    Commit
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Blob => write!(f, "Blob"),
            Kind::Tree => write!(f, "Tree"),
            Kind::Commit => write!(f, "Commit"),
        }
    }
}