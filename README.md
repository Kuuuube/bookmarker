# Bookmarker

Discord bot for bookmarking messages.

## Usage

1. Set up your Discord bot token in one of two ways:

    - Set an env variable named `BOOKMARK_BOT_DISCORD_TOKEN` to your token.

    - Create a file named `.env` in the directory you are running the bookmarker bot from containing `TOKEN=` followed by your token.

2. Run the bot:

    ```
    cargo run --release
    ```

## Bot Usage

### Bookmarking a message

- React to a message with the `:bookmark:` (`🔖`) emoji.

- The bot will send you a DM containing the full contents of the message including any embeds and information about it.

### Deleting a bookmarked message

- React to a bookmarked message in DMs with `:x:` (`❌`).

- The bot will delete the message from your DMs.
