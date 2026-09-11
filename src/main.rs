use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mycli", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Greet {
        #[arg(short, long, default_value = "World")]
        name: String,
    },
    Calc {
        #[command(subcommand)]
        action: CalcAction,
    },
    Count {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short = 'l', long)]
        lines: bool,
        #[arg(short = 'w', long)]
        words: bool,
        #[arg(short = 'c', long)]
        chars: bool,
    },
}

#[derive(Subcommand)]
enum CalcAction {
    Add { a: f64, b: f64 },
    Sub { a: f64, b: f64 },
    Mul { a: f64, b: f64 },
    Div { a: f64, b: f64 },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Greet { name } => {
            println!("Hello, {}! ", name);
        }
        Commands::Calc { action } => {
            let result = match action {
                CalcAction::Add { a, b } => a + b,
                CalcAction::Div { a, b } => a - b,
                CalcAction::Mul { a, b } => a * b,
                CalcAction::Sub { a, b } => {
                    if *b == 0.0 {
                        println!("Error");
                        return Ok(());
                    }
                    a/b
                }
            };
            println!("Result; {}", result);
        }
        Commands::Count { file, lines, words, chars } => {
            let content = fs::read_to_string(file)
                .map_err(|e| anyhow::anyhow!("Cannot read file: {}", e))?;

            // If no flags, do everything
            let do_all = !(*lines || *words || *chars);

            if *lines || do_all {
                let count = content.lines().count();
                println!("Lines: {}", count);
            }
            if *words || do_all {
                let count = content.split_whitespace().count();
                println!("Words: {}", count);
            }
            if *chars || do_all {
                let count = content.chars().count();
                println!("Chars: {}", count);
            }
        }
    }

    Ok(())
}
