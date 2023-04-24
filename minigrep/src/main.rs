use minigrep::{run, Config};
use std::error::Error;
use std::{env};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args)?;
    run(config)
}
