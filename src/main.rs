use std::path::Path;

mod hygge;
mod views;

fn main() {
    // Ensure the hygge cards are available (your existing logic)
    if !Path::new("data/hygge.json").exists() {
        hygge::generate_hygge_cards();
    }

    // Launch the UI which now contains the CLI functionality as buttons.
    if let Err(e) = views::ui::run() {
        eprintln!("Failed to launch UI: {}", e);
    }
}