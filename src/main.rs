#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate sha1;
extern crate serde_json;

//TODO not hardcoded path
const KITAMAN_HOME: &str = "/home/kirillvr/.kitaman/";

#[derive(Serialize)]
enum Requirement {
    Binary(&'static str),
}

#[derive(Serialize)]
struct Requirements {
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

fn main() {
    let mut requirements = Requirements::new();
    requirements.add(Requirement::Binary("ruby"));
    let json = serde_json::to_string(&requirements).unwrap();
    let mut hasher = sha1::Sha1::new();
    hasher.update(json.as_bytes());
    println!("{}", hasher.digest())
}
