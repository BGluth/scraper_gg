use chrono::Duration;

use super::store_utils::Store;
use crate::types::{DateTime, Provider, RollingAverage};

#[derive(Copy, Clone, Debug)]
pub enum StatsStoreAction {}

#[derive(Clone, Debug)]
pub struct StatsStore {
    name: String,
    tourney_stats: Vec<TourneyStats>,
    time_stats: TimeStats,
}

impl StatsStore {
    pub(super) async fn new<P: Provider>(provider: &P) -> Self {
        todo!()
    }
}

impl Store for StatsStore {
    type Action = StatsStoreAction;

    fn update(&mut self, action: Self::Action) {
        match action {}
    }
}

#[derive(Clone, Debug)]
struct TourneyStats {
    p_stats: RegedPlayerStats,
    g_stats: GameStats,
}

#[derive(Clone, Debug)]
struct RegedPlayerStats {
    num_reged: usize,

    /// The number of players that have checked-in and completed any other pre-reqs (eg. have payed).
    num_checked_in: usize,
}

#[derive(Clone, Debug)]
struct GameStats {
    num_games: usize,
    num_completed: usize,
}

#[derive(Clone, Debug)]
struct TimeStats {
    start_time: DateTime,
    avg_set_duration: RollingAverage<f32>,
}

impl TimeStats {
    fn process_completed_set_dur(&mut self, dur: Duration) {
        todo!()
    }

    fn get_estimated_tourney_completion_time(&self) -> DateTime {
        todo!()
    }
}

#[derive(Debug)]
struct ApiStats {
    avg_resp_time: RollingAverage<f32>,
    complexity_buf: QueryComplexityBuffer,
}

#[derive(Debug)]
struct QueryComplexityBuffer {
    curr_complexity: usize,
    backoff_complexity: usize,
}
