use crate::steam::steamgames::SteamGame;
use crate::gog::goggames::GogGame;

pub struct Game {
    pub userid: i64,
    pub appid: u64,
    pub name: String,
    pub playtime: u64,
    pub platform: String
}

pub struct Games {
    pub games: Vec<Game>
}

impl Game {
    fn new(userid: i64, appid: u64, name: String, playtime: u64, platform: String) -> Game {
        Game {userid, appid, name, playtime, platform}
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


    pub fn from_steam_games(steam_games: Vec<SteamGame>, userid: i64) -> Games {
        let games: Vec<Game> = steam_games
            .into_iter()
            .map(|steam_game| Game::new(
                userid,
                steam_game.appid,
                steam_game.name,
                steam_game.playtime_forever,
                String::from("Steam"),
            ))
            .collect();

        Games { games }
    }


    pub fn from_gog_games(gog_games: Vec<GogGame>) -> Games {
        let games: Vec<Game> = gog_games
            .into_iter()
            .map(|gog_game| Game::new(
                gog_game.appid.try_into().unwrap(),
                gog_game.title,
                0,
                String::from("Gog"),
            ))
            .collect();

        Games { games }
    }
}
