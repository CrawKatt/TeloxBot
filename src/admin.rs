//use std::time::Duration;
use crate::ResponseResult;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

use teloxide::types::InputFile;
use teloxide::types::ChatPermissions;
use teloxide::prelude::Requester;
use teloxide_core::prelude::{UserId};
use teloxide_core::types::{ChatMemberStatus, Message};
use crate::comandos::MyBot;


// Banear a un usuario respondiendo un mensaje
pub async fn ban_user(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let user = msg.reply_to_message().and_then(|replied| replied.from()).unwrap();
    let chat_id = msg.chat.id;
    let username_user = user.username.clone().unwrap_or_default();
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
    let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();
    if is_admin_or_owner {
        bot.delete_message(chat_id, msg.id).await?;
        bot.ban_chat_member(chat_id, user.id).await?;
        bot.send_message(chat_id, format!("@{} ha sido baneado", username_user), ).await?;
        let mut rng: StdRng = SeedableRng::from_entropy();
        let random_number = rng.gen_range(0..=14);
        let file_names = [
            "1.gif", "2.gif", "3.gif", "4.gif", "5.gif",
            "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
            "11.gif", "12.mp4", "13.mp4", "14.mp4", "15.mp4",
        ];
        let get_file_name = |index: usize| -> &'static str {
            file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
        };
        let file_path = format!("./assets/ban/{}", get_file_name(random_number));

        if file_path.ends_with(".gif") {
            bot.send_animation(chat_id, InputFile::file(file_path)).await?;
        } else {
            bot.send_video(chat_id, InputFile::file(file_path)).await?;
        }
    } else {
        bot.delete_message(chat_id, msg.id).await?;
        bot.send_message(chat_id, "No tienes permisos para usar este comando",).await?;
    }
    Ok(())
}
pub async fn ban_id(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let text = &msg.text().unwrap();
    let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
    let user_id = arguments.trim().parse::<i64>().unwrap();
    let chat_id = msg.chat.id;
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
    let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
    if is_admin_or_owner {
        bot.ban_chat_member(chat_id, UserId(user_id as u64)).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "Usuario Baneado").await?;
        let mut rng: StdRng = SeedableRng::from_entropy();
        let random_number = rng.gen_range(0..=14);
        let file_names = [
            "1.gif", "2.gif", "3.gif", "4.gif", "5.gif",
            "6.gif", "7.gif", "8.gif", "9.gif", "10.gif",
            "11.gif", "12.mp4", "13.mp4", "14.mp4", "15.mp4",
        ];
        let get_file_name = |index: usize| -> &'static str {
            file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
        };
        let file_path = format!("./assets/ban/{}", get_file_name(random_number));
        if file_path.ends_with(".gif") {
            bot.send_animation(chat_id, InputFile::file(file_path)).await?;
        } else {
            bot.send_video(chat_id, InputFile::file(file_path)).await?;
        }
    } else {
        bot.send_message(msg.chat.id, "No tienes permisos para banear a un usuario").await?;
    }
    Ok(())
}
pub async fn unban_user(bot: MyBot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            let user = replied.from().unwrap();
            let chat_id = msg.chat.id;
            let username_user = user.username.clone().unwrap_or_default();
            let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
            let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();
            if is_admin_or_owner {
                bot.delete_message(chat_id, msg.id).await?;
                bot.unban_chat_member(chat_id, user.id).await?;
                bot.send_message(chat_id, format!("@{} ha sido desbaneado", username_user)).await?;
                bot.send_video(chat_id, InputFile::file("./assets/unban/1.mp4")).await?;
            } else {
                bot.delete_message(chat_id, msg.id).await?;
                bot.send_message(chat_id, "No tienes permisos para remover el ban a un usuario.").await?;
            }
        }
        None => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.send_message(msg.chat.id, "Usa este comando respondiendo a otro mensaje").await?;
        }
    }
    Ok(())
}
pub async fn unban_id(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let text = &msg.text().unwrap();
    let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
    let user_id = arguments.trim().parse::<i64>().unwrap();
    let chat_id = msg.chat.id;
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
    let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
    if is_admin_or_owner {
        bot.unban_chat_member(chat_id, UserId(user_id as u64)).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "Usuario Desbaneado").await?;
        bot.send_video(chat_id, InputFile::file("./assets/unban/1.mp4")).await?;
    } else {
        bot.send_message(msg.chat.id, "No tienes permisos para remover el ban a un usuario").await?;
    }
    Ok(())
}
pub async fn mute_user_admin(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let user = msg.reply_to_message().unwrap().from().unwrap();
    let chat_id = msg.chat.id;
    let username_user = user.username.clone().unwrap_or_default();
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
    let is_admin_or_owner = chat_member.status().is_administrator() || chat_member.status().is_owner();
    if is_admin_or_owner {
        bot.delete_message(chat_id, msg.id).await?;
        bot.restrict_chat_member(chat_id, user.id, ChatPermissions::empty()).await?;
        bot.send_message(chat_id, format!("@{} ha sido silenciado", username_user)).await?;
        let mut rng: StdRng = SeedableRng::from_entropy();
        let file_names = [
            "1.gif", "2.gif", "3.gif", "4.gif", "5.jpg",
        ];
        let random_number = rng.gen_range(0..=file_names.len() - 1);
        let file_path = format!("./assets/mute/{}", file_names[random_number]);
        let file_extension = file_path.split(".").last().unwrap_or("");
        match file_extension {
            "gif" => bot.send_animation(chat_id, InputFile::file(file_path)).await?,
            "jpg" => bot.send_photo(chat_id, InputFile::file(file_path)).await?,
            _ => bot.send_message(chat_id, "No se pudo enviar el archivo").await?,
        };
    } else {
        bot.delete_message(chat_id, msg.id).await?;
        bot.send_message(chat_id, "No tienes permisos para silenciar a un usuario").await?;
    };
    Ok(())
}
pub async fn mute_id(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let text = &msg.text().unwrap();
    let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
    let user_id = arguments.trim().parse::<i64>().unwrap();
    let chat_id = msg.chat.id;
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
    let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
    if is_admin_or_owner {
        bot.restrict_chat_member(chat_id, UserId(user_id as u64), ChatPermissions::empty()).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "Usuario Silenciado").await?;
        let mut rng: StdRng = SeedableRng::from_entropy();
        let random_number = rng.gen_range(0..=6);
        let file_names = [
            "1.gif", "2.gif", "3.gif", "4.gif", "5.jpg",
        ];
        let get_file_name = |index: usize| -> &'static str {
            file_names.get(index).unwrap_or_else(|| file_names.last().unwrap())
        };
        let file_path = format!("./assets/mute/{}", get_file_name(random_number));
        if file_path.ends_with(".gif") {
            bot.send_animation(chat_id, InputFile::file(file_path)).await?;

        } else {
            bot.send_photo(chat_id, InputFile::file(file_path)).await?;
        }
    } else {
        bot.send_message(msg.chat.id, "No tienes permisos para silenciar a un usuario").await?;
    }
    Ok(())
}
pub async fn unmute_user(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let user = msg.reply_to_message().unwrap().from().unwrap();
    let username_user = user.clone().username;
    let chat_member = bot.get_chat_member(msg.chat.id, msg.from().unwrap().id).await?;
    let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
    if is_admin_or_owner {
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::all()).await?;
        bot.send_message(msg.chat.id, format!("@{} ya no est?? silenciado", username_user.unwrap())).await?;
        bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4")).await?;
    } else {
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "No tienes permisos para remover el silencio a un usuario").await?;
    }
    Ok(())
}
pub async fn unmute_id(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let text = &msg.text().unwrap();
    let (_, arguments) = text.split_at(text.find(' ').unwrap_or(text.len()));
    let user_id = arguments.trim().parse::<i64>().unwrap();
    let chat_id = msg.chat.id;
    let chat_member = bot.get_chat_member(chat_id, msg.from().unwrap().id).await?;
    let is_admin_or_owner = chat_member.status() == ChatMemberStatus::Administrator || chat_member.status() == ChatMemberStatus::Owner;
    if is_admin_or_owner {
        bot.restrict_chat_member(chat_id, UserId(user_id as u64), ChatPermissions::all()).await?;
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, "El Usuario ya no esta silenciado").await?;
        bot.send_video(msg.chat.id, InputFile::file("./assets/unmute/unmute.mp4")).await?;
    } else {
        bot.send_message(msg.chat.id, "No tienes permisos para remover el silencio a un usuario").await?;
    }
    Ok(())
}
pub async fn get_chat_member(bot: MyBot, msg: Message) -> ResponseResult<()> {
    let usuario = msg.reply_to_message().unwrap().from().unwrap();
    let user_id = usuario.id;
    bot.send_message(msg.chat.id, format!("{user_id}")).await?;
    Ok(())
}