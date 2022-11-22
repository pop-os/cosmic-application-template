use cosmic::iced::widget::{
    button, checkbox, column, container, progress_bar, row, scrollable, slider, text_input,
    toggler, vertical_rule, vertical_space,
};
use cosmic::iced::{executor, Alignment, Application, Command, Length, Settings};
use cosmic::{Element, Theme};
use iced_sctk::application::SurfaceIdWrapper;

use crate::config;

pub fn run() -> cosmic::iced::Result {
    let settings = Settings::default();
    CosmicApplicationTemplate::run(settings)
}

#[derive(Default)]
struct CosmicApplicationTemplate {
    theme: Theme,
    input_value: String,
    slider_value: f32,
    checkbox_value: bool,
    toggler_value: bool,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    ButtonPressed,
    SliderChanged(f32),
    CheckboxToggled(bool),
    TogglerToggled(bool),
    Closed(SurfaceIdWrapper),
}

impl Application for CosmicApplicationTemplate {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (CosmicApplicationTemplate::default(), Command::none())
    }

    fn title(&self) -> String {
        config::APP_ID.to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::ButtonPressed => {}
            Message::SliderChanged(value) => self.slider_value = value,
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::TogglerToggled(value) => self.toggler_value = value,
            Message::Closed(_) => todo!(),
        }
        Command::none()
    }

    fn view(&self, _: SurfaceIdWrapper) -> Element<Message> {
        let text_input = text_input(
            "Type something...",
            &self.input_value,
            Message::InputChanged,
        )
        .padding(10)
        .size(20);

        let button = button("Submit")
            .padding(10)
            .on_press(Message::ButtonPressed);

        let slider = slider(0.0..=100.0, self.slider_value, Message::SliderChanged);

        let progress_bar = progress_bar(0.0..=100.0, self.slider_value);

        let scrollable = scrollable(
            column![
                "Scroll me!",
                vertical_space(Length::Units(800)),
                "You did it!"
            ]
            .width(Length::Fill),
        )
        .height(Length::Units(100));

        let checkbox = checkbox("Check me!", self.checkbox_value, Message::CheckboxToggled);

        let toggler = toggler(
            String::from("Toggle me!"),
            self.toggler_value,
            Message::TogglerToggled,
        )
        .width(Length::Shrink)
        .spacing(10);

        let content = column![
            row![text_input, button].spacing(10),
            slider,
            progress_bar,
            row![
                scrollable,
                vertical_rule(38),
                column![checkbox, toggler].spacing(20)
            ]
            .spacing(10)
            .height(Length::Units(100))
            .align_items(Alignment::Center),
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme
    }

    fn close_requested(&self, id: iced_sctk::application::SurfaceIdWrapper) -> Self::Message {
        Message::Closed(id)
    }
}
