use serenity::all::{CreateMessage, Reaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

mod embeds;
mod logger;
mod token_env;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if !(reaction.emoji.unicode_eq("🔖") || reaction.emoji.unicode_eq("❌")) {
            return;
        }

        let channel = match reaction.channel(&ctx).await {
            Ok(ok) => ok,
            Err(err) => {
                logger::log_error("Failed to get channel".to_string(), err.to_string());
                return;
            }
        };

        match channel {
            serenity::all::Channel::Guild(_) if reaction.emoji.unicode_eq("🔖") => {
                match ctx
                    .http
                    .get_message(reaction.channel_id, reaction.message_id)
                    .await
                {
                    Ok(message) => {
                        let userid = match reaction.user_id {
                            Some(some) => some,
                            None => {
                                logger::log_error(
                                    "Failed to get userid".to_string(),
                                    "".to_string(),
                                );
                                return;
                            }
                        };
                        let mut mirrored_embeds =
                            embeds::embeds_into_create_embeds(message.embeds.clone());
                        mirrored_embeds.insert(0, embeds::make_info_embed(message));
                        let builder = CreateMessage::new().add_embeds(mirrored_embeds);
                        match userid.direct_message(&ctx, builder).await {
                            Ok(ok) => logger::log(format!("Sent DM message {:?}", ok)),
                            Err(err) => {
                                logger::log_error(
                                    "Failed to send DM message".to_string(),
                                    err.to_string(),
                                );
                                return;
                            }
                        };
                    }
                    Err(err) => {
                        logger::log_error("Failed to get message".to_string(), err.to_string());
                        return;
                    }
                };
            }
            serenity::all::Channel::Private(_) if reaction.emoji.unicode_eq("❌") => {
                match ctx
                    .http
                    .get_message(reaction.channel_id, reaction.message_id)
                    .await
                {
                    Ok(message) => {
                        match message.delete(&ctx).await {
                            Ok(_) => logger::log(format!("Deleted DM message {:?}", message)),
                            Err(err) => {
                                logger::log_error(
                                    "Failed to delete DM message".to_string(),
                                    err.to_string(),
                                );
                                return;
                            }
                        };
                    }
                    Err(err) => {
                        logger::log_error("Failed to get message".to_string(), err.to_string());
                        return;
                    }
                };
            }
            _ => (),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        logger::log(format!("{} is connected!", ready.user.name));
    }
}

#[tokio::main]
async fn main() {
    let token = token_env::get_dotenv_token()
        .ok_or_else(|| token_env::get_env_token())
        .expect("Failed to get token from .env or `BOOKMARK_BOT_DISCORD_TOKEN` env variable");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        logger::log(format!("Client error: {why:?}"));
    }
}
