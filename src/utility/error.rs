use std::fmt;

#[derive(Debug)]
pub struct Upsie {
    details: String
}

impl Upsie {
    pub fn new(msg: &str) -> Upsie {
        Upsie{details: msg.to_string()}
    }
}

impl fmt::Display for Upsie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"A Upsie has occurred {}",self.details)
    }
}

impl From<std::io::Error> for Upsie {
    fn from(err: std::io::Error) -> Self {
        Upsie::new(&err.to_string())
    }
}

impl From<std::env::VarError> for Upsie {
    fn from(err: std::env::VarError) -> Self {
        Upsie::new(&err.to_string())
    }
}
