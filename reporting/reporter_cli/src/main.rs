mod dispatcher;
mod message_loop;
mod pages;
mod prog_args;

use clap::Parser;
use message_loop::message_loop;
use prog_args::ProgArgs;
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

fn main() -> anyhow::Result<()> {
    let p_args = ProgArgs::parse();

    message_loop();

    Ok(())
}
