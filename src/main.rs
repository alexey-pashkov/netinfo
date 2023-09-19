use std::{env, process};

enum Mode{
    AllDevices,
    Device(String)
}

fn main() {
    start()
}


fn start(){
    match get_args(){
        Ok(mode) => match mode {
            Mode::AllDevices => {
                let devices = netinfo::get_all_devices().unwrap_or_else(|_|panic!("Error"));
                devices.into_iter().for_each(|device| print!("{}", device))
            },
            Mode::Device(dev_name) => {
                match netinfo::get_dev_by_name(dev_name) {
                    Ok(device) => println!("{}", device),
                    Err(err) => {
                        println!("{}", err);
    
                        process::exit(1);
                    }
                }
            }
        },
        Err(err) => {
            println!("{}", err);

            process::exit(1);
        }
    }
}

fn get_args() -> Result<Mode, &'static str> {
    let mut args = env::args().skip(1);

    match args.next(){
        Some(arg) => match &arg[0..] {
            "--all" => Ok(Mode::AllDevices),
            "--name" => match args.next(){
                Some(name) => Ok(Mode::Device(name)),
                None => Err("Device name not specified.")
            },
            _ => Err("Invalid arguments!")
        },
        None => Err("No arguments passed.")
    }
}


