use clap::{App, Arg};
use image::io::Reader as ImageReader;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let matches = App::new("WebP to PNG Converter")
        .version("1.0")
        .author("Chris Cashman")
        .about("Converts all WebP images in a directory to PNG format")
        .arg(
            Arg::with_name("directory")
                .short('d')
                .long("directory")
                .takes_value(true)
                .required(true)
                .help("The directory to search for WebP files"),
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();

    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path
            .extension()
            .map_or(false, |e| e.to_str() == Some("webp"))
        {
            println!("Converting file: {}", path.display());

            let img = match ImageReader::open(path) {
                Ok(reader) => reader.decode(),
                Err(e) => {
                    eprintln!("Failed to read '{}': {}", path.display(), e);
                    continue;
                }
            };

            let img = match img {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Failed to decode '{}': {}", path.display(), e);
                    continue;
                }
            };

            let new_path = path.with_extension("png");
            match img.save(&new_path) {
                Ok(_) => println!("Successfully converted to: {}", new_path.display()),
                Err(e) => {
                    eprintln!("Failed to save '{}': {}", new_path.display(), e);
                    continue;
                }
            };
        }
    }
}
