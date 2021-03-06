use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

pub const BOT_ADD_URL: &str = "https://discord.com/api/oauth2/authorize?client_id=475315467792285696&permissions=67226688&scope=bot";
pub const BOT_SOURCE: &str = "https://github.com/joxcat/catboat";

#[command]
async fn addme(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx,
              MessageBuilder::new()
                  .push_line("Ajoute moi à ton serveur en utilisant cette url : ")
                  .push(&BOT_ADD_URL)
                  .build()
    ).await?;

    Ok(())
}

#[command]
#[aliases("sauce")]
async fn source(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, BOT_SOURCE).await?;
    Ok(())
}
