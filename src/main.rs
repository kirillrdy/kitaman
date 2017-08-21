#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate sha1;
extern crate serde_json;

mod environment;
mod content;
mod source;
mod software;
mod config;

struct Config {
}





struct Builder {
    source: software::Source
}

struct StoreManager {
}


impl Builder {
    fn install_source(source: software::Source) {
        let builder = Builder{source: source};
        builder.install()
    }

    fn install(&self) {
        //TODO all the hard work

        //TODO finish
        //content::register()
    }
}



impl StoreManager {
    fn install(source: Source) {
        //TODO if not already installed
        Builder::install(source)
    }
}

fn main() {
    let mut requirements = environment::Requirements::new();
    requirements.add(environment::Requirement::Binary("ruby"));
    let json = serde_json::to_string(&requirements).unwrap();
    let mut hasher = sha1::Sha1::new();
    hasher.update(json.as_bytes());
    let digest = format!("{}", hasher.digest());
    println!("{}", digest);

    let source = Source{name: String::from("ruby-2.3.4")};
    StoreManager::install(source)



}
