// Librería para los Strings
use std::str::FromStr;

// Librería para manejar las variables de entorno
use dotenv::dotenv;

// Librería para manejar las el tiempo
use chrono::Duration;

// Librería para manejar el Bot
use teloxide::{prelude::*, types::ChatPermissions, utils::command::BotCommands};
use teloxide::adaptors::DefaultParseMode;
use teloxide::types::{InputFile};
use teloxide_core::types::ParseMode::MarkdownV2;

type MyBot = DefaultParseMode<Bot>;

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
description = "Hola, soy un Bot que administra grupos de Telegram y seré tu asistente personal en tu aprendizaje de Rust, El Lenguaje de Programación\\. \n\nEstos son los comandos disponibles:",
parse_with = "split"
)]

// Los comandos disponibles.
enum Command {
    #[command(description = "Banea a un usuario del chat\\. \n\nUso: /ban respondiendo un mensaje de un usuario\\. \n\n")]
    Ban,
    #[command(description = "Desbanea a un usuario del chat\\. \n\nUso: /unban respondiendo un mensaje de un usuario\\. \n\n")]
    Unban,
    #[command(description = "Silencia a un usuario del chat\\. \n\nUso: /mute respondiendo un mensaje de un usuario\\. \n\n")]
    Mute {
        time: u64,
        unit: UnitOfTime,
    },
    #[command(description = "Mensaje de inicio del Bot\\. \n")]
    Start,
    #[command(description = "Explica el uso de variables en Rust\\. \n")]
    Variables,
    #[command(description = "Explica el uso de constantes en Rust\\. \n")]
    Constantes,
    #[command(description = "Explica los Tipos de Datos en Rust\\. \n")]
    TiposDeDatos,
    #[command(description = "Explica el uso de los Operadores en Rust\\. \n")]
    Operadores,
    #[command(description = "Explica el uso de Arreglos/Arrays en Rust\\. \n")]
    Arrays,
    #[command(description = "Explica el uso de tuplas en Rust\\. \n")]
    Tuplas,
    #[command(description = "Explica el uso de vectores en Rust\\. \n")]
    Vectores,
    #[command(description = "Explica el uso de condicionales en Rust\\. \n")]
    Condicionales,
    #[command(description = "Explica el uso del ciclo loop en Rust\\. \n")]
    Loop,
    #[command(description = "Explica el uso del ciclo For en Rust\\. \n")]
    For,
    #[command(description = "Explica el uso del ciclo While en Rust\\. \n")]
    While,
    #[command(description = "Explica el uso de Match en Rust\\. \n")]
    Match,
    #[command(description = "Explica el uso de los Enums en Rust\\. \n")]
    Enums,
    #[command(description = "Explica el uso de Funciones en Rust\\. \n")]
    Funciones,
    #[command(description = "Explica el uso de Return en Rust\\. \n")]
    Return,
    #[command(description = "Explica el uso de Métodos en Rust\\. \n")]
    Metodos,
    #[command(description = "Envía una Imagen\\. \n")]
    Image,
    #[command(description = "Envía un Video\\. \n")]
    Video,
    #[command(description = "Envía un Gif\\. \n")]
    Gif,
    #[command(description = "Envía este mensaje\\. \n")]
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

    let bot = teloxide::Bot::from_env().parse_mode(MarkdownV2);

    Command::repl(bot, action).await;
}

