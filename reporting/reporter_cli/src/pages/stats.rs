use ratatui::{buffer::Buffer, layout::Rect, widgets::WidgetRef};
use reporter_state::stores::{stats::StatsStore, store_utils::StoreDataRef};

#[derive(Debug)]
pub(crate) struct Root {
    stats_store: StoreDataRef<StatsStore>,
}

impl Root {
    pub(crate) fn new(stats_store: StoreDataRef<StatsStore>) -> Self {
        todo!()
    }
}

impl WidgetRef for Root {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}

#[derive(Debug)]
struct StatsWidget {}

impl WidgetRef for StatsWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
