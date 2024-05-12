use clap::{App, Arg};
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
        println!("{:?}", path);
    }
}
