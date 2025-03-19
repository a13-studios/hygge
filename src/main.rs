use std::path::Path;
use std::process;
use clap::{Parser, Subcommand};

mod hygge;

#[derive(Parser)]
#[command(name = "hygge")]
#[command(version = "1.0")]
#[command(about = "Infinite Hygge CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Draw a card
    Draw,

    Exit
}


fn main()
{
    // check if hygge cards are available in json file
    if  !Path::new("data/hygge.json").exists() {
        hygge::generate_hygge_cards();
    }

    let mut deck = match hygge::HyggeDeck::load_from_file("data/hygge.json") {
        Ok(deck) => deck,
        Err(e) => panic!("Failed to load deck: {}", e)
    };

    deck.shuffle();

    let cli = Cli::parse();

    match cli.command {
        Commands::Draw => {
            if let Some(card) = deck.draw() {
                println!("You drew: {}", card.text);
            } else {
                println!("The deck is empty!");
            }
        }
        Commands::Exit => {
            process::exit(1);
        }
    }
}
