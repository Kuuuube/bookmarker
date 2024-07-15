use serenity::all::{CreateMessage, Reaction};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

mod embeds;
mod token_env;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if !(reaction.emoji.unicode_eq("🔖") || reaction.emoji.unicode_eq("❌")) {
            return;
        }
        match reaction.channel(&ctx).await.unwrap() {
            serenity::all::Channel::Guild(_) => {
                if reaction.emoji.unicode_eq("🔖") {
                    match ctx
                        .http
                        .get_message(reaction.channel_id, reaction.message_id)
                        .await
                    {
                        Ok(message) => {
                            let mut mirrored_embeds =
                                embeds::embeds_into_create_embeds(message.embeds.clone());
                            mirrored_embeds.insert(0, embeds::make_info_embed(message));
                            let builder = CreateMessage::new().add_embeds(mirrored_embeds);
                            let _ = reaction
                                .user_id
                                .unwrap()
                                .direct_message(&ctx, builder)
                                .await;
                        }
                        Err(err) => println!("{}", err),
                    };
                }
            }
            serenity::all::Channel::Private(_) => {
                if reaction.emoji.unicode_eq("❌") {
                    match ctx
                        .http
                        .get_message(reaction.channel_id, reaction.message_id)
                        .await
                    {
                        Ok(ok) => {
                            let _ = ok.delete(&ctx).await;
                        }
                        Err(err) => println!("{}", err),
                    };
                }
            }
            _ => (),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
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
        println!("Client error: {why:?}");
    }
}
