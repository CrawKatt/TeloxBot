// Librería para los Strings
use std::str::FromStr;

// Librería para manejar las variables de entorno
use dotenv::dotenv;

// Librería para manejar las el tiempo
use chrono::Duration;

// Librería para manejar el Bot
use teloxide::{prelude::*, types::ChatPermissions, utils::command::BotCommands};

// Derive BotCommands para analizar texto con un comando en esta enumeración.
//
// 1. `rename_rule = "lowercase"` convierte todos los comandos en letras minúsculas.
// 2. `description = "..."` especifica un texto antes de todos los comandos.
//
// Es decir, puede simplemente llamar a Command::descriptions() para obtener una descripción de
// sus comandos en este formato:
// %GENERAL-DESCRIPTION% /// %DESCRIPCIÓN GENERAL%
// %PREFIX%%COMMAND% - %DESCRIPTION% /// %PREFIJO%%COMANDO% - %DESCRIPCIÓN%
#[derive(BotCommands, Clone)]
#[command(
rename_rule = "lowercase",
description = "Estos son los comandos disponibles:",
parse_with = "split"
)]
// Los comandos disponibles.
enum Command {
    #[command(description = "Expulsa a un usuario del chat (puede volver a unirse con un enlace de invitación).\n\nUso: /kick respondiendo un mensaje de un usuario. \n\n")]
    Kick,
    #[command(description = "Banea a un usuario del chat. \n\nUso: /ban respondiendo un mensaje de un usuario. \n\n")]
    Ban,
    #[command(description = "Silencia a un usuario del chat. \n\nUso: /mute respondiendo un mensaje de un usuario. \n\n")]
    Mute {
        time: u64,
        unit: UnitOfTime,
    },
    #[command(description = "Mensaje de inicio del Bot. \n")]
    Start,
    #[command(description = "Explica el uso de variables en Rust. \n")]
    Variables,
    #[command(description = "Explica el uso de constantes en Rust. \n")]
    Constantes,
    #[command(description = "Explica los Tipos de Datos en Rust. \n")]
    TiposDeDatos,
    #[command(description = "Explica el uso de los Operadores en Rust. \n")]
    Operadores,
    #[command(description = "Explica el uso de Arreglos/Arrays en Rust. \n")]
    Arrays,
    #[command(description = "Explica el uso de tuplas en Rust. \n")]
    Tuplas,
    #[command(description = "Explica el uso de vectores en Rust. \n")]
    Vectores,
    #[command(description = "Explica el uso de condicionales en Rust. \n")]
    Condicionales,
    #[command(description = "Explica el uso del ciclo loop en Rust. \n")]
    Loop,
    #[command(description = "Explica el uso del ciclo For en Rust. \n")]
    For,
    #[command(description = "Explica el uso del ciclo While en Rust. \n")]
    While,
    #[command(description = "Envía este mensaje \n")]
    Help,
}
// Unidad de tiempo para el comando mute.
#[derive(Clone)]
enum UnitOfTime {
    Seconds,
    Minutes,
    Hours,
}
// Implementación de FromStr para UnitOfTime
impl FromStr for UnitOfTime {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "h" | "hours" => Ok(UnitOfTime::Hours),
            "m" | "minutes" => Ok(UnitOfTime::Minutes),
            "s" | "seconds" => Ok(UnitOfTime::Seconds),
            _ => Err("Unidades Permitidas: h, m, s"),
        }
    }
}
// Función principal para el inicio del Bot mediante una Variable de Entorno.
async fn run() {
    dotenv().ok();
}

// Función principal que inicia el Bot
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    run().await;
    log::info!("Starting admin bot...");

    let bot = teloxide::Bot::from_env();

    Command::repl(bot, action).await;
}

