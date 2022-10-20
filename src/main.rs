use clap::{Parser, Subcommand};
use rtk_cli::{decompress, compress};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        #[arg(short = 'i', long, value_name = "FILE", required(true))]
        input: String,
        #[arg(short = 'o', long, value_name = "FILE", required(true))]
        output: String,
    },
    Decompress {
        #[arg(short = 'i', long, value_name = "FILE", required(true))]
        input: String,
        #[arg(short = 'o', long, value_name = "FILE", required(true))]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Compress { input, output }) => {
            compress(input, output).expect("Failed to compress.");
        }
        Some(Commands::Decompress { input, output }) => {
            decompress(input, output).expect("Failed to decompress.");
        }
        None => {}
    }
}
