use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustPassArgs {
    /// Use Uppercase
    #[clap(long, short, action=ArgAction::SetTrue)]
    pub uppercase: bool,
    /// Use Lowercase
    #[clap(long, short, action=ArgAction::SetTrue)]
    pub lowercase: bool,
    /// Use Special Characters
    #[clap(long, short, action=ArgAction::SetTrue)]
    pub special_chars: bool,
    /// Use Digits
    #[clap(long, short, action=ArgAction::SetTrue)]
    pub digits: bool,
    /// Username
    pub username: String,
    /// Service Name
    pub service: String,
    /// Length of your Password
    #[clap(default_value_t = 16)]
    pub len: i32,
    /// Seed Value
    #[clap(default_value_t = 0)]
    pub seed: i32,
}
