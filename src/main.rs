//! Tool to help building stm32 hal

use self::Cmd::*;
use std::{
    env, env::Args, error::Error, fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
};
use stm32builder::{DeviceId, Device, device::DeviceIn, render, Context};

fn usage() {
    println!("Available commands:");
    println!("   decode <id>   - Decode an device identification number");
    println!("   parse <device>");
    println!("                 - Print the parsed data found on <device> file before being converted");
    println!("   list <device>");
    println!("                 - List all devices this <device> file support");
    println!("   show <id> <device> [device|info|gpio|rcc]");
    println!("                 - Show device informations from <device> file that match <id> device");
    println!("                   Select 'device' to show all data (the default), 'info' for device informations only");
    println!("   print <id> <device> [device|info|gpio|rcc]");
    println!("                 - Print device information as passed to template");
    println!("   render <id> <device> <template> <output> [device|info|gpio|rcc]");
    println!("                 - Render <template> to <output> from <device> informations matching <id>");
    println!("   help          - Print this message");
}

enum Cmd {
    Decode { id: DeviceId },
    Parse { device: File },
    List { device: File },
    Show { id: DeviceId, device: File, data: Data },
    Print { id: DeviceId, device: File, data: Data },
    Render { id: DeviceId, device: File, template: File, output: File, data: Data },
    Help,
}

enum Data {
    Device,
    Info,
    Gpio,
    Rcc,
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
        List { device } => {
            let device = DeviceIn::from_file(&device)?;
            let device_name = device.name;
            Ok(device.parts.iter().for_each(|part| {
                let part_name = &part.name.0;
                part.parts.iter().for_each(|part| println!("{}{}{}", device_name, part_name, part));
            }))
        }
        Show { id, device, data } => {
            let device = Device::from_id_and_file(&id, &device)?;
            match data {
                Data::Device => Ok(println!("{:#?}", device)),
                Data::Info => Ok(println!("{:#?}", device.info)),
                Data::Gpio => Ok(println!("{:#?}", device.peripherals.gpio)),
                Data::Rcc => Ok(println!("{:#?}", device.peripherals.rcc)),
            }
        }
        Print { id, device, data } => {
            let device = Device::from_id_and_file(&id, &device)?;
            Ok(println!("{}", match data {
                Data::Device => serde_yaml::to_string(&device)?,
                Data::Info => serde_yaml::to_string(&device.info)?,
                Data::Gpio => serde_yaml::to_string(&device.peripherals.gpio)?,
                Data::Rcc => serde_yaml::to_string(&device.peripherals.rcc)?,
            }))
        }
        Render { id, device, mut template, mut output, data, } => {
            let device = Device::from_id_and_file(&id, &device)?;
            let context = Context::new();
            Ok(match data {
                Data::Device => render(&device, &mut template, &mut output, &context)?,
                Data::Info => render(&device.info, &mut template, &mut output, &context)?,
                Data::Gpio => render(&device.peripherals.gpio, &mut template, &mut output, &context)?,
                Data::Rcc => render(&device.peripherals.rcc, &mut template, &mut output, &context)?,
            })
        }
    }
}

impl Data {
    pub fn from_arg(arg: &str) -> Result<Data, CliError> {
        match arg {
            "device" => Ok(Data::Device),
            "info" => Ok(Data::Info),
            "gpio" => Ok(Data::Gpio),
            "rcc" => Ok(Data::Rcc),
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
            "list" => Ok(List {
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
            "render" => Ok(Render {
                id: DeviceId::from_str(&args[2])?,
                device: File::open(&args[3])?,
                template: File::open(&args[4])?,
                output: File::create(&args[5])?,
                data: args.get(6).map_or(Data::Device, |arg| Data::from_arg(arg).unwrap()),
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
