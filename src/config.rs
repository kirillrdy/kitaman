pub fn kitaman_dir() -> String {
    String::from("/home/kirillvr/.kitaman/")
}
pub fn store_dir() -> String {
    Config::kitaman_dir() + "store"
}
pub fn builder_dir() -> String {
    Config::kitaman_dir() + "build"
}
//TODO some function to create them all if missing
