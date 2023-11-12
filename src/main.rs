use std::process;
use std::env;
use minigrep::lib::Config;
use minigrep::lib::run;

fn main() {
    let args: Vec<String> = env::args().collect();

        let config = Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Porblem parsing arguments: {err}");
            process::exit(1);
        });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod test;