// Función de acción para cada comando.
async fn action(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, "Hola, soy un Bot que administra grupos de Telegram y seré tu asistente personal en tu aprendizaje de Rust, El Lenguaje de Programación").await?;
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Hola, soy un Bot de Administración.").await?;
        }
        Command::Variables => {
            bot.send_message(msg.chat.id, "*Variables*: Son un espacio en memoria cuyo valor puede asignarse y cambiar. \n\nEjemplo en Rust: \nlet mi_variable = valor").await?;
        }
        Command::Constantes => {
            bot.send_message(msg.chat.id, "Constantes: Son una variable de solo lectura, su valor no puede cambiarse durante la ejecución del programa").await?;
        }
        Command::TiposDeDatos => {
            bot.send_message(msg.chat.id, "Tipos de Datos: Las variables se definen con un tipo de dato que puede ser: \n\nUn número entero \nUn número Flotante/Decimal \nUn numero negativo \nUn String/Cadena (Palabra o letra), etc \n\nEjemplo en Rust: \ni8,i16,i32,i64,i128 = Tipo Entero \nu8,u16,u32,u64,u128 = Tipo Entero (Solo números positivos)").await?;
        }
        Command::Operadores => {
            bot.send_message(msg.chat.id, "Operadores: En programación, tenemos distintos tipos de operadores para manejar datos en nuestras variables. Entre estos están: \n\n//Los Operadores Básicos: \n\n+ Suma \n\n- Resta \n\n* Multiplicación \n\n/ Division \n\n% División (Con resto/residuo) \n\n//Los Operadores Relacionales: \n\n> Mayor que \n\n< Menor que \n\n>= Mayor o igual que \n\n<= Menor o igual que \n\n== Igual \n\n!= Diferente de \n\n//Los Operadores Lógicos \n\n|| Or (o) \n\n&& And (y)").await?;
        }
        Command::Arrays => {
            bot.send_message(msg.chat.id, "Son un conjunto de variables").await?;
        }
        Command::Tuplas => {
            bot.send_message(msg.chat.id, "Son un conjunto de variables similar a un Arreglo/Array").await?;
        }
        Command::Vectores => {
            bot.send_message(msg.chat.id, "Son un conjunto de variables").await?;
        }
        Command::Condicionales => {
            bot.send_message(msg.chat.id, "Son un conjunto de variables").await?;
        }
        Command::Loop => {
            bot.send_message(msg.chat.id, "Es un Bucle infinito").await?;
        }
        Command::For => {
            bot.send_message(msg.chat.id, "Son un conjunto de variables").await?;
        }
        Command::While => {
            bot.send_message(msg.chat.id, "Son un conjunto de variables").await?;
        }
        Command::Kick => kick_user(bot, msg).await?,
        Command::Ban => ban_user(bot, msg).await?,
        Command::Mute { time, unit } => mute_user(bot, msg, calc_restrict_time(time, unit)).await?,
    };

    Ok(())
}

// Expulsar a un usuario con un mensaje respondido.
async fn kick_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            // bot.unban_chat_member también puede expulsar a un usuario de un chat grupal.
            bot.unban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
            bot.send_message(msg.chat.id, "Usuario Expulsado!").await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }
    }
    Ok(())
}

// Banear a un usuario con mensaje respondido.
async fn ban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            // bot.unban_chat_member también puede expulsar a un usuario de un chat grupal.
            bot.ban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
            bot.send_message(msg.chat.id, "Usuario Baneado!").await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }
    }
    Ok(())
}


// Silenciar a un usuario con un mensaje respondido.
async fn mute_user(bot: Bot, msg: Message, time: Duration) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            bot.restrict_chat_member(
                msg.chat.id,
                replied.from().expect("Must be MessageKind::Common").id,
                ChatPermissions::empty(),
            )
                .until_date(msg.date + time)
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje!")
                .await?;
        }
    }
    Ok(())
}

// Calcula el tiempo de restricción de usuarios.
fn calc_restrict_time(time: u64, unit: UnitOfTime) -> Duration {
    match unit {
        UnitOfTime::Hours => Duration::hours(time as i64),
        UnitOfTime::Minutes => Duration::minutes(time as i64),
        UnitOfTime::Seconds => Duration::seconds(time as i64),
    }
}