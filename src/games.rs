struct Game {
    appid: u64,
    name: String,
    playtime: u64,
    platform: String
}

struct Games {
    games: Vec<Game>
}

impl Game {
    fn value(&self) -> &Game {
        &self.game
    }

    fn new(appid: u64, name: String, playtime: u64, platform: String) -> Game {
        Games {appid, name, playtime, platform}
    }

    fn add_percentage_achievment(&self) -> Game {
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
}