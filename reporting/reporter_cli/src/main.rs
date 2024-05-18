mod dispatcher;
mod pages;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug)]
struct App {

}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized {
        todo!()
    }
}

fn main() {
    todo!()
}
