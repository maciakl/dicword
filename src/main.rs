use clap::Parser;


/// Generates a random dictionary word of arbitrary length.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Length of the word
    #[arg(short, long, default_value_t = 5)]
    length: usize,
}

fn main() {

    let args = Args::parse();

    let len = args.length;
    
    if len > 15 {
        eprintln!("unable to generate words longer than 15 characters");
        std::process::exit(1);
    }

    let word = random_word::gen_len(len, random_word::Lang::En).unwrap();

    println!("{}", word);
}
