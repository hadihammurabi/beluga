use iced::{
    executor, keyboard,
    widget::{column, container, pick_list, row},
    Application,
};

use iced::{Alignment, Command, Element, Length, Settings, Subscription, Theme};

use crate::{layout, message};

#[derive(Debug)]
struct app {
    theme: Theme,
}

pub fn launch() -> iced::Result {
    app::run(Settings::default())
}

impl Application for app {
    type Message = message::Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<message::Message>) {
        (
            Self {
                theme: Theme::Light,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Beluga")
    }

    fn update(&mut self, _message: Self::Message) -> Command<message::Message> {
        match _message {
            // Message::Next => {
            //     self.example = self.example.next();
            // }
            // Message::Previous => {
            //     self.example = self.example.previous();
            // }
            // message::Message::ExplainToggled(explain) => {
            //     self.explain = explain;
            // }
            message::Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<message::Message> {
        // use keyboard::key;

        keyboard::on_key_release(|key, _modifiers| match key {
            // keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::Previous),
            // keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => None,
        })
    }

    fn view(&self) -> Element<message::Message> {
        let header = row![pick_list(
            Theme::ALL,
            Some(&self.theme),
            message::Message::ThemeSelected
        ),]
        .spacing(20)
        .align_items(Alignment::Center);

        let _layout = container(layout::layout())
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Appearance::default().with_border(palette.background.strong.color, 4.0)
            })
            .padding(4)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        // let controls = row([
        //     (!self.example.is_first()).then_some(
        //         button("← Previous")
        //             .padding([5, 10])
        //             .on_press(message::Message::Previous)
        //             .into(),
        //     ),
        //     Some(horizontal_space(Length::Fill).into()),
        //     (!self.example.is_last()).then_some(
        //         button("Next →")
        //             .padding([5, 10])
        //             .on_press(message::Message::Next)
        //             .into(),
        //     ),
        // ]
        // .into_iter()
        // .flatten());

        column![header, _layout].spacing(10).padding(20).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
