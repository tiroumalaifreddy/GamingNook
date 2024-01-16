use dotenv::dotenv;
use std::env;
use steam_rs::{steam_id::SteamId, Steam};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Get the Steam API Key as an environment variable
    let steam = Steam::new(&std::env::var("STEAM_API_KEY").expect("Missing an API key"));

    // Request the recently played games of SteamID `76561197960434622`
    let steam_id = SteamId::new(76561198118055178);
    let recently_played_games = steam.get_recently_played_games(steam_id, None).await.unwrap();

    // Print the total count of the user's recently played games
    println!("{}", recently_played_games.total_count);
    println!(
        "{:?}",
        steam
            .get_player_summaries(vec![steam_id])
            .await
            .unwrap()
    );
    println!(
        "{:?}",
        steam
            .get_owned_games(steam_id,true,false,12,true,Some(true),"english",true)
            .await
            .unwrap()
    );
}