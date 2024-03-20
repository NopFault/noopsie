#[derive(Debug, Clone)]
pub enum AttributeType {
    Uint(u32),
    Int(i32),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: AttributeType,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub attributes: Vec<Attribute>
}

impl Config {
    pub const PER_PAGE: &'static str = "per_page";

    pub fn new() -> Self {
        Config {
            attributes: Vec::new(),
        }
    }

    pub fn add(&mut self, key: String, val: AttributeType) -> &mut Self {
        self.attributes.push(Attribute{ key, value: val});
        self
    }

    pub fn get(self, key:String) -> Attribute {
        self.attributes.iter().find(|attr| attr.key == key).cloned().unwrap()
    }


}
