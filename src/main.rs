use teloxide::prelude::*;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    let bot_token = env!("BOT_TOKEN");
    let bot = Bot::new(bot_token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let bot_username = env!("BOT_USERNAME");
        if msg.text().unwrap().contains(bot_username)
            && (msg.chat.is_group() || msg.chat.is_supergroup())
        {
            let message = format!("{}\n\ncc {}", env!("BOT_MESSAGE"), env!("BOT_MEMBERS"));
            bot.send_message(msg.chat.id, message).await?;
        }
        Ok(())
    })
    .await;
}
