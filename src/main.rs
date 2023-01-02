// generate_5000choyen(top, bottom, &file).unwrap();

use std::path::PathBuf;

use teloxide::{
    prelude::*,
    types::{InputFile, MessageKind, MediaKind},
    types::{
        InlineQuery, InlineQueryResult, InlineQueryResultArticle, InlineQueryResultCachedPhoto,
        InputMessageContent, InputMessageContentText,
    },
    utils::command::BotCommands,
};

use choyen_5000::generate_5000choyen;

const PRAVITE_CHANNEL_ID: &'static str = "-1001805077818";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    std::fs::create_dir_all("temp").unwrap();

    let bot = Bot::from_env();

    let inline_handler = Update::filter_inline_query().branch(dptree::endpoint(
        |bot: Bot, q: InlineQuery| async move {
            let results = if let Some((top, bottom)) = q.query.split_once("|") {
                let unique_id = &q.id;
                let file = PathBuf::from(&format!("temp/{unique_id}.webp"));
                generate_5000choyen(top, bottom, &file).unwrap();
                let input_photo = InputFile::file(file);

                let upload_photo = bot.send_photo(PRAVITE_CHANNEL_ID.to_owned(), input_photo).send().await;
                match upload_photo {
                    Ok(resp) => {
                        let mut results = Vec::new();
                        let kind = resp.kind;
                        if let MessageKind::Common(common) = kind {
                            if let MediaKind::Photo(photo) = common.media_kind {
                                let file_id = &photo.photo[0].file.id;
                                let cached_photo = InlineQueryResultCachedPhoto::new("0", file_id);
                               results.push(InlineQueryResult::CachedPhoto(cached_photo));
                            }
                        }
                        results
                    },
                    Err(err) => {
                        log::error!("Error in hanlder: {:?}", err);
                        vec![]
                    }
                }
            } else {
                let content = InputMessageContent::Text(InputMessageContentText::new(
                    "usage:\n/choyen [top]|[bottom]",
                ));
                let article =
                    InlineQueryResultArticle::new("0", "usage:\n/choyen [top]|[bottom]", content);
                vec![InlineQueryResult::Article(article)]
            };

            let response = bot.answer_inline_query(&q.id, results).send().await;
            if let Err(err) = response {
                log::error!("Error in handler: {:?}", err);
            }
            respond(())
        },
    ));

    Dispatcher::builder(bot.clone(), inline_handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "generate a 5000choyen meme. (top|bottom)")]
    Choyen(String),
}

async fn answer(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Choyen(text) => {
            let unique_id = message.id.0;
            let file = PathBuf::from(&format!("temp/{unique_id}.webp"));

            if let Some((top, bottom)) = text.split_once("|") {
                generate_5000choyen(top, bottom, &file).unwrap();
                let input_photo = InputFile::file(file);
                bot.send_animation(message.chat.id, input_photo).await?
            } else {
                bot.send_message(message.chat.id, "usage:\n/choyen [top]|[bottom]")
                    .await?
            }
        }
    };

    Ok(())
}
