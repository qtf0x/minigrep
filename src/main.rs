use minigrep::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};

fn main() {
    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(cfg) {
        eprintln!("Application error: {err}");
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
    fn build<T>(mut args: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next(); // throw out exe name

        let Some(query) = args.next() else {
            return Err("Didn't get a query string");
        };

        let Some(file_path) = args.next() else {
            return Err("Didn't get a file path");
        };

        Ok(Self {
            query,
            file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}
