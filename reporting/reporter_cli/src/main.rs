mod dispatcher;
mod message_loop;
mod pages;

use message_loop::message_loop;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug)]
struct App {}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}

fn main() {
    message_loop();
}
