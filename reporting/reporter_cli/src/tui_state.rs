use std::{collections::VecDeque, fmt::Debug, time::Duration};

use ratatui::widgets::Widget;
use tokio::{
    task,
    time::{sleep_until, Instant},
};

use crate::{
    message_loop::{MsgLoopTx, TuiAction},
    pages::root::Root,
    prog_args::RenderCfg,
};

pub(crate) struct TuiState {
    active_page_stack: VecDeque<Box<dyn Widget>>,
    render_timer: RenderBackoffTimer,
}

impl TuiState {
    pub(crate) fn new(tx: MsgLoopTx, render_time_cfg: RenderCfg) -> Self {
        Self {
            active_page_stack: Self::create_page_queue(),
            render_timer: RenderBackoffTimer::new(tx, render_time_cfg),
        }
    }

    fn create_page_queue() -> VecDeque<Box<dyn Widget>> {
        let mut p_stack = VecDeque::new();
        p_stack.push_front(Box::new(Root::new()) as Box<dyn Widget>);

        p_stack
    }
}

#[derive(Debug)]
/// Used to prevent a flooding of render calls from rendering the TUI way too fast.
struct RenderBackoffTimer {
    last_render_time: Instant,
    min_time_between_renders: Duration,
    render_already_queued: bool,
    msg_loop_tx: MsgLoopTx,
}

impl RenderBackoffTimer {
    fn new(msg_loop_tx: MsgLoopTx, cfg: RenderCfg) -> Self {
        Self {
            last_render_time: Instant::now(),
            min_time_between_renders: cfg.delay_between_renders(),
            render_already_queued: false,
            msg_loop_tx,
        }
    }

    /// Sends a render message back to the message loop or else queues one to be seen as soon as enough time passes.
    async fn render_or_backoff(&mut self) {
        let now = Instant::now();
        let next_render_time = self.last_render_time + self.min_time_between_renders;

        match now >= next_render_time {
            // We're rendering too fast, so we'll potentially queue up a render message for later.
            false => {
                if !self.render_already_queued {
                    self.render_already_queued = true;
                    Self::create_delayed_render_task(self.msg_loop_tx.clone(), next_render_time);
                }
            }

            // Enough time has passed to render.
            true => {
                self.last_render_time = now;
                self.render_already_queued = false;

                Self::send_refresh_msg(&self.msg_loop_tx).await;
            }
        }
    }

    fn create_delayed_render_task(tx: MsgLoopTx, next_render_time: Instant) {
        task::spawn(async move {
            sleep_until(next_render_time).await;
            Self::send_refresh_msg(&tx).await;
        });
    }

    async fn send_refresh_msg(tx: &MsgLoopTx) {
        // Should I think only ever be able to be hit when the other end closes?
        tx.send(TuiAction::TuiRefresh).await.expect("Failed to send a TUI refresh message!");
    }
}
