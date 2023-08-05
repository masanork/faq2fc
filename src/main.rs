use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input_file = String::new();
    let mut output_file = String::new();

    if args.len() == 1 {
        print_usage();
        return;
    }

    // Parse command-line arguments
    for i in 1..args.len() {
        match args[i].as_str() {
            "-v" | "--version" => {
                println!("faq2fc version 1.0.0");
                return;
            }
            "-h" | "--help" => {
                print_usage();
                return;
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = args[i + 1].clone();
                }
            }
            _ => {
                if input_file.is_empty() {
                    input_file = args[i].clone();
                }
            }
        }
    }

    if input_file.is_empty() {
        println!("Error: Input file not specified.");
        print_usage();
        return;
    }

    // If output file is not specified, use input_file.json
    if output_file.is_empty() {
        let input_path = Path::new(&input_file);
        let file_stem = input_path.file_stem().unwrap();
        let file_stem_str = file_stem.to_str().unwrap();
        output_file = format!("{}.json", file_stem_str);
    }

    // Read input file
    if let Ok(lines) = read_lines(&input_file) {
        let mut prompt = String::new();
        let mut output_data = String::new();

        for line in lines {
            if let Ok(line_content) = line {
                if line_content.starts_with("## ") {
                    prompt = line_content.replace("## ", "").trim().to_string();
                } else if line_content.starts_with("### ") {
                    let completion = line_content.replace("### ", "").trim().to_string();
                    let json_entry = format!(
                        "{{\"prompt\": \"{}\", \"completion\": \"{}\"}}\n",
                        prompt, completion
                    );
                    output_data.push_str(&json_entry);
                }
            }
        }

        // Write output to file
        if let Err(e) = write_output_file(&output_file, &output_data) {
            println!("Error writing to output file: {}", e);
            return;
        }
    }

    // Helper function to print the usage instructions
    fn print_usage() {
        println!("Usage: faq2fc [OPTIONS] <input_file>");
        println!();
        println!("Options:");
        println!("-v, --version   Display version information");
        println!("-h, --help      Display usage instructions");
        println!("-o, --output    Specify an output file");
    }

    // Helper function to read lines from a file
    fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    // Helper function to write output to a file
    fn write_output_file<P>(filename: &P, data: &str) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
