use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedTournament<I> {
    t_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedBracket<I> {
    b_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedSet<I> {
    s_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedGame<I> {
    g_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedPlayerGameInfo<I> {
    pg_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedPlayer<I> {
    p_id: I,
}
