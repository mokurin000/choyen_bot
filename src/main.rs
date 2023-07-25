// generate_5000choyen(top, bottom, &file).unwrap();

use std::{path::PathBuf, sync::OnceLock};

use teloxide::{
    dispatching::DpHandlerDescription,
    prelude::*,
    types::{
        InlineQuery, InlineQueryResult, InlineQueryResultArticle, InlineQueryResultCachedSticker,
        InputMessageContent, InputMessageContentText,
    },
    types::{InputFile, MediaKind, MessageKind},
    utils::command::BotCommands,
    RequestError,
};

use anyhow::Result;

use choyen_5000::generate_5000choyen;

const PRAVITE_CHANNEL_ID: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    std::fs::create_dir_all("temp").unwrap();
    let bot = Bot::from_env();

    let inline_handler = inline_handler();
    let command_handler = command_handler();

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(inline_handler)
            .branch(command_handler),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
    Ok(())
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

fn inline_handler() -> teloxide::prelude::Handler<
    'static,
    teloxide::prelude::DependencyMap,
    Result<(), RequestError>,
    DpHandlerDescription,
> {
    Update::filter_inline_query().branch(dptree::endpoint(|bot: Bot, q: InlineQuery| async move {
        let splitted = q.query.split_once("|");
        let results = if splitted.is_some()
            && unsafe { splitted.unwrap_unchecked() }.1.ends_with("$")
        {
            let unique_id = &q.id;
            let file = PathBuf::from(&format!("temp/{unique_id}.webp"));

            let (top, bottom) = unsafe { splitted.unwrap_unchecked() };

            generate_5000choyen(top, bottom.trim_end_matches("$"), &file).unwrap();
            let input_photo = InputFile::file(file);

            let upload_photo = bot
                .send_sticker(
                    PRAVITE_CHANNEL_ID
                        .get_or_init(|| std::env::var("CHOYEN_CHANNEL_ID").unwrap())
                        .clone(),
                    input_photo,
                )
                .send()
                .await;
            match upload_photo {
                Ok(resp) => {
                    let mut results = Vec::new();
                    let kind = resp.kind;
                    if let MessageKind::Common(common) = kind {
                        if let MediaKind::Sticker(sticker) = common.media_kind {
                            let sticker_file_id = &sticker.sticker.file.id;
                            let cached_sticker = InlineQueryResultCachedSticker::new("0", sticker_file_id);
                            results.push(InlineQueryResult::CachedSticker(cached_sticker));
                        }
                    }
                    results
                }
                Err(err) => {
                    log::error!("Error in hanlder: {:?}", err);
                    vec![]
                }
            }
        } else {
            let content = InputMessageContent::Text(InputMessageContentText::new(
                "usage:\n@choyen_bot [top]|[bottom]$",
            ));
            let article =
                InlineQueryResultArticle::new("0", "usage:\n@choyen_bot [top]|[bottom]$", content);
            vec![InlineQueryResult::Article(article)]
        };

        let response = bot.answer_inline_query(&q.id, results).send().await;
        if let Err(err) = response {
            log::error!("Error in handler: {:?}", err);
        }
        respond(())
    }))
}

fn command_handler() -> teloxide::prelude::Handler<
    'static,
    teloxide::prelude::DependencyMap,
    Result<(), RequestError>,
    DpHandlerDescription,
> {
    Update::filter_message().branch(dptree::entry().filter_command::<Command>().endpoint(answer))
}

async fn answer(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Choyen(text) => {
            let msg_id = message.id.0;
            let chat_id = message.chat.id.0;
            let file = PathBuf::from(&format!("temp/{chat_id}_{msg_id}.webp"));

            if let Some((top, bottom)) = text.split_once("|") {
                generate_5000choyen(top, bottom, &file).unwrap();
                let input_photo = InputFile::file(file);
                bot.send_animation(message.chat.id, input_photo)
                    .reply_to_message_id(message.id)
                    .await?
            } else {
                bot.send_message(message.chat.id, "usage:\n/choyen [top]|[bottom]")
                    .reply_to_message_id(message.id)
                    .await?
            }
        }
    };

    Ok(())
}
