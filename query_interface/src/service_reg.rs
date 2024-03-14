use futures::{Future, Stream};

pub struct ScraperService {
    name: String,
}

// TODO: Will break this apart into separate traits later on as we figure out what they are...
pub trait CoreQuery {
    // Each scraper can define these id types to be whatever works the best for them.
    type PlayerId;
    type TournamentId;
    type SetId;
    type GameId;

    // TODO: Look at defining these behind a macro...
    fn get_all_known_player_ids(&self) -> impl Future<Output = impl Stream<Item = Self::PlayerId>> + Send;
    fn get_all_known_tournament_ds(&self) -> impl Future<Output = impl Stream<Item = Self::TournamentId>> + Send;
    fn get_all_known_set_ids(&self) -> impl Future<Output = impl Stream<Item = Self::SetId>> + Send;
    fn get_all_known_game_ids(&self) -> impl Future<Output = impl Stream<Item = Self::GameId>> + Send;

    fn get_all_sets_of_tournament(&self, id: Self::TournamentId) -> impl Future<Output = impl Stream<Item = Self::SetId>> + Send;
    fn get_all_games_of_set(&self, id: Self::SetId) -> impl Future<Output = impl Stream<Item = Self::GameId>> + Send;
}
