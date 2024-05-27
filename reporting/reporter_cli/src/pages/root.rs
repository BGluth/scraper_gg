use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug)]
pub(crate) struct Root {}

impl Widget for Root {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}

#[derive(Debug)]
struct StatsWidget {}

impl Widget for StatsWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
