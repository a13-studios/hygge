use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::io::Write;


use serde::{Deserialize, Serialize};
use serde_json;
use rand::Rng;
use std::path::Path;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, Debug)]
pub struct HyggeCard {
    id: u16,
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyggeDeck {
    cards : Vec<HyggeCard>
}

impl HyggeDeck {

    pub fn new() -> Self {
        HyggeDeck {
            cards: Vec::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Since the JSON is an array of HyggeCard, deserialize into Vec<HyggeCard>.
        let cards: Vec<HyggeCard> = serde_json::from_reader(reader)?;

        // Then wrap it into a HyggeDeck.
        Ok(Self { cards })
    }

    pub fn export_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        // Convert only the `cards` field to JSON, since we want an array of objects.
        let json_str = serde_json::to_string_pretty(&self.cards)
            .expect("Failed to serialize `Vec<HyggeCard>` to JSON");

        let mut file = File::create(path)?;
        file.write_all(json_str.as_bytes())?;

        println!("HyggeDeck successfully exported as an array of cards.");
        Ok(())
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<HyggeCard> {
        self.cards.pop()
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read: {}", filename));
    contents.lines().map(|line| line.to_string()).collect()
}

pub fn generate_hygge_cards() {
    // Load lines from "data/hygge_cards.txt"
    let hygge_text = read_lines("data/hygge_cards.txt");
    let mut deck = HyggeDeck::new();

    let mut rng = rand::thread_rng();

    // Build the deck
    for line in hygge_text {
        if line.len() > 32 {
            let token: u16 = rng.r#gen();
            let card = HyggeCard { id: token, text: line };
            deck.cards.push(card);
        }
    }

    // Export the deck's cards to "data/hygge.json" as a JSON array
    deck.export_to_file("data/hygge.json").unwrap();
}