use crate::commands::*;

pub async fn ban_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = if let Some(from) = replied.from() {
                from
            } else {
                let error_msg = bot.send_message(msg.chat.id, "❌ No se pudo obtener el usuario").reply_to_message_id(msg.id).await?;
                let error_msg_id = error_msg.id;
                sleep(Duration::from_secs(5)).await;
                bot.delete_message(msg.chat.id, error_msg_id).await?;
                bot.delete_message(msg.chat.id, msg.id).await?;
                return Ok(());
            };
            if let Some(from) = msg.from() {
                let username_user = match user.clone().username {
                    Some(username) => username,
                    None => String::new(),
                };
                let chat_member = bot.get_chat_member(msg.chat.id, from.id).await?;
                let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();
                if is_admin_or_owner {
                    let chat_member = bot.get_chat_member(msg.chat.id, user.id).await?;
                    if chat_member.status().is_banned() {
                        let err = bot.send_message(msg.chat.id, format!
                        ("❌ {} Ya está baneado. Usa este comando solo para banear a alguien que no este baneado ", username_user))
                            .reply_to_message_id(msg.id)
                            .parse_mode(ParseMode::Html)
                            .await?;

                        sleep(Duration::from_secs(5)).await;
                        bot.delete_message(msg.chat.id, err.id).await?;
                        bot.delete_message(msg.chat.id, msg.id).await?;

                        return Ok(());
                    } else {
                        bot.ban_chat_member(msg.chat.id, user.id).await?;
                        let mut rng: StdRng = SeedableRng::from_entropy();
                        let random_number = rng.gen_range(0..=14);
                        let file_names = [
                            "1.gif", "2.gif", "3.gif", "4.gif", "5.gif",
                            "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
                            "11.gif", "12.mp4", "13.mp4", "14.mp4",
                        ];
                        let get_file_name = |index: usize| -> &'static str {
                            file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
                        };
                        let file_path = format!("./assets/ban/{}", get_file_name(random_number));

                        match file_path.ends_with(".gif") {

                            true => {
                                bot.send_animation(msg.chat.id, InputFile::file(file_path))
                                    .caption(format!("✅ @{} [<code>{}</code>] baneado", username_user, user.id))
                                    .parse_mode(ParseMode::Html)
                                    .reply_to_message_id(msg.id)
                                    .await?;
                            }

                            false => {
                                bot.send_video(msg.chat.id, InputFile::file(file_path))
                                    .caption(format!("✅ @{} <code>[{}</code>] baneado", username_user, user.id))
                                    .parse_mode(ParseMode::Html)
                                    .reply_to_message_id(msg.id)
                                    .await?;
                            }

                        }

                    }

                } else {
                    let err = bot.send_message(msg.chat.id, "❌ No tienes permisos para usar este comando").reply_to_message_id(msg.id).await?;
                    sleep(Duration::from_secs(5)).await;
                    bot.delete_message(msg.chat.id, err.id).await?;
                    bot.delete_message(msg.chat.id, msg.id).await?;
                };

            }

        }
        None => {
            get_user_id_by_arguments(bot, msg).await?;
        }

    }

    Ok(())
}

/*
╔═════════════════════════════════════════════════════╗
║    || - || Desarrollado por @CrawKatt || - ||       ║
║    --| https://github.com/CrawKatt/TeloxBeta |--    ║
╚═════════════════════════════════════════════════════╝
*/
