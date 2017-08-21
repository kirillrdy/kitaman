use software;
use config;

pub fn path(source: software::Source) -> String {
    config::sources_dir() + source.name + ".tar.gz"
}
