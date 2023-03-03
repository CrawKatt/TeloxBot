use crate::admin_commands::*;

pub async fn ban_animation_generator(bot: Bot, msg: Message) -> ResponseResult<()> {

    let text = if let Some(text) = msg.text() {
        text
    } else {
        return Ok(());
    };

    let (_, username) = match text.find('@') {
        Some(index) => text.split_at(index),
        None => ("", text),
    };

    if username.contains('@') {
        let contents = read_database_file().await?;
        let user_data_vec: Vec<UserData> = match serde_json::from_str(&contents) {
            Ok(vec) => vec,
            Err(_) => {
                eprintln!("❌ No se pudo leer el archivo de base de datos");
                Vec::new()
            }
        };

        let user_id = user_data_vec.iter()
            .find_map(|data| {
                if let Some(name) = &data.username {
                    if name == username {
                        Some(data.id.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

        if let Some(user_id) = user_id {

            let user_id = match user_id.parse::<u64>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("❌ No se pudo obtener el ID del usuario: {}", e);
                    return Ok(());
                }
            };

            let text = if let Some(msg_text) = msg.text() {
                msg_text
            } else {
                println!("❌ No se pudo obtener el texto del mensaje {:#?}", msg);
                return Ok(())
            };

            // get the arguments after the command trigger
            let (_, arguments) = match text.find(' ') {
                Some(index) => text.split_at(index),
                None => ("", text),
            };

            // check if the arguments are empty
            if arguments.is_empty() {
                bot.send_message(msg.chat.id, "❌ No has especificado un ID para obtener el usuario").await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                println!("❌ No has especificado un ID para obtener el usuario {:#?}", msg);

                return Ok(());
            }

            let mut rng: StdRng = SeedableRng::from_entropy();

            // generate a random number from 0 to 14
            let random_number = rng.gen_range(0..=14);

            // List of ban animations
            let file_names = [
                "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif",
                "9.gif", "10.gif", "11.gif", "12.mp4", "13.mp4", "14.mp4",
            ];

            // Get the file name from the list
            let get_file_name = |index: usize| -> &'static str {
                if let Some(file_name) = file_names.get(index) {
                    file_name
                } else {
                    match file_names.last() {
                        Some(last_file) => last_file,
                        None => {
                            ""
                        }
                    }
                }
            };

            let file_path = format!("./assets/ban/{}", get_file_name(random_number));
            match file_path.ends_with(".gif") {

                true => {
                    let gif = bot.send_animation(msg.chat.id, InputFile::file(file_path))
                        .caption(format!("{} [<code>{}</code>] baneado", username, UserId(user_id))).parse_mode(ParseMode::Html).await?;
                    sleep(Duration::from_secs(60)).await;
                    bot.delete_message(msg.chat.id, gif.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                }

                // Else, send it as a video.
                false => {
                    let video = bot.send_video(msg.chat.id, InputFile::file(file_path))
                        .caption(format!("{} [<code>{}</code>] baneado", username, user_id)).parse_mode(ParseMode::Html).await?;
                    sleep(Duration::from_secs(60)).await;
                    bot.delete_message(msg.chat.id, video.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                }

            };

        }

    }

    Ok(())
}

pub async fn mute_animation_generator(bot: Bot, msg: Message) -> ResponseResult<()> {

    let text = if let Some(text) = msg.text() {
        text
    } else {
        return Ok(());
    };

    let (_, username) = match text.find('@') {
        Some(index) => text.split_at(index),
        None => ("", text),
    };

    if username.contains('@') {
        let contents = read_database_file().await?;
        let user_data_vec: Vec<UserData> = match serde_json::from_str(&contents) {
            Ok(vec) => vec,
            Err(_) => {
                Vec::new()
            }
        };

        let user_id = user_data_vec.iter()
            .find_map(|data| {
                if let Some(name) = &data.username {
                    if name == username {
                        Some(data.id.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

        if let Some(user_id) = user_id {

            let user_id = match user_id.parse::<u64>() {
                Ok(id) => id,
                Err(_) => {
                    return Ok(());
                }
            };

            let mut rng: StdRng = SeedableRng::from_entropy();
            let file_names = ["1.gif", "2.gif", "3.gif", "4.gif", "5.jpg"];
            let random_number = rng.gen_range(0..=file_names.len() - 1);

            let file_path = format!("./assets/mute/{}", file_names[random_number]);
            let file_extension = match file_path.split('.').last() {
                Some(extension) => extension,
                None => "",
            };

            match file_extension {

                "gif" => {
                    let ok = bot.send_animation(msg.chat.id, InputFile::file(file_path))
                        .caption(format!("{} [<code>{}</code>] ha sido silenciado", username, UserId(user_id)
                        ))
                        .parse_mode
                        (ParseMode::Html)
                        .reply_to_message_id
                        (msg.id)
                        .await?;

                    sleep(Duration::from_secs(60)).await;
                    bot.delete_message(msg.chat.id, ok.id).await?;
                },

                "jpg" => {
                    let ok = bot.send_photo(msg.chat.id, InputFile::file(file_path))
                        .caption(format!(
                            "{} [<code>{}</code>] ha sido silenciado", username, UserId(user_id)
                        ))
                        .parse_mode
                        (ParseMode::Html)
                        .reply_to_message_id
                        (msg.id)
                        .await?;

                    sleep(Duration::from_secs(60)).await;
                    bot.delete_message(msg.chat.id, ok.id).await?;
                },

                _ => {
                    let err = bot.send_message(msg.chat.id, "❌ No se pudo enviar el archivo").await?;
                    sleep(Duration::from_secs(60)).await;
                    bot.delete_message(msg.chat.id, err.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                }

            };

        }

    }

    Ok(())
}

pub async fn send_random_meme_generator(bot: Bot, msg: Message) -> ResponseResult<()> {
    let file_names = [
        "1.mp4", "2.mp4", "3.mp4", "4.mp4", "5.mp4",
        "6.jpg", "7.jpg", "8.jpg", "9.jpg", "10.jpg", "11.jpg", "12.jpg", "13.jpg", "14.jpg",
        "15.jpg", "16.jpg", "17.jpg", "18.jpg", "19.jpg", "20.jpg", "21.jpg", "22.jpg", "23.jpg",
        "24.jpg", "25.jpg", "26.jpg", "27.jpg", "28.jpg", "29.jpg",
    ];
    let get_file_name = |index: usize| -> &'static str {
        if let Some(file_name) = file_names.get(index) {
            file_name
        } else {
            match file_names.last() {
                Some(last_file) => last_file,
                None => {
                    ""
                }
            }
        }
    };

    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);

    let file_path = format!("./assets/memes/{}", get_file_name(random_number));
    if Path::new(&file_path).is_file() {
        if file_path.ends_with(".mp4") {
            bot.send_video(msg.chat.id, InputFile::file(&file_path)).caption("Aquí tienes un meme de programación").reply_to_message_id(msg.id).await?;
        } else {
            bot.send_photo(msg.chat.id, InputFile::file(&file_path))
                .caption("Aquí tienes un meme de programación")
                .reply_to_message_id(msg.id)
                .await?;
        }
    }

    Ok(())
}

pub async fn random_pat(bot: Bot, msg: Message) ->ResponseResult<()> {

    let file_names = [
        "1.gif", "2.gif", "3.gif", "4.gif", "5.gif", "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
        "11.gif", "12.gif", "13.gif", "14.gif", "15.gif", "16.gif", "17.gif", "18.gif", "19.gif",
        "20.gif", "21.gif", "22.gif", "23.gif", "24.gif", "25.gif", "26.gif", "27.gif", "28.gif",
        "29.gif",
    ];

    let get_file_name = |index: usize| -> &'static str {
        if let Some(file_name) = file_names.get(index) {
            file_name
        } else {
            match file_names.last() {
                Some(last_file) => last_file,
                None => {
                    ""
                }
            }
        }
    };

    let mut rng: StdRng = SeedableRng::from_entropy();
    let random_number = rng.gen_range(0..=file_names.len() - 1);

    let file_path = format!("./assets/pat/{}", get_file_name(random_number));

    if Path::new(&file_path).is_file() {
        let ok = bot.send_animation(msg.chat.id, InputFile::file(&file_path)).await?;
        sleep(Duration::from_secs(5)).await;
        bot.delete_message(msg.chat.id, ok.id).await?;
    }

    Ok(())
}