use chrono::Duration;

use crate::types::{DateTime, RollingAverage};

#[derive(Debug)]
struct Stats {
    name: String,
    tourney_stats: Vec<TourneyStats>,
    time_stats: TimeStats,
}

#[derive(Debug)]
struct TourneyStats {
    p_stats: RegedPlayerStats,
    g_stats: GameStats,
}

#[derive(Debug)]
struct RegedPlayerStats {
    num_reged: usize,

    /// The number of players that have checked-in and completed any other pre-reqs (eg. have payed).
    num_checked_in: usize,
}

#[derive(Debug)]
struct GameStats {
    num_games: usize,
    num_completed: usize,
}

#[derive(Debug)]
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
