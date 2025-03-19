use std::path::Path;

mod hygge;

fn main()
{
    // check if hygge cards are available in json file
    if  !Path::new("data/hygge.json").exists() {
        hygge::generate_hygge_cards();
    }

    let deck = hygge::HyggeDeck::load_from_file("data/hygge.json");
    // println!("Loaded deck: {:?}", deck);
}
