// views/ui.rs
use iced::{
    executor, Application, Command, Element, Length, Settings,
};
use iced::widget::{Button, Column, Container, Text};

use crate::hygge::HyggeDeck; // assuming hygge::HyggeDeck is defined in your hygge module

#[derive(Debug, Clone)]
pub enum Message {
    DrawCard,
    Exit,
}

pub struct HyggeUI {
    deck: HyggeDeck,
    current_text: String,
}

impl HyggeUI {
    fn new_deck() -> HyggeUI {
        // Load and shuffle the deck (using your existing logic)
        let mut deck = match HyggeDeck::load_from_file("data/hygge.json") {
            Ok(deck) => deck,
            Err(e) => panic!("Failed to load deck: {}", e),
        };
        deck.shuffle();

        HyggeUI {
            deck,
            current_text: String::from("Welcome to Hygge Game!"),
        }
    }
}

impl Application for HyggeUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme; // Using the default theme

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (HyggeUI::new_deck(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Hygge Game CLI UI")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::DrawCard => {
                if let Some(card) = self.deck.draw() {
                    self.current_text = card.text;
                } else {
                    self.current_text = String::from("The deck is empty!");
                }
            }
            Message::Exit => {
                // Exit the process – note that iced will terminate.
                std::process::exit(0);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let content = Column::new()
            .spacing(20)
            .push(Text::new(&self.current_text).size(30))
            .push(
                Button::new(Text::new("Draw a card"))
                    .on_press(Message::DrawCard)
            )
            .push(
                Button::new(Text::new("Exit"))
                    .on_press(Message::Exit)
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

/// Run the UI application
pub fn run() -> iced::Result {
    HyggeUI::run(Settings::default())
}