use serenity::all::Context;
use serenity::all::{CreateMessage, Reaction};

use crate::embeds;
use crate::logger;

pub async fn bookmark_message(ctx: &Context, reaction: Reaction) {
    let message = match ctx
        .http
        .get_message(reaction.channel_id, reaction.message_id)
        .await
    {
        Ok(ok) => ok,
        Err(err) => {
            logger::log_error("Failed to get message".to_string(), err.to_string());
            return;
        }
    };

    let userid = match reaction.user_id {
        Some(some) => some,
        None => {
            logger::log_error("Failed to get userid".to_string(), "".to_string());
            return;
        }
    };
    let mut mirrored_embeds = embeds::embeds_into_create_embeds(message.embeds.clone());
    mirrored_embeds.insert(0, embeds::make_info_embed(message));
    let builder = CreateMessage::new().add_embeds(mirrored_embeds);
    match userid.direct_message(&ctx, builder).await {
        Ok(ok) => logger::log(format!("Sent DM message {:?}", ok)),
        Err(err) => {
            logger::log_error("Failed to send DM message".to_string(), err.to_string());
            return;
        }
    };
}

pub async fn delete_bookmark(ctx: &Context, reaction: Reaction) {
    let message = match ctx
        .http
        .get_message(reaction.channel_id, reaction.message_id)
        .await
    {
        Ok(ok) => ok,
        Err(err) => {
            logger::log_error("Failed to get message".to_string(), err.to_string());
            return;
        }
    };

    match message.delete(&ctx).await {
        Ok(_) => logger::log(format!("Deleted DM message {:?}", message)),
        Err(err) => {
            logger::log_error("Failed to delete DM message".to_string(), err.to_string());
            return;
        }
    };
}
