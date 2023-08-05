use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_usage();
            }
            "-v" | "--version" => {
                print_version();
            }
            "-o" | "--output" => {
                if args.len() < 4 {
                    println!("Error: Output file not specified.");
                    print_usage();
                } else {
                    let input_file = &args[2];
                    let output_file = &args[3];
                    if let Err(err) = convert_to_fine_tuning_json(input_file, output_file) {
                        println!("Error: {}", err);
                    }
                }
            }
            _ => {
                let input_file = &args[1];
                let output_file = &format!("{}.json", input_file);
                if let Err(err) = convert_to_fine_tuning_json(input_file, output_file) {
                    println!("Error: {}", err);
                }
            }
        }
    } else {
        print_usage();
    }
}

fn convert_to_fine_tuning_json(input_file: &str, output_file: &str) -> io::Result<()> {
    let input_content = fs::read_to_string(input_file)?;
    let mut output_content = String::new();

    let mut lines = input_content.lines();
    let mut prompt = String::new();

    while let Some(line) = lines.next() {
        if line.starts_with("## ") {
            if !prompt.is_empty() {
                output_content.push_str(&format!("{{\"prompt\": \"{}\", \"completion\": \"{}\"}}\n", prompt, line[3..].trim()));
            }
            prompt = line[3..].trim().to_string();
        } else if line.starts_with("### ") {
            let completion = line[4..].trim();
            output_content.push_str(&format!("{{\"prompt\": \"{}\", \"completion\": \"{}\"}}\n", prompt, completion));
        } else if line.starts_with("#### ") {
            output_content.push_str(&format!("// {}\n", line[5..].trim()));
        } else {
            output_content.push_str(&format!("// {}\n", line.trim()));
        }
    }

    if !prompt.is_empty() {
        output_content.push_str(&format!("{{\"prompt\": \"<prompt text>\", \"completion\": \"<ideal generated text>\"}}\n"));
    }

    let mut output_file = fs::File::create(output_file)?;
    output_file.write_all(output_content.as_bytes())?;

    println!("Conversion completed successfully.");
    Ok(())
}

fn print_usage() {
    println!("Usage: faq2fc [OPTIONS] [INPUT_FILE]");
    println!("Options:");
    println!("  -h, --help           Display usage instructions.");
    println!("  -v, --version        Display version information.");
    println!("  -o, --output FILE    Specify an output file.");
}

fn print_version() {
    println!("faq2fc version 1.0");
}
