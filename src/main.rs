use crate::commands::*;
pub mod database;
pub mod commands;
pub mod utils;
pub mod buttons;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Iniciando Bot...");
    dotenv().ok();
    conectar().await.expect("Error al conectar con la Base de Datos");
    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));
    Dispatcher::builder(bot.clone(), handler).enable_ctrlc_handler().build().dispatch().await;
    Command::repl(bot,action).await;
    Ok(())
}

