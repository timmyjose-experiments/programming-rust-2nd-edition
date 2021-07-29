use regex::Regex;
use std::fs;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );

    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();

    let contents = match fs::read_to_string(&args.filename) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!(
                "{} failed to read from file `{}`: {:?}",
                "Error".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_contents = match replace(&args.target, &args.replacement, &contents) {
        Ok(replaced_contents) => replaced_contents,
        Err(e) => {
            eprintln!("{} - failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_contents) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "{} failed to write to fil `{}`: {:?}",
                "Error".red().bold(),
                args.output,
                e
            );
            std::process::exit(1);
        }
    }
}
