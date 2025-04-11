// views/ui.rs
use iced::{
    executor, Application, Command, Element, Length, Settings,
};
use iced::font::Weight::Black;
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
        String::from("Hygge")
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
        // Create a vertical layout (Column) to stack widgets on top of each other.
        let content = Column::new()
            // Set the spacing between each child widget in the column to 20 pixels.
            .spacing(20)
            // Add a Text widget that displays the current text stored in the state.
            // The `.size(30)` modifier sets the font size of the text to 30.
            .push(Text::new(&self.current_text).size(30))
            // Add a Button widget with a label "Draw a card".
            // When pressed, it sends the `Message::DrawCard` to trigger drawing a card.
            .push(
                Button::new(Text::new("Draw a card"))
                    .on_press(Message::DrawCard)
                    .style(iced::theme::Button::Primary)

            )
            // Add another Button widget with a label "Exit".
            // When pressed, it sends the `Message::Exit` to trigger exiting the application.
            .push(
                Button::new(Text::new("Exit"))
                    .on_press(Message::Exit)
                    .style(iced::theme::Button::Primary)
            );

        // Wrap the column in a Container widget that provides layout and styling.
        Container::new(content)
            // Set the container's width to fill all available horizontal space.
            .width(Length::Fill)
            // Set the container's height to fill all available vertical space.
            .height(Length::Fill)
            // Center the container's child content horizontally.
            .center_x()
            // Center the container's child content vertically.
            .center_y()
            // Convert the container into an Element, which is the type expected by iced's view function.
            .into()
    }
}

/// Run the UI application
pub fn run() -> iced::Result {
    HyggeUI::run(Settings::default())
}