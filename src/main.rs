// Librería para los Strings
use std::str::FromStr;

// Librería para manejar las variables de entorno
use dotenv::dotenv;

// Librería para manejar el tiempo
//use chrono::Duration;

// Librería para manejar el Bot
use teloxide::{prelude::*, types::ChatPermissions, utils::command::BotCommands};
use teloxide::adaptors::DefaultParseMode;
use teloxide::types::{InputFile};
use teloxide_core::types::ParseMode::MarkdownV2;

// Librería para aleatorizar imágenes
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

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
    Bann,
    #[command(description = "Desbanea a un usuario del chat\\. \n\nUso: /unban respondiendo un mensaje de un usuario\\. \n\n")]
    Unbann,
    #[command(description = "Silencia a un usuario del chat\\. \n\nUso: /mute respondiendo un mensaje de un usuario\\. \n\n")]
    Mute,
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
    #[command(description = "Explica el uso de Closures en Rust\\. \n")]
    Closures,
    #[command(description = "Explica el uso de Structs en Rust\\. \n")]
    Structs,
    #[command(description = "Explica el uso de Traits en Rust\\. \n")]
    Traits,
    #[command(description = "Explica el uso de Option en Rust\\. \n")]
    Option,
    #[command(description = "Explica el uso de Result en Rust\\. \n")]
    Result,
    #[command(description = "Explica el uso de Generics en Rust\\. \n")]
    Generics,
    #[command(description = "Explica el uso de Lifetimes en Rust\\. \n")]
    Lifetimes,
    #[command(description = "Explica el uso de Macros en Rust\\. \n")]
    Macros,
    #[command(description = "Explica el uso de Ownership en Rust\\. \n")]
    Ownership,
    #[command(description = "Explica el uso de Referencias en Rust\\. \n")]
    Referencias,
    #[command(description = "Explica el uso de Borrowing en Rust\\. \n")]
    Borrowing,
    #[command(description = "Explica el uso de los Módulos en Rust\\. \n")]
    Modulos,
    #[command(description = "Explica el uso de los Crates en Rust\\. \n")]
    Crates,
    #[command(description = "Explica el Shadowing en Rust\\. \n")]
    Shadowing,
    #[command(description = "Explica el uso de los Slices en Rust\\. \n")]
    Slices,
    #[command(description = "Explica el uso de los Strings en Rust\\. \n")]
    Strings,
    #[command(description = "Explica el uso de los Iterators en Rust\\. \n")]
    Iterators,
    #[command(description = "Explica los Scopes en Rust\\. \n")]
    Scopes,
    #[command(description = "Explica el uso de rand en Rust\\. \n")]
    Random,
    #[command(description = "Explica el uso de los Chrono en Rust\\. \n")]
    Chrono,
    #[command(description = "Explica el uso de los Async en Rust\\. \n")]
    Async,
    #[command(description = "Explica el uso de los Botones de Teloxide en Rust\\. \n")]
    Menu,
    #[command(description = "Explica el uso de los Inline de Teloxide en Rust\\. \n")]
    Inline,
    #[command(description = "Envía una Imagen\\. \n")]
    Image,
    #[command(description = "Envía un Video\\. \n")]
    Video,
    #[command(description = "Envía un Gif\\. \n")]
    Pat,
    #[command(description = "Envía un Meme de Programación\\. \n")]
    Meme,
    #[command(description = "Envía este mensaje\\. \n")]
    Help,
    #[command(description = "Acerca de este Bot\\. \n")]
    About,
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

    let bot = Bot::from_env().parse_mode(MarkdownV2);

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
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
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
            bot.send_message(msg.chat.id, "Tipos de Datos: Las variables se definen con un tipo de dato que puede ser: \n\nUn número entero \nUn número Flotante/Decimal \nUn numero negativo \nUn String/Cadena \\(Palabra o letra\\), etc \n\nEjemplo en Rust: \ni8,i16,i32,i64,i128 \\= Tipo Entero \nu8,u16,u32,u64,u128 \\= Tipo Entero \\(Solo números positivos\\)").await?;
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
            bot.send_message(msg.chat.id, "El bucle es una estructura de control que nos permite repetir un bloque de código tantas veces como sea necesario\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut contador = 0; \n\n   loop { \n      contador += 1; \n\n      if contador == 10 { \n         break; \n      } \n   } \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::For => {
            bot.send_message(msg.chat.id, "En Rust, el ciclo for nos permitirá iterar sobre una colección de datos\\. Ya sea un vector, un Arreglo/Array, una tupla, etc\\. El ciclo for funcionara como un for each \n\nEjemplo en Rust: \n`let numeros : [i32; 5] = [1, 2, 3, 4, 5]; \n\nfor numero in numeros.iter( ) {\n    println!('El valor de número: {:?}', numero); \n}` \n\nEjemplo de algoritmo Fizz Buzz utilizando el ciclo for en Rust: \n\n`for numero in 1..101 {\n\n    if numero % 3 == 0 && numero % 5 == 0 {\n    println!('Fizz Buzz'); \n\n} else if numero % 3 == 0 { \n    println!('Fizz'); \n\n} else if numero % 5 == 0 {\n    println!('Buzz'); \n\n} else {\n    println!('{}', numero); \n   }\n}` \n\nNota: Debido a las limitaciones de Teloxide, no es posible utilizar comillas dobles más de una vez al enviar un mensaje a través del Bot, se sugiere reemplazar las comillas simples del ejemplo por comillas dobles al practicar").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::While => {
            bot.send_message(msg.chat.id, "El bucle while se usa para ejecutar un bloque de código mientras una condición sea verdadera\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut x = 1; \n\n   while x < 1000 { \n      x *= 2; \n\n      if x == 64 { continue; } \n\n      println!('x = {}', x); \n   } \n}`").await?;
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
            bot.send_message(msg.chat.id, "La declaración return nos permite retornar un valor de una función\\. \n\nEjemplo en Rust: \n`fn main() { \n   let x = 5; \n\n   let y = { \n       let x = 3; \n       x + 1 \n   }; \n\n   println!('El valor de y es: {}', y); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Structs => {
            bot.send_message(msg.chat.id, "Las estructuras son tipos de datos personalizados que nos permiten agrupar diferentes valores en un solo tipo\\. \n\nEjemplo en Rust: \n`struct Rectángulo { \n   ancho: u32, \n   alto: u32, \n} \n\nfn main() { \n   let rectangulo = Rectangulo { \n     ancho: 30, \n     alto: 50, \n   }; \n\n   println!('El área del rectángulo es: {}', \n   rectángulo.ancho * rectángulo.alto); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Traits => {
            bot.send_message(msg.chat.id, "Los rasgos son una característica de Rust que nos permite definir comportamientos comunes para diferentes tipos de datos\\. \n\nEjemplo en Rust: \n`trait Sumar { \n   fn sumar(&self, x: i32) -> i32; \n} \n\nstruct Numero { \n   valor: i32, \n} \n\nimpl Sumar for Numero { \n   fn sumar(&self, x: i32) -> i32 { \n      self.valor + x \n   } \n} \n\nfn main() { \n   let numero = Numero { valor: 5 }; \n\n   println!('El resultado es: {}', numero.sumar(5)); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Closures => {
            bot.send_message(msg.chat.id, "Las Closures son funciones anónimas que se pueden almacenar en variables o pasar como argumentos a otras funciones\\. \n\nEjemplo en Rust: \n`let suma = |a: i32, b: i32| -> i32 { \n   a + b \n}; \n\nfn main() { \n   let resultado = suma(5, 5); \n   println!('El resultado es: {}', resultado); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Generics => {
            bot.send_message(msg.chat.id, "Los Generics nos permiten crear funciones y estructuras que pueden aceptar diferentes tipos de datos\\. \n\nEjemplo en Rust: \n`struct Punto<T> { \n   x: T, \n   y: T, \n} \n\nfn main() { \n   let punto = Punto { \n      x: 5, \n      y: 10, \n   }; \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Option => {
            bot.send_message(msg.chat.id, "Option es un Enum que puede tener dos variantes: Some o None\\. \n\nEjemplo en Rust: \n`let numero : Option<i32> = Some(5); \n\nmatch numero { \n   Some(x) => println!('El número es: {}', x), \n   None => println!('No hay número'), \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Result => {
            bot.send_message(msg.chat.id, "El tipo Result es un Enum que representa el resultado de una operación que puede fallar\\. \n\nEjemplo en Rust: \n`enum Result<T, E> { \n   Ok(T), \n   Err(E), \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Iterators => {
            bot.send_message(msg.chat.id, "Los Iteradores son un tipo de dato que nos permiten iterar sobre una colección de datos\\. \n\nEjemplo en Rust: \n`let numeros = vec![1, 2, 3, 4, 5]; \n\nfor numero in numeros.iter() { \n   println!('El valor de número: {:?}', numero); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Crates => {
            bot.send_message(msg.chat.id, "Un Crate es un paquete de código que podemos utilizar en nuestro programa\\. \n\nEjemplo en Rust: \n`use rand::Rng; \n\nfn main() { \n   let numero = rand::thread_rng().gen_range(1..101); \n   println!('El número aleatorio es: {}', numero); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Shadowing => {
            bot.send_message(msg.chat.id, "El Shadowing nos permite reutilizar el nombre de una variable con un nuevo valor\\. \n\nEjemplo en Rust: \n`fn main() { \n   let x = 5; \n\n   let x = x + 1; \n\n   let x = x * 2; \n\n   println!('El valor de x es: {}', x); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Ownership => {
            bot.send_message(msg.chat.id, "La propiedad de Rust es un concepto que nos permite controlar el acceso y la modificación de datos\\. \n\nEjemplo en Rust: \n`fn main() { \n   let s1 = String::from('hola'); \n   let s2 = s1; \n   println!('El valor de s1 es: {}', s1); \n   println!('El valor de s2 es: {}', s2); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Referencias => {
            bot.send_message(msg.chat.id, "Las referencias nos permiten acceder a un valor sin tomar posesión de él\\. \n\nEjemplo en Rust: \n`fn main() { \n   let s1 = String::from('Hola'); \n   let len = calcular_longitud(&s1); \n\n   println!('La longitud de {} es {}', s1, len); \n} \n\nfn calcular_longitud(s: &String) -> usize { \n   s.len() \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Borrowing => {
            bot.send_message(msg.chat.id, "El préstamo es una característica de Rust que permite que un valor sea prestado a una función, método u otro valor\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut s = String::from('Hola'); \n\n   cambiar(&mut s); \n\n   println!('El valor de s es: {}', s); \n} \n\nfn cambiar(s: &mut String) { \n   s.push_str(', mundo'); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Slices => {
            bot.send_message(msg.chat.id, "Un slice es una referencia a una secuencia de elementos de una colección\\. \n\nEjemplo en Rust: \n`let numeros = [1, 2, 3, 4, 5]; \n\nlet slice = &numeros[1..3]; \n\nprintln!('El slice es: {:?}', slice); \n\n// El slice es: [2, 3]`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Modulos => {
            bot.send_message(msg.chat.id, "Los módulos nos permiten organizar nuestro código en diferentes archivos\\. \n\nEjemplo en Rust: \n`// main.rs \nmod funciones; \n\nfn main() {\n   funciones::saludar('Juan'); \n}` \n\n`// funciones.rs \npub fn saludar(nombre: &str) {\n   println!('Hola {}', nombre); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Strings => {
            bot.send_message(msg.chat.id, "Las cadenas son una estructura de datos que almacena una secuencia de caracteres\\. \n\nEjemplo en Rust: \n`fn main() { \n   let mut s = String::new(); \n\n   let datos = 'Hola'; \n\n   let s = datos.to_string(); \n\n   let s = `").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Lifetimes => {
            bot.send_message(msg.chat.id, "Las vidas útiles nos permiten especificar la duración de una referencia\\. \n\nEjemplo en Rust: \n`fn main() { \n   let s1 = String::from('Hola'); \n   let s`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Macros => {
            bot.send_message(msg.chat.id, "Los macros nos permiten escribir código que produce código\\. \n\nEjemplo en Rust: \n`macro_rules! say_hello { \n   () => ( \n      println!('Hola'); \n   ) \n} \n\nfn main() { \n   say_hello!(); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Random => {
            bot.send_message(msg.chat.id, "El módulo de aleatoriedad nos permite generar números aleatorios\\. \n\nEjemplo en Rust: \n`use rand::Rng; \n\nfn main() { \n   let mut rng = rand::thread_rng(); \n   let numero = rng.gen_range(1..101); \n   println!('El número aleatorio es: {}', numero); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Async => {
            bot.send_message(msg.chat.id, "La programación asíncrona es un estilo de programación que permite ejecutar múltiples tareas simultáneamente\\. \n\nEjemplo en Rust: \n`use tokio::time::{sleep, Duration}; \n\n#[tokio::main] \nasync fn main() { \n   println!('Iniciando'); \n   sleep(Duration::from_secs(1)).await; \n   println!('Terminado'); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Chrono => {
            bot.send_message(msg.chat.id, "El módulo Chrono nos permite trabajar con fechas y horas\\. \n\nEjemplo en Rust: \n`use chrono::prelude::*; \n\nfn main() { \n   let ahora = Utc::now(); \n   println!('La fecha y hora actual es: {}', ahora); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Scopes => {
            bot.send_message(msg.chat.id, "Los ámbitos nos permiten controlar la visibilidad de los elementos\\. \n\nEjemplo en Rust: \n`fn main() { \n   let x = 5; \n\n   { \n      let y = 10; \n      println!('El valor de x es: {} y el valor de y es: {}', x, y); \n   } \n\n   println!('El valor de x es: {} y el valor de y es: {}', x, y); \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
/*      // Comando para enviar imagen (DEPRECATED)
        Command::Image => {
            bot.send_photo(msg.chat.id, InputFile::file("./assets/images/1.png")).await?;
            // Usamos InputFile::file para enviar un archivo local y asignamos la ruta con (./ruta/de/la/imagen.jpg)).await?;
            // Nota: crear la carpeta de assets en la ubicación raíz del proyecto
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
*/
        // Llamar a la función (No Funciona)
        Command::Image => send_random_image(bot, msg).await?,

        Command::Inline => {
            bot.send_message(msg.chat.id, "Los botones inline nos permiten crear botones dentro de un mensaje\\. \n\nEjemplo en Rust: \n`use teloxide::prelude::*; \nuse teloxide::types::InlineKeyboardButton; \nuse teloxide::types::InlineKeyboardMarkup; \n\n#[tokio::main] \nasync fn main() { \n   teloxide::enable_logging!(); \n   log::info!('Bot iniciado'); \n   run().await; \n} \n\nasync fn run() { \n   let bot = Bot::from_env(); \n   Dispatcher::new(bot) \n      .messages_handler(|rx: DispatcherHandlerRx<Message>| { \n         rx.for_each(|message| async move { \n            let bot = message.bot.clone(); \n            let chat_id = message.chat.id; \n            let mut keyboard = InlineKeyboardMarkup::default(); \n            keyboard.add_row(vec![ \n               InlineKeyboardButton::callback('Botón 1', 'boton1'), \n               InlineKeyboardButton::callback('Botón 2', 'boton2'), \n            ]); \n            bot.send_message(chat_id, 'Mensaje con botones') \n               .reply_markup(keyboard) \n               .send() \n               .await \n               .log_on_error() \n               .await; \n         }) \n      }) \n      .dispatch() \n      .await; \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Menu => {
            bot.send_message(msg.chat.id, "Los menús nos permiten crear menús de opciones\\. \n\nEjemplo en Rust: \n`use teloxide::prelude::*; \nuse teloxide::types::ReplyKeyboardMarkup; \n\n#[tokio::main] \nasync fn main() { \n   teloxide::enable_logging!(); \n   log::info!('Bot iniciado'); \n   run().await; \n} \n\nasync fn run() { \n   let bot = Bot::from_env(); \n   Dispatcher::new(bot) \n      .messages_handler(|rx: DispatcherHandlerRx<Message>| { \n         rx.for_each(|message| async move { \n            let bot = message.bot.clone(); \n            let chat_id = message.chat.id; \n            let mut keyboard = ReplyKeyboardMarkup::default(); \n            keyboard.add_row(vec![ \n               KeyboardButton::new('Opción 1'), \n               KeyboardButton::new('Opción 2'), \n            ]); \n            bot.send_message(chat_id, 'Mensaje con menú') \n               .reply_markup(keyboard) \n               .send() \n               .await \n               .log_on_error() \n               .await; \n         }) \n      }) \n      .dispatch() \n      .await; \n}`").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

/*      // Comando para enviar Video (DEPRECATED)
        Command::Video => {
            bot.send_video(msg.chat.id, InputFile::file("./assets/video/boshi.mp4")).await?;
            // Usamos InputFile::file para enviar un archivo local y asignamos la ruta con (./ruta/de/la/video.mp4)).await?;
            // Nota: crear la carpeta de assets en la ubicación raíz del proyecto
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
*/
        Command::Video => send_random_video(bot, msg).await?,

/*      // Comando para enviar un gif (DEPRECATED)
        Command::Gif => {
            bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/ban.gif")).await?;
            // Usamos InputFile::file para enviar un archivo local y asignamos la ruta con (./ruta/de/la/audio.mp3)).await?;
            // Nota: crear la carpeta de assets en la ubicación raíz del proyecto
            bot.delete_message(msg.chat.id, msg.id).await?;
        }
*/

        Command::Pat => send_random_gif(bot, msg).await?,
        Command::Meme => send_random_meme(bot, msg).await?,
        Command::About => {
            bot.send_message(msg.chat.id, "Bot creado por @CrawKatt \n\nGitHub: \nhttps://github\\.com/CrawKatt").await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
        }

        Command::Bann => ban_user(bot, msg).await?,
        Command::Unbann => unban_user(bot, msg).await?,
        //Command::Mute { time, unit } => mute_user(bot, msg, calc_restrict_time(time, unit)).await?, // DEPRECATED
        Command::Mute => mute_user_two(bot, msg).await?,
    };

    Ok(())
}

// Enviar una imagen aleatoria

async fn send_random_image(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let mut rng : StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=6);
    bot.delete_message(msg.chat.id, msg.id).await?;

    match random_number {
        0 => bot.send_photo(msg.chat.id, InputFile::file("./assets/images/1.png")).await?,
        1 => bot.send_photo(msg.chat.id, InputFile::file("./assets/images/2.jpg")).await?,
        2 => bot.send_photo(msg.chat.id, InputFile::file("./assets/images/3.png")).await?,
        3 => bot.send_photo(msg.chat.id, InputFile::file("./assets/images/4.png")).await?,
        _ => bot.send_photo(msg.chat.id, InputFile::file("./assets/images/5.png")).await?,
    };
    Ok(())
}

// Enviar un gif aleatorio

async fn send_random_gif(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let mut rng : StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=21);
    bot.delete_message(msg.chat.id, msg.id).await?;

    match random_number {
        0 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/1.gif")).await?,
        1 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/2.gif")).await?,
        2 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/3.gif")).await?,
        3 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/4.gif")).await?,
        4 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/5.gif")).await?,
        5 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/6.gif")).await?,
        6 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/7.gif")).await?,
        7 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/8.gif")).await?,
        8 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/9.gif")).await?,
        9 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/10.gif")).await?,
        10 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/11.gif")).await?,
        11 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/12.gif")).await?,
        12 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/13.gif")).await?,
        13 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/14.gif")).await?,
        14 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/15.gif")).await?,
        15 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/16.gif")).await?,
        16 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/17.gif")).await?,
        17 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/18.gif")).await?,
        18 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/19.gif")).await?,
        19 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/20.gif")).await?,
        20 => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/21.gif")).await?,
        _ => bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/21.gif")).await?,
    };
    Ok(())
}

async fn send_random_meme(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let mut rng : StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=21);
    bot.delete_message(msg.chat.id, msg.id).await?;

    match random_number {
        0 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/1.jpg")).await?,
        1 => bot.send_video(msg.chat.id, InputFile::file("./assets/memes/2.mp4")).await?,
        2 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/3.jpg")).await?,
        3 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/4.jpg")).await?,
        4 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/5.jpg")).await?,
        5 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/6.jpg")).await?,
        6 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/7.jpg")).await?,
        7 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/8.jpg")).await?,
        8 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/9.jpg")).await?,
        9 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/10.jpg")).await?,
        10 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/11.jpg")).await?,
        11 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/12.jpg")).await?,
        12 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/13.jpg")).await?,
        13 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/14.jpg")).await?,
        14 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/15.jpg")).await?,
        15 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/16.jpg")).await?,
        16 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/17.jpg")).await?,
        17 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/18.jpg")).await?,
        18 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/19.jpg")).await?,
        19 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/20.jpg")).await?,
        20 => bot.send_photo(msg.chat.id, InputFile::file("./assets/memes/21.jpg")).await?,

        _ => bot.send_photo(msg.chat.id, InputFile::file("./assets/images/21.jpg")).await?,
    };
    Ok(())
}

// Enviar un video aleatorio
async fn send_random_video(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let mut rng : StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=21);
    bot.delete_message(msg.chat.id, msg.id).await?;

    match random_number {
        0 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/1.mp4")).await?,
        1 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/2.mp4")).await?,
        2 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/3.mp4")).await?,
        3 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/4.mp4")).await?,
        4 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/5.mp4")).await?,
        5 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/6.mp4")).await?,
        6 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/7.mp4")).await?,
        7 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/8.mp4")).await?,
        8 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/9.mp4")).await?,
        9 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/10.mp4")).await?,
        10 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/11.mp4")).await?,
        11 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/12.mp4")).await?,
        12 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/13.mp4")).await?,
        13 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/14.mp4")).await?,
        14 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/15.mp4")).await?,
        15 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/16.mp4")).await?,
        16 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/17.mp4")).await?,
        17 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/18.mp4")).await?,
        18 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/19.mp4")).await?,
        19 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/20.mp4")).await?,
        20 => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/21.mp4")).await?,
        _ => bot.send_video(msg.chat.id, InputFile::file("./assets/videos/21.mp4")).await?,
    };
    Ok(())
}

// Banear a un usuario respondiendo un mensaje
async fn ban_user(bot: MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = msg.reply_to_message().unwrap().from().unwrap();
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.ban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
            bot.send_message(msg.chat.id, format!("{} ha sido baneado", user.first_name)).await?;
            bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/20.gif")).await?;
        }
        None => {
            bot.delete_message(msg.chat.id, msg.id).await?;
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
            bot.send_video(msg.chat.id, InputFile::file("./assets/videos/unban.mp4")).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }
    }
    Ok(())
}

// silenciar a un usuario por tiempo indeterminado
async fn mute_user_two(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let user = msg.reply_to_message().unwrap().from().unwrap();
    bot.delete_message(msg.chat.id, msg.id).await?;
    bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::empty()).await?;
    bot.send_message(msg.chat.id, format!("{} ha sido silenciado", user.first_name)).await?;
    bot.send_animation(msg.chat.id, InputFile::file("./assets/gifs/20.gif")).await?;

    Ok(())
}

/*
// Silenciar a un usuario con un mensaje respondido (DEPRECATED).
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

*/