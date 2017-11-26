use std::env;
use std::path::PathBuf;
use std::process;

struct BuildJob {
    source_directory: PathBuf
}

impl BuildJob {
    fn build(&self) {
        //TODO consider own command so that std{in,our,err} can be controlled
        process::Command::new("./configure")
            .arg(String::from("--prefix=") + self.prefix().as_str())
            .status().
            expect("Configure failed");

        process::Command::new("make").status().expect("make failed");
    }

    fn install(&self) {
        //TODO maybe detect if its necessary to build
        self.build();

        process::Command::new("make")
            .arg("install")
            .status().expect("make install failed");
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

    fn exec(&self, args: Vec<String>) {
        let cmd = &args[0]; //TODO Panic
        let rest_of_args: Vec<&String> = args.iter().skip(1).collect();
        println!("{}", cmd);
        println!("{:?}", rest_of_args);
        process::Command::new(cmd)
            .args(rest_of_args)
            .status().expect("Failed exec");
    }
}

fn main() {
    //TODO proper args parsing
    let command = env::args().nth(1).expect("Please provide at least one argument");

    let job = BuildJob{source_directory: env::current_dir().expect("Failed to get current directory")};
    if command == "build" {
        job.build();
    }
    if command == "install" {
        job.install();
    }
    if command == "prefix" {
        job.prefix();
    }

    if command == "name" {
        let name = job.name();
        println!("{}", name);
    }

    if command == "exec" {
        let args: Vec<String> = env::args().skip(2).collect();
        job.exec(args);
    }
}
