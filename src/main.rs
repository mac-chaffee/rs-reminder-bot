use std::env;
use std::future::IntoFuture;

use chrono::Datelike;
// use chrono::TimeZone;
use twilight_http::Client;
use twilight_model::id::Id;
use tokio::signal;
use tokio::spawn;
use tokio_schedule::{every, Job};
use chrono::{Utc, Weekday, Duration};

const WEEKLY: &'static str = concat!(
    ":bulb: Today is Runescape's weekly reset!\n",
    "Some useful activities include:\n",
    "- Buying Necromancy supplies from Thalmund\n",
    "- Playing Herby Werby\n",
    "- Playing Tears of Guthix\n",
    "- And more: https://runescape.wiki/w/Repeatable_events#Weekly_events"
);
const MONTHLY: &'static str = ":bulb: All monthly Distractions & Diversions will reset in 24 hours! Today's your last chance to do your monthlies!\nhttps://runescape.wiki/w/Repeatable_events#Monthly_events";
const TREASURE_HUNT: &'static str = ":bulb: The weekly clan treasure hunt is happening in 5 minutes! Bring a spade to Edgeville on World 70 to be able to win a rare item!\nhttps://runescape.wiki/w/Treasure_chest_(Carnillean_Rising)";
const PENGUIN_HUNT: &'static str = ":bulb: The weekly clan penguin hunt is happening in 5 minutes! Bring your penguin spy device to Edgeville on World 71!\nhttps://runescape.wiki/w/Penguin_Hide_and_Seek";
const CITADEL_RESET: &'static str = ":bulb: Our clan citadel's weekly reset just happened!\nhttps://runescape.wiki/w/Clan_Citadel";
const RAVEN: &'static str = ":bulb: A raven has spawned somewhere in Prifddinas today. Spot it to unlock a title! More info on spawn locations can be found here: https://runescape.wiki/w/Raven_(Prifddinas)";
const CLAN_MINIGAME_PREFIX: &'static str = ":bulb: Join us for a clan minigame event at 19:00 game time! We'll be in world 65 playing";

// Source: https://runescape.wiki/w/Module:Rotations#L-477
const CLAN_MINIGAME_NAMES: &'static [&'static str] = &[
    "Pest Control",
    "Soul Wars",
    "Fist of Guthix",
    "Barbarian Assault",
    "Conquest",
    "Fishing Trawler",
    "The Great Orb Project",
    "Flash Powder Factory",
    "Castle Wars",
    "Stealing Creation",
    "Cabbage Facepunch Bonanza",
    "Heist",
    "Soul Wars",
    "Barbarian Assault",
    "Conquest",
    "Fist of Guthix",
    "Castle Wars",
    "Pest Control",
    "Soul Wars",
    "Fishing Trawler",
    "The Great Orb Project",
    "Flash Powder Factory",
    "Stealing Creation",
    "Cabbage Facepunch Bonanza",
    "Heist",
    "Trouble Brewing",
    "Castle Wars",
];

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let debug = match env::var("DEBUG") {
        Ok(_) => true,
        Err(_) => false,
    };

    if debug {
        println!("Debug mode active");
        let debug = every(1).minute()
            .perform(|| async { send(WEEKLY).await });
        spawn(debug);
    }

    let weekly = every(1).week()
        .on(Weekday::Wed).at(00, 01, 00)
        .in_timezone(&Utc)
        .perform(|| async { send(WEEKLY).await });
    spawn(weekly);

    let treasure = every(1).week()
        .on(Weekday::Thu).at(00, 25, 00)
        .in_timezone(&Utc)
        .perform(|| async { send(TREASURE_HUNT).await });
    spawn(treasure);

    let penguin = every(1).week()
        .on(Weekday::Fri).at(00, 25, 00)
        .in_timezone(&Utc)
        .perform(|| async { send(PENGUIN_HUNT).await });
    spawn(penguin);

    let citadel = every(1).week()
        .on(Weekday::Sun).at(00, 30, 00)
        .in_timezone(&Utc)
        .perform(|| async { send(CITADEL_RESET).await });
    spawn(citadel);

    let raven = every(1).day()
        .at(00, 10, 00)
        .in_timezone(&Utc)
        .perform(|| async {
            if is_raven_spawned(Utc::now().timestamp()){
                send(RAVEN).await
            }
        });
    spawn(raven);

    let monthly = every(1).day()
        .at(00, 00, 00)
        .in_timezone(&Utc)
        .perform(|| async {
            // Make sure this is the last day of the month
            let now = Utc::now();
            if (now + Duration::days(1)).day0() == 0 {
                send(MONTHLY).await
            }
        });
    spawn(monthly);

    let minigame = every(1).day()
        .at(18, 45, 00)
        .perform(|| async {
            let now = Utc::now();
            if now.weekday() != Weekday::Sun {
                return;
            }
            let index = get_current_minigame(now.timestamp());
            let game = CLAN_MINIGAME_NAMES[index];
            let game_link = format!("https://runescape.wiki/w/{}", game.replace(" ", "_"));
            let msg = format!("{} {}!\n{}", CLAN_MINIGAME_PREFIX, game, game_link);
            send(msg.as_str()).await;
        });
    spawn(minigame);

    let weekly = every(1).week()
        .on(Weekday::Wed).at(00, 01, 00)
        .in_timezone(&Utc)
        .perform(|| async { send(WEEKLY).await });
    spawn(weekly);
    println!("Schedules have been initialized");

    match signal::ctrl_c().await {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        },
    }
}

