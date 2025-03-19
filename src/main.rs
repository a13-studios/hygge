use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};
use dialoguer::{Input, Select, Confirm};
use indicatif::{ProgressBar, ProgressStyle};

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

fn draw_card(deck : &mut hygge::HyggeDeck) {
    if let Some(card) = deck.draw() {
        println!("\n");
        println!("{}", card.text);
        println!("\n");
    } else {
        println!("The deck is empty!");
    }
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

    loop
    {
        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .default(0)
            .item("Draw a card")
            .item("Exit")
            .report(false) // 👈 Hides the selected option
            .interact()
            .unwrap();

        match selection {
            0 => draw_card(&mut deck),
            1 => {
                process::exit(1);
            }
            _ => {
                println!("Invalid selection!");
            }
        }
    }
}
