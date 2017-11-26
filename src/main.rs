use std::env;
use std::path::PathBuf;
use std::process;

struct BuildJob {
    source_directory: PathBuf
}

impl BuildJob {
    fn build(&self) {
        process::Command::new("./configure")
            .arg(String::from("--prefix=") + self.prefix().as_str())
            .status().
            expect("Configure failed");

        process::Command::new("make").status().expect("make failed");
    }

    fn name(&self) -> String {
        let dir_name = self.source_directory.components().last().unwrap();
        match dir_name {
            std::path::Component::Normal(name) => {
                 name.to_os_string().into_string().unwrap()
            },
            _ => {
                println!("Failed");
                "Failed".to_string()
            }

        }
    }

    fn prefix(&self) -> String {
        String::from("/home/kirillvr/.kitaman/pool/") + self.name().as_str()
    }
}

fn main() {
    let command = env::args().nth(1).expect("Please provide at least one argument");

    let job = BuildJob{source_directory: env::current_dir().expect("Failed to get current directory")};
    if command == "build" {
        job.build();
    }
    if command == "prefix" {
        job.prefix();
    }

    if command == "name" {
        let name = job.name();
        println!("{}", name);
    }
}
