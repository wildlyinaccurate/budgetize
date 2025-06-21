use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum FormatValue {
    Westpac,
    Amex,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    format: FormatValue,

    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    if !std::path::Path::new(&args.input).exists() {
        eprintln!("File '{}' does not exist.", args.input);
        std::process::exit(1);
    }

    let file = std::fs::File::open(&args.input).expect("Failed to open input file");
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        println!("{:?}", record);
    }
}
