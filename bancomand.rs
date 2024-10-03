use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::all::ChannelId;
use serenity::framework::standard::CommandError;
use serenity::builder::CreateEmbed;
use serenity::all::CreateMessage;
use serenity::all::Permissions;
use serenity::all::Message;
use serenity::all::GuildId;


use crate::constantes::{GUILD_i_ID, CHANNEL_BAGUNCA};

use crate::embediss;

const CHANNEL_LOGS_ID: u64 = 1159998830239027272;

pub async fn ban_user(
    ctx: &Context,             //referência
    msg: &Message,             //referência
    guild_id: GuildId          //GuildId como argumento
) -> Result<(), CommandError> {
    if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
        if member.permissions(&ctx.cache).unwrap_or_else(|_| Permissions::empty()).ban_members() {
            let mut args = msg.content.split_whitespace();
            args.next();

            if let Some(user_id_or_mention) = args.next() {       
                let user_id_str = user_id_or_mention.trim_matches(|c| c == '<' || c == '@' || c == '>' || c == '!');
                if let Ok(user_id) = user_id_str.parse::<u64>() {
                    let member_id = UserId::new(user_id);
                    let user_mention_arroba = format!("<@{}>", user_id);

                    let reason = args.collect::<Vec<&str>>().join(" ");
                    let ban_reason = if reason.is_empty() {
                        "Motivo do banimento".to_string()
                    } else {
                        reason
                    };

                    guild_id.ban_with_reason(&ctx.http, member_id, 7, &ban_reason).await?;

                    msg.channel_id.say(&ctx.http, format!("Usuário {} banido com sucesso! Motivo: {}", user_mention_arroba, ban_reason)).await?;

                    let embed = embediss::create_ban_log_embed(user_id, ban_reason);
                    let channel_logs_id = ChannelId::new(CHANNEL_LOGS_ID);
                    let message_embed = CreateMessage::default().embed(embed);
                    channel_logs_id.send_message(&ctx.http, message_embed).await?;
                } else {
                    msg.channel_id.say(&ctx.http, "ID de usuário inválido.").await?;
                }
            } else {
                msg.channel_id.say(&ctx.http, "Por favor, forneça o ID do usuário.").await?;
            }
        } else {
            msg.channel_id.say(&ctx.http, "Você não tem permissão para usar esse comando.").await?;
        }
    }
    Ok(())
}