// Función de acción para cada comando.
async fn action(bot: MyBot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "`Hola, soy un Bot de Administración`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Variables => {
            bot.send_message(msg.chat.id, "Variables: Son un espacio en memoria cuyo valor puede asignarse y cambiar \n\nEjemplo en Rust: \n`let mi_variable = valor:`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Constantes => {
            bot.send_message(msg.chat.id, "Constantes: Son una variable de solo lectura, su valor no puede cambiarse durante la ejecución del programa").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::TiposDeDatos => {
            bot.send_message(msg.chat.id, "Tipos de Datos: Las variables se definen con un tipo de dato que puede ser: \n\nUn número entero \nUn número Flotante/Decimal \nUn numero negativo \nUn String/Cadena (Palabra o letra), etc \n\nEjemplo en Rust: \ni8,i16,i32,i64,i128 = Tipo Entero \nu8,u16,u32,u64,u128 = Tipo Entero (Solo números positivos)").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Operadores => {
            bot.send_message(msg.chat.id, "Operadores: En programación, tenemos distintos tipos de operadores para manejar datos en nuestras variables\\. Entre estos están: \n\n`//Los Operadores Básicos: \n\n+ Suma \n\n- Resta \n\n* Multiplicación \n\n/ Division \n\n% División (Con resto/residuo) \n\n//Los Operadores Relacionales: \n\n> Mayor que \n\n< Menor que \n\n>= Mayor o igual que \n\n<= Menor o igual que \n\n== Igual \n\n!= Diferente de \n\n//Los Operadores Lógicos \n\n|| Or (o) \n\n&& And (y)`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Arrays => {
            bot.send_message(msg.chat.id, "un Arreglo/Array nos permite almacenar múltiples valores dentro de una colección\\. En Rust, un Arreglo/Array debe almacenar el mismo tipo de dato \\(Solo enteros, Solo Strings, Solo Booleanos, etc\\) \n\nEjemplo en Rust: \n`let array = [1, 2, 3, 4, 5];` \n\nConsejo: En Rust, los Arreglos/Arrays se rigen por la regla de los indices\\. A cada elemento le corresponde un indice y los indices comienzan en cero\\. Si tomamos nuestro ejemplo el índice en dicho ejemplo es: \n\n`0 -> 1 \n1 -> 2 \n2 -> 3 \n3 -> 4 \n4 -> 5`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Tuplas => {
            bot.send_message(msg.chat.id, "Al igual que con los Arreglos/Arrays las tuplas nos permiten almacenar diferentes elementos dentro de una colección \n\nEjemplo en Rust: \n`let tupla = (10, false, 5.5);` \n\nConsejo: A diferencia entre una tupla y un Arreglo/Array consiste en que las tuplas pueden almacenar diferentes tipos de datos, cosa que con los Arreglos/Arrays no es posible\\.").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Vectores => {
            bot.send_message(msg.chat.id, "Al igual que un Arreglo/Array los Vectores nos permiten almacenar diferentes valores dentro de una colección\\. \n\nEjemplo en Rust: \n`let mut vector = vec![1, 2, 3];`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Condicionales => {
            bot.send_message(msg.chat.id, "Son grupos de sentencias o sentencias individuales que te permiten condicionar la decisión entre la elección de una opción y otra\\. \n\nEjemplo en Rust: \n`let color = 'verde'; \n\nif color == 'Verde' {\n   println!('Puede continuar.'); \n} else { \n   println!('El color no es verde'); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Loop => {
            bot.send_message(msg.chat.id, "Es un Bucle infinito").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::For => {
            bot.send_message(msg.chat.id, "En Rust, el ciclo for nos permitirá iterar sobre una colección de datos\\. Ya sea un vector, un Arreglo/Array, una tupla, etc\\. El ciclo for funcionara como un for each \n\nEjemplo en Rust: \n`let numeros : [i32; 5] = [1, 2, 3, 4, 5]; \n\nfor numero in numeros.iter( ) {\n    println!('El valor de número: {:?}', numero); \n}` \n\nEjemplo de algoritmo Fizz Buzz utilizando el ciclo for en Rust: \n\n`for numero in 1..101 {\n\n    if numero % 3 == 0 && numero % 5 == 0 {\n    println!('Fizz Buzz'); \n\n} else if numero % 3 == 0 { \n    println!('Fizz'); \n\n} else if numero % 5 == 0 {\n    println!('Buzz'); \n\n} else {\n    println!('{}', numero); \n   }\n}` \n\nNota: Debido a las limitaciones de Teloxide, no es posible utilizar comillas dobles más de una vez al enviar un mensaje a través del Bot, se sugiere reemplazar las comillas simples del ejemplo por comillas dobles al practicar").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::While => {
            bot.send_message(msg.chat.id, "Próximamente").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Match => {
            bot.send_message(msg.chat.id, "En Rust, Match es el equivalente a Switch en otros lenguajes\\. Con Match podemos evaluar un valor en diferentes casos\\. \n\nEjemplo en Rust: \n`let numero : i32 = 55; \n\nmatch numero {\n    1 => println!('El número es uno.'), \n\n    2 => println!('El número es dos.'), \n\n    3 => println!('El número es tres.'), \n\n    4 | 5 | 6 => println!('El numero está entre cuatro y seis.'), \n\n    7..=100 => {\n    println!('El número es mayor o igual a 7'); \n    println!('El número se evalúa mediante un rango.'); \n    }, \n\n    _ => println!('{}', numero) \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Enums => {
            bot.send_message(msg.chat.id, "Un Enum es un tipo que almacena diferentes variantes, almacena diferentes opciones\\. \n\nEjemplo en Rust: \n`enum Response {\n    Sucess, // Se completó correctamente \n    Error(u32, String), // Podemos indicar un código de Error a través de una Tupla \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Funciones => {
            bot.send_message(msg.chat.id, "Las funciones son bloques de código que se pueden reutilizar en diferentes partes de nuestro programa\\. \n\nEjemplo en Rust: \n`fn saludar(nombre: &str) {\n   println!('Hola {}', nombre); \n} \n\nfn main() {\n   saludar('Juan'); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Metodos => {
            bot.send_message(msg.chat.id, "Los métodos son similares a las funciones, pero se diferencian en que los métodos se definen dentro de un contexto, como una estructura o un Enum\\. \n\nEjemplo en Rust: \n`struct Rectángulo { \n   ancho: u32, \n   alto: u32, \n} \n\nimpl Rectángulo { \n   fn area(&self) -> u32 { \n      self.ancho * self.alto \n      } \n} \n\nfn main() { \n   let rectangulo = Rectangulo { \n     ancho: 30, \n     alto: 50, \n   }; \n\n   println!('El área del rectángulo es: {}', \n   rectángulo.area()); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Return => {
            bot.send_message(msg.chat.id, "Próximamente").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Image => {
            bot.send_photo(msg.chat.id, InputFile::file("./assets/img/comprobar.png")).await?;
            // Usamos InputFile::file para enviar un archivo local y asignamos la ruta con (./ruta/de/la/imagen.jpg)).await?;
            // Nota: crear la carpeta de assets en la ubicación raíz del proyecto
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Video => {
            bot.send_video(msg.chat.id, InputFile::file("./assets/video/report.mp4")).await?;
            // Usamos InputFile::file para enviar un archivo local y asignamos la ruta con (./ruta/de/la/video.mp4)).await?;
            // Nota: crear la carpeta de assets en la ubicación raíz del proyecto
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Gif => {
            bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/ban.gif")).await?;
            // Usamos InputFile::file para enviar un archivo local y asignamos la ruta con (./ruta/de/la/audio.mp3)).await?;
            // Nota: crear la carpeta de assets en la ubicación raíz del proyecto
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
        Command::Ban => ban_user(bot, msg).await?,
        Command::Unban => unban_user(bot, msg).await?,
        Command::Mute { time, unit } => mute_user(bot, msg, calc_restrict_time(time, unit)).await?,
    };

    Ok(())
}

// Banear a un usuario con mensaje respondido.
async fn ban_user(bot: MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.ban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
            bot.send_message(msg.chat.id, "Usuario Baneado\\!").await?;
            bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/ban.gif")).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }
    }
    Ok(())
}

async fn unban_user(bot: MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.unban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
            bot.send_message(msg.chat.id, "Usuario Desbaneado\\!").await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }
    }
    Ok(())
}

// Silenciar a un usuario con un mensaje respondido.
async fn mute_user(bot: MyBot, msg: Message, time: Duration) -> ResponseResult<()> {
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