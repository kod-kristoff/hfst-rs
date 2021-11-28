pub struct OutputStream;


impl OutputStream {
    pub fn from_name(name: &str) -> Self {
        println!("Creating OutputStream from '{}'", name);
        Self {}
    }
}
