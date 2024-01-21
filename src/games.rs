use crate::steam::steamgames::SteamGame;

pub struct Game {
    pub appid: u64,
    pub name: String,
    pub playtime: u64,
    pub platform: String
}

pub struct Games {
    pub games: Vec<Game>
}

impl Game {
    fn new(appid: u64, name: String, playtime: u64, platform: String) -> Game {
        Game {appid, name, playtime, platform}
    }

    fn add_percentage_achievment(&self){
        // pass
    }
}

impl Games {
    fn value(&self) -> &Vec<Game> {
        &self.games
    }

    fn new(games: Vec<Game>) -> Games {
        Games {games}
    }

    pub fn from_steam_games(steam_games: Vec<SteamGame>) -> Games {
        let games: Vec<Game> = steam_games
            .into_iter()
            .map(|steam_game| Game::new(
                steam_game.appid,
                steam_game.name,
                steam_game.playtime_forever,
                String::from("Steam"),
            ))
            .collect();

        Games { games }
    }
}