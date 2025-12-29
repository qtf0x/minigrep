use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.file_path);

    run(cfg);
}

fn run(cfg: Config) {
    let contents =
        fs::read_to_string(cfg.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Self {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
