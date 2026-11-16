const USAGE: &str = "usage: xiv-otp [--setup | -s]";

pub(crate) enum Command {
    Run,
    Setup,
}

impl Command {
    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        match args.nth(1).as_deref() {
            None => Ok(Self::Run),
            Some("--setup" | "-s") => Ok(Self::Setup),
            Some(_) => Err(USAGE),
        }
    }
}