/// Creates a fresh client and sends a given message
async fn send(message: &str) {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let channel_str = env::var("DISCORD_CHANNEL_ID").unwrap();
    let channel_id = Id::new(
        u64::from_str_radix(channel_str.as_str(), 10).unwrap()
    );

    let client = Client::builder()
        .token(token)
        .build();


    let result = client
        .create_message(channel_id)
        .content(message)
        .unwrap()
        .into_future()
        .await;
    match result {
        Ok(_) => {},
        Err(err) => eprintln!("Failed to send message: {}", err),
    }
}

/// Spotlighted minigames change every 3-days since the Unix Epoch
/// but the rotations require a hardcoded offset to get correct results
fn get_current_minigame(timestamp: i64) -> usize {
    const OFFSET: i64 = 49;
    let rotations_since_epoch = (timestamp / 60 / 60 / 24 - OFFSET) / 3;
    let rotations = usize::try_from(rotations_since_epoch).unwrap();
    let rotation_index = rotations % CLAN_MINIGAME_NAMES.len();
    return rotation_index;
}

/// Ravens spawn every 13 days since the Unix Epoch
/// but the rotations require a hardcoded offset to get correct results
fn is_raven_spawned(timestamp: i64) -> bool {
    const OFFSET: i64 = 7;
    let days_since_epoch = timestamp / 60 / 60 / 24 + OFFSET;
    return days_since_epoch % 13 == 0;
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::{get_current_minigame, is_raven_spawned};

    #[test]
    fn test_get_current_minigame() {
        let cabbage = Utc.with_ymd_and_hms(2024, 4, 30, 6, 0, 0).unwrap();
        assert_eq!(get_current_minigame(cabbage.timestamp()), 10);
        let heist = Utc.with_ymd_and_hms(2024, 5, 3, 6, 0, 0).unwrap();
        assert_eq!(get_current_minigame(heist.timestamp()), 11);
        let soul = Utc.with_ymd_and_hms(2024, 5, 6, 6, 0, 0).unwrap();
        assert_eq!(get_current_minigame(soul.timestamp()), 12);
        let conquest = Utc.with_ymd_and_hms(2024, 5, 12, 19, 0, 0).unwrap();
        assert_eq!(get_current_minigame(conquest.timestamp()), 14);
        // ...
        let castle = Utc.with_ymd_and_hms(2024, 6, 17, 6, 0, 0).unwrap();
        assert_eq!(get_current_minigame(castle.timestamp()), 26);
        let pest = Utc.with_ymd_and_hms(2024, 6, 20, 6, 0, 0).unwrap();
        assert_eq!(get_current_minigame(pest.timestamp()), 0);
    }

        #[test]
    fn test_is_raven_spawned() {
        let first_day = Utc.with_ymd_and_hms(2014, 10, 4, 6, 0, 0).unwrap();
        assert!(is_raven_spawned(first_day.timestamp()));
        let test_day = Utc.with_ymd_and_hms(2024, 5, 1, 6, 0, 0).unwrap();
        assert!(is_raven_spawned(test_day.timestamp()));
        let next_day = Utc.with_ymd_and_hms(2024, 5, 2, 6, 0, 0).unwrap();
        assert!(!is_raven_spawned(next_day.timestamp()));
    }
}
