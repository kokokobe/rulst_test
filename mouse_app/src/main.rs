use iced::{Application, Command, Element, Settings, Text, TextInput, Column, Length, HorizontalAlignment,
           Scrollable, scrollable, Container, text_input};

use iced::widget::text_input::State;

fn main() {
    MouseEvent::run(Settings::default())
}

enum MouseEvent {
    Stopped(Position),
    Moving(Position),
}

#[derive(Debug, Clone)]
struct Position {
    coordinate: (i32, i32),
    input: text_input::State,
}

#[derive(Debug, Clone)]
enum MouseInput {
    AllSkipInput(Position),
    ConfirmInput(Position),
    BackwardInput(Position),
    FightInput(Position),
    PassInput(Position),
}

#[derive(Debug, Clone)]
pub enum MouseMessage {
    //大跳
    AllSkip(MouseInput),
    //确认
    Confirm(MouseInput),
    //返回
    Backward(MouseInput),
    //挑战
    Fight(MouseInput),
    //关卡
    Pass(MouseInput),
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
            MouseEvent::Moving(pos) => loading_message(),
            MouseEvent::Stopped(pos) => {
                let title = Text::new("todos")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);
                let all_skip_input = TextInput::new(
                    &mut pos.input,
                    "What needs to be done?",
                    "",
                    |_arg| {
                        println!("changed input");
                        MouseMessage::AllSkip
                    },
                ).padding(15).size(30);
                let confirm_input = TextInput::new(
                    &mut pos.input,
                    "What needs to be done?",
                    "",
                    |_arg| {
                        println!("changed input");
                        MouseMessage::AllSkip
                    },
                ).padding(15).size(30);
                let backward_input = TextInput::new(
                    &mut pos.input,
                    "What needs to be done?",
                    "",
                    |_arg| {
                        println!("changed input");
                        MouseMessage::AllSkip
                    },
                ).padding(15).size(30);
                let fight_input = TextInput::new(
                    &mut pos.input,
                    "What needs to be done?",
                    "",
                    |_arg| {
                        println!("changed input");
                        //TODO
                        MouseMessage::AllSkip
                    },
                ).padding(15).size(30);
                let pass_input = TextInput::new(
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
                    .push(all_skip_input)
                    .push(confirm_input)
                    .push(backward_input)
                    .push(fight_input)
                    .push(pass_input);
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
