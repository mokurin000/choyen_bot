// generate_5000choyen(top, bottom, &file).unwrap();

use std::{ path::PathBuf};
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

use choyen_5000::generate_5000choyen;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    std::fs::create_dir_all("temp").unwrap();

    let bot = Bot::from_env();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule="lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "generate a 5000choyen meme. (top|bottom)")]
    Choyen(String),
}

async fn answer(
    bot: Bot,
    message: Message,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Choyen(text) => {
            let unique_id = message.id.0;
            let file = PathBuf::from(&format!("temp/{unique_id}.png"));

            if let Some((top, bottom)) = text.split_once("|") {
                generate_5000choyen(top, bottom, &file).unwrap();
                let input_photo = InputFile::file(file);
                bot.send_photo(message.chat.id, input_photo).await?
            } else {
                bot.send_message(message.chat.id, "usage:\n/choyen [top]|[bottom]")
                    .await?
            }
        }
    };

    Ok(())
}
