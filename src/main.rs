//! Tool to help building stm32 hal

use self::Cmd::*;
use std::{
    env, env::Args, error::Error, fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
};
use stm32builder::{DeviceId, Device, device::DeviceIn};

fn usage() {
    println!("Available commands:");
    println!("   decode <id>   - Decode an device identification number");
    println!("   parse <device>");
    println!("                 - Print the parsed data found on <device> file before being converted");
    println!("   show <id> <device> [device|info|gpio]");
    println!("                 - Show device informations from <device> file that match <id> device");
    println!("                   Select 'device' to show all data (the default), 'info' for device informations only");
    println!("   print <id> <device> [device|info|gpio]");
    println!("                 - Print device information as passed to template");
    println!("   help          - Print this message");
}

enum Cmd {
    Decode { id: DeviceId },
    Parse { device: File },
    Show { id: DeviceId, device: File, data: Data },
    Print { id: DeviceId, device: File, data: Data },
    Help,
}

enum Data {
    Device,
    Info,
    Gpio,
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
        Parse { device } => {
            let device = DeviceIn::from_file(&device)?;
            Ok(println!("{:#?}", device))
        }
        Show { id, device, data } => {
            let device = Device::from_id_and_file(&id, &device)?;
            match data {
                Data::Device => Ok(println!("{:#?}", device)),
                Data::Info => Ok(println!("{:#?}", device.info)),
                Data::Gpio => Ok(println!("{:#?}", device.peripherals.gpio)),
            }
        }
        Print { id, device, data } => {
            let device = Device::from_id_and_file(&id, &device)?;
            Ok(println!("{}", match data {
                Data::Device => serde_yaml::to_string(&device)?,
                Data::Info => serde_yaml::to_string(&device.info)?,
                Data::Gpio => serde_yaml::to_string(&device.peripherals.gpio)?,
            }))
        }
    }
}

impl Data {
    pub fn from_arg(arg: &str) -> Result<Data, CliError> {
        match arg {
            "device" => Ok(Data::Device),
            "info" => Ok(Data::Info),
            "gpio" => Ok(Data::Gpio),
            arg => Err(CliError::UnknownDataArg(arg.to_owned())),
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
            "parse" => Ok(Parse {
                device: File::open(&args[2])?,
            }),
            "show" => Ok(Show {
                id: DeviceId::from_str(&args[2])?,
                device: File::open(&args[3])?,
                data: args.get(4).map_or(Ok(Data::Device), |arg| Data::from_arg(arg))?,
            }),
            "print" => Ok(Print {
                id: DeviceId::from_str(&args[2])?,
                device: File::open(&args[3])?,
                data: args.get(4).map_or(Data::Device, |arg| Data::from_arg(arg).unwrap()),
            }),
            cmd => Err(CliError::UnknownCommand(cmd.to_string()).into()),
        }
    }
}

#[derive(Debug)]
enum CliError {
    UnknownDataArg(String),
    UnknownCommand(String),
    NumberOfArguments(usize),
}

impl Error for CliError {}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::CliError::*;
        match *self {
            UnknownDataArg(ref args) => write!(f, "data argument '{}' unknown", args),
            UnknownCommand(ref cmd) => write!(f, "command '{}' unknown", cmd),
            NumberOfArguments(ref num) => write!(f, "wrong number of arguments, {} given", num),
        }
    }
}
