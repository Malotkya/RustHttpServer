use std::{
    io::{Result, ErrorKind},
    fs::read_to_string
};
use crate::ServerOpts;

pub(crate) struct CommandLineArguments {
    pub port:Option<u16>,
    pub hostname:Option<String>,
    pub threads:Option<usize>,
    pub config:Option<String>
}

pub(crate) fn get_cmd_line_args() -> CommandLineArguments {
    let mut port = None;
    let mut hostname = None;
    let mut threads = None;
    let mut config = None;

    for input in std::env::args() {
        if let Some(index) = input.find("=") {
            let key = &input[..index];
            let value = &input[index+1..];

            match key.to_ascii_lowercase().as_str() {
                "port" => {
                    port = Some(value.parse().unwrap())
                },
                "hostname" => {
                    hostname = Some(value.to_owned())
                },
                "config" => {
                    config = Some(value.to_owned())
                },
                "threads" => {
                    threads = Some(value.parse().unwrap())
                },
                key => {
                    panic!("Unknown command line argument: {key}!")
                }
            }
        } else {
           panic!("Unknown command line argument: {input}!")
        }
    }

    CommandLineArguments{ port, hostname, config, threads}
}

fn open_config_file(name:&str) -> Result<Option<String>> {
    match read_to_string(name) {
        Ok(str) => Ok(Some(str)),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(None),
            _ => Err(e)
        }
    }
}

pub(crate) fn read_config_file(name:&str)-> Result<Option<ServerOpts>> {
    open_config_file(name).map(|opt|opt.map(|buffer|{
        let mut port = None;
        let mut hostname = None;
        let mut threads = None;

        for line in buffer.split("\n") {
            let mut parts = line.split("=");

            if let Some(key) = parts.next().map(|s|s.trim()) {
                let value = parts.next()
                    .map(|s|s.trim())
                    .unwrap_or("");

                match key {
                    "port" => match value.parse() {
                        Ok(value) => port = Some(value),
                        Err(_) => println!("Unable to set {} to port value!", value)
                    },
                    "hostname" => hostname = Some(value.to_string()),
                    "threads" => match value.parse() {
                        Ok(value) => threads = Some(value),
                        Err(_) => println!("Unable to set {} to threads value!", value)
                    },
                    _ => println!("Unkown property \"{}\" and will be ignored!", key)
                }
            }
        }

        ServerOpts { port, hostname, threads }
    }))
}

