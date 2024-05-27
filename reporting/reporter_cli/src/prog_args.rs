use clap::Parser;
use reporter_state::rest_provider::GGRestToken;

#[derive(Debug, Parser)]
#[command(version, about, author)]
pub(crate) struct ProgArgs {
    /// The token to use when sending requests to `start.gg`
    #[arg(short = 't')]
    gg_token: GGRestToken,
}
