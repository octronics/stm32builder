use self::Cmd::*;
use std::{
    env,
    env::Args,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use stm32builder::DeviceId;

fn usage() {
    println!("Available commands:");
    println!("   decode <id>   - Decode an device identification number");
    println!("   help          - Print this message");
}

enum Cmd {
    Decode { id: DeviceId },
    Help,
}

fn main() -> Result<(), Box<dyn Error>> {
    match Cmd::from_args(env::args())? {
        Help => Ok(usage()),
        Decode { id } => {
            println!("            id: {}", id.id);
            println!("          name: {}", id.name());
            println!("        family: {}", id.family());
            println!("    sub-family: {}", id.sub_family());
            println!("          part: {}", id.part());
            println!("       package: {:?}", id.package);
            println!("    flash size: {}", id.flash_size.0);
            println!(
                "   temperature: {} min to {} max",
                id.temperature.min, id.temperature.max
            );
            Ok(())
        }
    }
}

impl Cmd {
    pub fn from_args(args: Args) -> Result<Self, Box<dyn Error>> {
        let args: Vec<String> = args.collect();

        if args.len() <= 1 {
            return Err(CliError::NumberOfArguments(0).into());
        }

        match args[1].as_str() {
            "decode" => Ok(Decode {
                id: DeviceId::from_str(&args[2])?,
            }),
            "help" => Ok(Help),
            cmd => Err(CliError::UnknownCommand(cmd.to_string()).into()),
        }
    }
}

#[derive(Debug)]
enum CliError {
    UnknownCommand(String),
    NumberOfArguments(usize),
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::CliError::*;
        match *self {
            UnknownCommand(ref cmd) => write!(f, "command '{}' unknown", cmd),
            NumberOfArguments(ref num) => write!(f, "wrong number of arguments, {} given", num),
        }
    }
}
