# WebP to PNG Converter

## Description
This application recursively converts all `.webp` images within a specified directory and its subdirectories to `.png` format. It ensures that existing `.png` files are not overwritten by appending an incremented number to the filename of new files if a `.png` file with the same name already exists.

## Prerequisites
- Rust

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/ChrispenCrash/webp-to-png.git
   cd webp-to-png

2. Build the project:
   ```bash
   cargo build --release

3. The executable will be located in the `target/release` directory.

## Usage
To use the application, you need to specify the directory containing the `.webp` images you want to convert. Run the application with the following command:
```bash
cargo run -- -d /path/to/your/webp/images
```