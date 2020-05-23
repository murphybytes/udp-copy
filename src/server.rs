
pub struct Listener {
    host: String 
}

impl Listener {
    pub fn new(host: String) -> Self {
        Listener{host}
    }
}