pub struct ServerOpts {
    pub port:Option<u16>,
    pub hostname:Option<String>,
    pub threads:Option<usize>
}

pub fn get_user_options(config_file:Option<String>) -> std::io::Result<ServerOpts> {
    let ServerArguments{mut port, mut hostname, mut threads, config} = get_arguments();

    if let Some(filename) = config {
        if let Some(opts) = read_config_file(&filename)? {
            port = port.or(opts.port);
            hostname = hostname.or(opts.hostname);
            threads = threads.or(opts.threads);
        }
    } else if let Some(filename) = config_file {
        if let Some(opts) = read_config_file(&filename)? {
            port = port.or(opts.port);
            hostname = hostname.or(opts.hostname);
            threads = threads.or(opts.threads);
        }
    }

    Ok(ServerOpts { port, hostname, threads })
}

fn open_config_file(name:&str) -> std::io::Result<Option<String>> {
    match std::fs::read_to_string(name) {
        Ok(str) => Ok(Some(str)),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(None),
            _ => Err(e)
        }
    }
}

fn read_config_file(name:&str)->std::io::Result<Option<ServerOpts>> {
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

struct ServerArguments {
    pub port:Option<u16>,
    pub hostname:Option<String>,
    pub threads:Option<usize>,
    pub config:Option<String>
}

fn get_arguments() -> ServerArguments {
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
        }
    }

    ServerArguments{ port, hostname, config, threads}
}