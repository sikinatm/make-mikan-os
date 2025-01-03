use std::io;
use std::io::{BufRead, Write};
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    input_file_path: String,
    output_file_path: String,
}

struct Font8x16 {
    bitmap: [u8; 16],
}

struct BDFParser;

impl BDFParser {
    pub fn parse(file_path: String)-> Result<Vec<Font8x16>, io::Error> {
        let file = std::fs::File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();
        let mut fonts = Vec::new();
        while let Some(line) = lines.next() {
            let line = line?;
            if !line.starts_with("STARTCHAR") {
                continue;
            }

            while let Some(line) = lines.next() {
                let line = line?;
                if line.starts_with("ENDCHAR") {
                    break;
                }
                if !line.starts_with("BITMAP") {
                    continue;
                }

                let mut bitmap = [0; 16];
                for i in 0..16 {
                    let line = lines.next().unwrap().unwrap();
                    let byte = u8::from_str_radix(&line, 16).unwrap();
                    bitmap[i] = byte;
                }
                fonts.push(Font8x16 {
                    bitmap,
                });
            }
        }

        Ok(fonts)
    }
}

fn main() {
    let args = Args::parse();
    println!("file_path: {}", args.input_file_path);
    let result = BDFParser::parse(args.input_file_path).unwrap();
    let mut file = std::fs::File::create(args.output_file_path).unwrap();
    for font in result {
        file.write_all(&font.bitmap).unwrap();
    }
}