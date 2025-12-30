use minigrep::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(cfg) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file_path)?;

    let res = if cfg.ignore_case {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };

    for line in res {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Self {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}
