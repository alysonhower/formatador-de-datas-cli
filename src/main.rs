use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use regex::Regex;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'E', long = "entrada")]
    input_file: PathBuf,

    #[arg(short = 'S', long = "saida")]
    output_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input_file = args.input_file.with_extension("txt");
    let output_file = args.output_file.with_extension("txt");

    // Open the input file
    let file = File::open(&input_file).expect("Falha ao abrir o arquivo de entrada");
    let reader = BufReader::new(file);

    // Create the output file
    let mut output = File::create(&output_file).expect("Falha ao criar o arquivo de saida");

    // Regular expression pattern to match dates in YYYY-MM-DD format
    let date_pattern = Regex::new(r"\b(\d{4})-(\d{2})-(\d{2})\b").expect("Falha ao criar o padrão de data");

    // Iterate over each line in the input file
    for line in reader.lines() {
        let line = line.expect("Falha ao ler linha");

        // Replace dates in YYYY-MM-DD format with DD-MM-YYYY format
        let modified_line = date_pattern.replace_all(&line, |caps: &regex::Captures| {
            let year = caps.get(1).unwrap().as_str();
            let month = caps.get(2).unwrap().as_str();
            let day = caps.get(3).unwrap().as_str();
            format!("{}-{}-{}", day, month, year)
        });

        // Write the modified line to the output file
        writeln!(output, "{}", modified_line).expect("Falha ao escrever na saida");
    }

    println!("Conversão de datas concluída. Arquivo de saida: {}", output_file.display());
}