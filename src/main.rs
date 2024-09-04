mod arguments;
mod validators;
mod ppm_loader;
mod converter;

use arguments::Args;
use clap::Parser;
use converter::load_and_convert;

fn main() {
    let args = Args::parse();

    println!("Attemping to convert {} to {}...", args.input, args.output);

    match load_and_convert(&args.input, &args.output) {
        Ok(success) => {
            if !success {
                eprintln!("ERROR: conversion returned false unexpectedly.");
                std::process::exit(1);
            } else {
                println!("Image conversion was successful.");
            }

        }
        Err(err) => {
            eprintln!("ERROR: Failed to convert file {}", err);
            std::process::exit(1);
        }
    }
}
