#[derive(Debug, Clone, Default)]
pub struct Incident {
    name: String,
    content: String,
}

impl ToString for Incident {
    fn to_string(&self) -> String {
        format!("{}: {}\n", self.name, self.content)
    }
}

impl Incident {
    pub fn new(name: String, content: String) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
        }
    }
}
