pub enum Mode {
    Delete,
    Edit,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Delete => "d",
                Mode::Edit => "e",
            }
        )
    }
}
