use chrono::Local;

pub type DateTime = chrono::DateTime<Local>;

#[derive(Debug)]
pub(crate) struct RollingAverage<T> {
    curr_avg: T,
    n_samples: usize,
}

pub(crate) trait Provider {}
