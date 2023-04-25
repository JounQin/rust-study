use minigrep::{run, Config};
use std::error::Error;
use std::{env};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args())?;
    run(config)
}
