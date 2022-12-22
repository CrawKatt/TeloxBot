use std::path::Path;
use teloxide::types::InputFile;
use teloxide::prelude::Requester;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

use crate::comandos;
use crate::Message;
use crate::ResponseResult;


pub async fn send_pat(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif",
        "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
        "11.gif", "12.gif", "13.gif", "14.gif", "15.gif",
        "16.gif", "17.gif", "18.gif", "19.gif", "20.gif",
        "21.gif", "22.gif", "23.gif", "24.gif", "25.gif",
        "26.gif", "27.gif", "28.gif", "29.gif",
    ];
    let get_file_name = |index: usize| -> &'static str {
        file_names.get(index).unwrap_or(&file_names[0])
    };
    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);
    bot.delete_message(msg.chat.id, msg.id).await?;
    let file_path = format!("./assets/pat/{}", get_file_name(random_number));
    if Path::new(&file_path).is_file() {
        bot.send_animation(msg.chat.id, InputFile::file(&file_path)).await?;
    }
    Ok(())
}

pub async fn send_random_meme(bot: comandos::MyBot, msg: Message) -> ResponseResult<()> {
    let file_names = [
        "1.mp4", "2.mp4", "3.mp4", "4.mp4", "5.mp4",
        "6.jpg", "7.jpg", "8.jpg", "9.jpg", "10.jpg",
        "11.jpg", "12.jpg", "13.jpg", "14.jpg", "15.jpg",
        "16.jpg", "17.jpg", "18.jpg", "19.jpg", "20.jpg",
        "21.jpg", "22.jpg", "23.jpg", "24.jpg", "25.jpg",
        "26.jpg", "27.jpg", "28.jpg", "29.jpg",
    ];
    let get_file_name = |index: usize| -> &'static str {
        file_names.get(index).unwrap_or(&file_names[0])
    };

    // generamos un numero aleatorio
    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);
    bot.delete_message(msg.chat.id, msg.id).await?;
    let file_path = format!("./assets/memes/{}", get_file_name(random_number));
    if Path::new(&file_path).is_file() {
        if file_path.ends_with(".mp4") {
            bot.send_video(msg.chat.id, InputFile::file(&file_path)).await?;
        } else {
            bot.send_photo(msg.chat.id, InputFile::file(&file_path)).await?;
        }
    }

    Ok(())

}
