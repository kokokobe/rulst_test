use iced::{Application, Command, Element, Settings, Text, TextInput, Column, Length, HorizontalAlignment, Scrollable, scrollable, Container, text_input};

use iced::widget::text_input::State;

fn main() {
    MouseEvent::run(Settings::default())
}

enum MouseEvent {
    Stopped(Position),
    Moving,
}

struct Position {
    coordinate: (i32, i32),
    input: text_input::State,
}

#[derive(Debug)]
struct MouseInput {
    all_skip_input: text_input::State,
    confirm_input: String,
    backward_input: String,
    fight_input: String,
    pass_input: String,
}

#[derive(Debug, Clone)]
pub enum MouseMessage {
    //大跳
    AllSkip,
    //确认
    Confirm,
    //返回
    Backward,
    //挑战
    Fight,
    //关卡
    Pass,
}

impl Application for MouseEvent {
    type Executor = iced::executor::Default;
    type Message = MouseMessage;
    type Flags = ();

    fn new(_flags: ()) -> (MouseEvent, Command<Self::Message>) {
        (MouseEvent::Stopped(Position { coordinate: (0, 0), input: text_input::State::new() }), Command::none())
    }

    fn title(&self) -> String {
        String::from("Mouse Tracker")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self {
            MouseEvent::Moving => loading_message(),
            MouseEvent::Stopped(pos) => {
                let title = Text::new("todos")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);
                let input = TextInput::new(
                    &mut pos.input,
                    "What needs to be done?",
                    "",
                    |_arg| {
                        println!("changed input");
                        MouseMessage::AllSkip
                    },
                ).padding(15).size(30);
                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title)
                    .push(input);
                Column::new().push(content).into()
            }
        }
    }
}

fn loading_message() -> Element<'static, MouseMessage> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}
