// generate_5000choyen(top, bottom, &file).unwrap();

use std::{error::Error, path::PathBuf};
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

use choyen_5000::generate_5000choyen;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "generate a 5000choyen meme. (top|bottom)")]
    Choyen(String),
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Choyen(text) => {
            let unique_id = message.id;
            let file = PathBuf::from(&format!("temp/{unique_id}.png"));

            if let Some((top, bottom)) = text.split_once("|") {
                generate_5000choyen(top, bottom, &file)?;
                let input_photo = InputFile::file(file);
                bot.send_photo(message.chat.id, input_photo).await?
            } else {
                bot.send_message(message.chat.id, "usage:\n/choyen {top}|{bottom}")
                    .await?
            }
        }
    };

    Ok(())
}
