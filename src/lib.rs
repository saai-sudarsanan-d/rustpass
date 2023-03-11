/*
    Flags
        - username
        - service
        - master-pwd
        - opts
        - length
        - seed
*/

pub struct Flags {
    pub username: String,
    pub service: String,
    pub md: String,
    pub opts: String,
    pub len: String,
    pub seed: String,
}

impl Flags {
    pub fn new(args: &[String]) -> Result<Flags, &'static str> {
        if args.len() < 6 {
            return Err("not enough arguments");
        }

        let username = args[1].clone();
        let service = args[2].clone();
        let md = args[3].clone();
        let opts = args[4].clone();
        let len = args[5].clone();
        let seed = args[6].clone();

        Ok(Flags {
            username,
            service,
            md,
            opts,
            len,
            seed,
        })
    }
}
