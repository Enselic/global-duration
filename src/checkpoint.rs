use std::time::Instant;

pub struct Checkpoint {
    pub name: String,
    pub instant: Instant,
}

impl Checkpoint {
    pub fn new(name: &str) -> Checkpoint {
        Checkpoint {
            name: String::from(name),
            instant: Instant::now(),
        }
    }
}
