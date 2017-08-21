#[derive(Serialize)]
pub enum Requirement {
    Binary(&'static str),
}

#[derive(Serialize)]
pub struct Requirements {
    requirements: Vec<Requirement>,
}

impl Requirements {
    pub fn new() -> Requirements {
        Requirements {
            requirements: Vec::new(),
        }
    }
    pub fn add(&mut self, requirement: Requirement) {
        self.requirements.push(requirement)
    }
}
