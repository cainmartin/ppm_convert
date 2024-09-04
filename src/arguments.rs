use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // The input image, must be ppm format
    #[arg(short, long)]
    pub input: String,

    // Output file
    #[arg(short, long)]
    pub output: String,
}