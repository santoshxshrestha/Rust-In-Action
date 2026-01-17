#![allow(unused)]
use iced::Element;
use iced::widget::{Column, column};
use iced::widget::{button, text};

#[derive(Default, Debug)]
pub struct Count {
    value: i64,
}
impl Count {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Reset => {
                self.value = 0;
            }
        }
    }
    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
            button("reset").on_press(Message::Reset)
        ]
    }
}
#[derive(Clone, Debug, Copy)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

fn main() -> iced::Result {
    // let mut counter = Count::default();
    //
    // loop {
    //     let interface = counter.view();
    //     display(&interface);

    //     let messages = interact(&interface);
    //     for message in messages {
    //         counter.update(message);
    //     }
    // }
    iced::run(Count::update, Count::view)
}
