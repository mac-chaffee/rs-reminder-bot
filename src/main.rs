use std::env;
use std::future::IntoFuture;

use chrono::Datelike;
use chrono::TimeZone;
use twilight_http::Client;
use twilight_model::id::Id;
use tokio::signal;
use tokio::spawn;
use tokio_schedule::{every, Job};
use chrono::{Utc, Weekday};

static WEEKLY: &'static str = concat!(
    ":bulb: Today is Runescape's weekly reset!\n",
    "Some useful activities include:\n",
    "- Buying Necromancy supplies from Thalmund\n",
    "- Playing Herby Werby\n",
    "- Playing Tears of Guthix\n",
    "- And more: https://runescape.wiki/w/Repeatable_events#Weekly_events"
);
static MONTHLY: &'static str = ":bulb: All monthly Distractions & Diversions have reset!\nhttps://runescape.wiki/w/Repeatable_events#Monthly_events";
static WILDERNESS: &'static str = ":bulb: A special Wilderness Flash Event is happening in 5 minutes!\nhttps://runescape.wiki/w/Wilderness_Flash_Events";
static TREASURE_HUNT: &'static str = ":bulb: The weekly clan treasure hunt is happening in 5 minutes! Bring a spade to Edgeville on World 70 to be able to win a rare item!\nhttps://runescape.wiki/w/Treasure_chest_(Carnillean_Rising)";
static PENGUIN_HUNT: &'static str = ":bulb: The weekly clan penguin hunt is happening in 5 minutes! Bring your penguin spy device to Edgeville on World 71!\nhttps://runescape.wiki/w/Penguin_Hide_and_Seek";
static CITADEL_RESET: &'static str = ":bulb: Our clan citadel's weekly reset just happened!\nhttps://runescape.wiki/w/Clan_Citadel";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let weekly = every(1).week()
        .on(Weekday::Mon).at(00, 00, 00)
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
        .on(Weekday::Sat).at(00, 30, 00)
        .in_timezone(&Utc)
        .perform(|| async { send(CITADEL_RESET).await });
    spawn(citadel);

    let monthly = every(1).day()
        .at(00, 00, 00)
        .in_timezone(&Utc)
        .perform(|| async {
            // Make sure this is the first day of the month
            let now = Utc::now();
            if now.day0() == 0 {
                send(MONTHLY).await
            }
        });
    spawn(monthly);

    let wildy = every(1).hour()
        .at(55, 00)
        .perform(|| async {
            // Special events happen on specific rotations since a specific date
            // and there are 14 events per rotation.
            // Must subtract 2 from the wiki numbers due to 1-indexed arrays
            // and the notification triggering 5 min before
            let epoch = Utc.with_ymd_and_hms( 2024, 2, 5, 7, 0, 0).unwrap();
            let rotations_since_epoch = (Utc::now() - epoch).num_hours();
            if [1, 4, 8, 12].contains(&(rotations_since_epoch % 14)) {
                send(WILDERNESS).await
            }
        });
    spawn(wildy);

    let weekly = every(1).week()
        .on(Weekday::Mon).at(00, 00, 00)
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
