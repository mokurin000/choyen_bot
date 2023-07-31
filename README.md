# choyen_bot

Another 5000choyen genertor bot in Rust.

Based on [5000choyen](https://github.com/poly000/5000choyen).

## Run

`CACHE_CHAT_ID` - the chat id that bot could send stickers, to cache stickers for inline queries.
`TELOXIDE_TOKEN` - token from BotFather

## Usage

- `/choyen [arg1]|[arg2]`
- `@choyen_bot arg1|arg2$`, the last `$` will be removed. it's used to avoid unwanted image generation.
