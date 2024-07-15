use serenity::all::{CreateEmbed, Embed};

pub fn embeds_into_create_embeds(embeds: Vec<Embed>) -> Vec<CreateEmbed> {
    let mut create_embeds: Vec<CreateEmbed> = vec![];
    for embed in embeds {
        create_embeds.push(embed_into_create_embed(embed));
    }
    return create_embeds;
}

pub fn embed_into_create_embed(embed: Embed) -> CreateEmbed {
    let mut create_embed = CreateEmbed::new();
    for field in embed.fields {
        create_embed = create_embed.field(field.name, field.value, field.inline);
    }
    embed.author.and_then(|author| {
        create_embed = create_embed.clone().author(author.into());
        Some(())
    });
    embed.colour.and_then(|colour| {
        create_embed = create_embed.clone().colour(colour);
        Some(())
    });
    embed.description.and_then(|description| {
        create_embed = create_embed.clone().description(description);
        Some(())
    });
    embed.footer.and_then(|footer| {
        create_embed = create_embed.clone().footer(footer.into());
        Some(())
    });
    embed.image.and_then(|image| {
        create_embed = create_embed.clone().image(image.url);
        Some(())
    });
    embed.thumbnail.and_then(|thumbnail| {
        create_embed = create_embed.clone().thumbnail(thumbnail.url);
        Some(())
    });
    embed.timestamp.and_then(|timestamp| {
        create_embed = create_embed.clone().timestamp(timestamp);
        Some(())
    });
    embed.title.and_then(|title| {
        create_embed = create_embed.clone().title(title);
        Some(())
    });
    embed.url.and_then(|url| {
        create_embed = create_embed.clone().url(url);
        Some(())
    });
    return create_embed;
}
