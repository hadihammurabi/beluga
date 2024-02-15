use crate::message;
use iced::mouse;
use iced::theme;
use iced::widget::{canvas, column, container, horizontal_space, row, scrollable};
use iced::{Alignment, Element, Length, Point, Rectangle, Renderer, Theme};

pub fn layout<'a>() -> Element<'a, message::Message> {
    let header = container(
        row![
            square(40),
            horizontal_space(Length::Fill),
            "Header!",
            horizontal_space(Length::Fill),
            square(40),
        ]
        .padding(10)
        .align_items(Alignment::Center),
    )
    .style(|theme: &Theme| {
        let palette = theme.extended_palette();

        container::Appearance::default().with_border(palette.background.strong.color, 1)
    });

    let sidebar = container(
        column!["Sidebar!", square(50), square(50)]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
    )
    .style(theme::Container::Box)
    .height(Length::Fill)
    .center_y();

    let content = container(
        scrollable(
            column!["Content!", square(400), square(200), square(400), "The end"]
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
        )
        .height(Length::Fill),
    )
    .padding(10);

    column![header, row![sidebar, content]].into()
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, message::Message> {
    struct Square;

    impl canvas::Program<message::Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
