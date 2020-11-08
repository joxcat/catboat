mod commands;
mod error;

use crate::error::CatBoatError;
use human_panic::setup_panic;
use hyper_tls::HttpsConnector;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::{group, help}, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use serenity::framework::standard::{help_commands, CommandResult, CommandGroup, HelpOptions, Args};
use serenity::model::prelude::{UserId, Message};
use serenity::model::gateway::Activity;

use std::{collections::HashSet, env, sync::Arc};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::{general::*, owner::*, public::*};

pub type StandardResult<T> = Result<T, error::CatBoatError>;
pub const BOT_PREFIX: &str = "?";

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct HttpClient;
impl TypeMapKey for HttpClient {
    type Value = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
        ctx.reset_presence().await;
        ctx.set_activity(Activity::competing("être le meilleur boat !")).await;
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(phub, addme, source)]
struct General;

#[group]
#[only_in(guilds)]
#[owners_only]
#[commands(quit)]
struct Owner;

#[help]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() -> StandardResult<()> {
    setup_panic!();

    dotenv::dotenv().expect("Failed to load .env file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| CatBoatError::Unknown)
        .expect("Failed to start the logger");

    // let client = hyper::Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let token = env::var("DISCORD_TOKEN")
        .map_err(|_| CatBoatError::Unknown)
        .expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new().configure(|c| {
        c.owners(owners)
            .on_mention(Some(bot_id))
            .prefix(BOT_PREFIX)
            .case_insensitivity(true)
            .allow_dm(true)
            .no_dm_prefix(true)
    })
    .group(&OWNER_GROUP)
    .group(&GENERAL_GROUP)
    .help(&HELP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .map_err(|_| CatBoatError::Unknown)
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<HttpClient>(
            hyper::Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
        );
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
