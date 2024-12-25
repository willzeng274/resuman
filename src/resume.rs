pub struct Resume {
    pub name: String,
    pub template: Option<String>,
    pub public: bool,
    // Add fields like experience, education, etc.
}

impl Resume {
    pub fn new(name: String, template: Option<String>, public: bool) -> Self {
        Resume {
            name,
            template,
            public,
        }
    }
}
