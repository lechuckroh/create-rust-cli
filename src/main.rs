use clap::Parser;

use exif_rename::{format_filename, read_exif_file};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Exif filename
    #[arg(short, long)]
    exif: Option<String>,

    /// filename pattern
    #[arg(short, long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();

    let exif_filename: &str = args.exif.as_ref().map_or("", String::as_str);
    let pattern: &str = args.pattern.as_str();

    match read_exif_file(exif_filename) {
        Ok(exif_vars) => {
            let filename = format_filename(pattern, exif_vars);
            println!("{}", filename)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
