use crate::steam::steamgames::SteamGame;
use crate::gog::goggames::GogGame;
use egs_api::api::types::library::Record;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Game {
    pub userid: String,
    pub appid: String,
    pub name: String,
    pub playtime: u64,
    pub platform: String
}

#[derive(Debug)]
pub struct Games {
    pub games: Vec<Game>
}

impl Game {
    fn new(userid: String, appid: String, name: String, playtime: u64, platform: String) -> Game {
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

    pub fn remove_duplicates(&mut self) {
        let mut seen = HashSet::new();
        self.games.retain(|game| seen.insert(game.appid.clone()));
    }

    pub fn from_steam_games(steam_games: Vec<SteamGame>, userid: String) -> Games {
        let games: Vec<Game> = steam_games
            .into_iter()
            .map(|steam_game| Game::new(
                userid.clone(),
                steam_game.appid.to_string(),
                steam_game.name,
                steam_game.playtime_forever,
                String::from("Steam"),
            ))
            .collect();

        Games { games }
    }


    pub fn from_gog_games(gog_games: Vec<GogGame>, userid: String) -> Games {
        let games: Vec<Game> = gog_games
            .into_iter()
            .map(|gog_game| Game::new(
                userid.clone(),
                gog_game.appid.to_string(),
                gog_game.title,
                0,
                String::from("Gog"),
            ))
            .collect();

        Games { games }
    }

    pub fn from_epic_games(epic_games: Vec<Record>, userid: String) -> Games {
        let games: Vec<Game> = epic_games
            .into_iter()
            .map(|epic_game| {
                Game::new(
                    userid.clone(),
                    epic_game.product_id,
                    epic_game.sandbox_name,
                    0,
                    String::from("Epic"),
                )
            })
            .collect();

        Games { games }
    }
}
