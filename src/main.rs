//! Tool to help building stm32 hal

use self::Cmd::*;
use std::{
    env, env::Args, error::Error, fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
};
use stm32builder::{DeviceId, Device};

fn usage() {
    println!("Available commands:");
    println!("   decode <id>   - Decode an device identification number");
    println!("   show <id> <device>");
    println!("                 - Show device informations from <device> file that match <id> device");
    println!("   help          - Print this message");
}

enum Cmd {
    Decode { id: DeviceId },
    Show { id: DeviceId, device: File },
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
        Show { id, device } => {
            let device = Device::from_id_and_file(&id, &device)?;
            Ok(println!("{:#?}", device))
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
            "show" => Ok(Show {
                id: DeviceId::from_str(&args[2])?,
                device: File::open(&args[3])?,
            }),
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
