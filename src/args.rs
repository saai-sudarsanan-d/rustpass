use clap:: {
    Parser,
};

#[derive(Debug, Parser)]
#[clap(author,version,about)]
pub struct RustPassArgs {
    /// Username
    pub username: String,
    /// Service Name
    pub service: String,
    /// Master Password
    pub mpass: String,
    /// Length of your Password
    pub len: i32,
    /// Seed Value
    pub seed: i32,   
}