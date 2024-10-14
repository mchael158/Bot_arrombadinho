use serenity::builder::CreateEmbed;
use chrono::Utc;
use chrono_tz::Tz;
use serenity::all::{MessageDeleteEvent, User};


pub fn create_ban_log_embed(user_id: u64, ban_reason: String) -> CreateEmbed {
    let timestamp_utc = Utc::now();
    let brt: Tz = "America/Sao_Paulo".parse().unwrap();
    let formatted_timestamp = timestamp_utc.with_timezone(&brt).format("%d/%m/%Y %H:%M:%S").to_string();

    let user_mention_arroba = format!("<@{}>", user_id);
    CreateEmbed::new()
        .title("Logs do servidor.")
        .description(format!(
            "Usuário: {}\nTipo: Banido\nData e Hora: {}\nRazão: {}",
            user_mention_arroba, formatted_timestamp, ban_reason
        ))
        .color(0x00ff00)
}


pub fn create_mute_log_embed(user_id: u64, mute_reason: String) -> CreateEmbed{
    let timestamp_utc = Utc::now();
    let brt: Tz = "America/Sao_Paulo".parse().unwrap();
    let formatted_timestamp = timestamp_utc.with_timezone(&brt).format("%d/%m/%Y %H:%M:%S").to_string();
    let user_mention_arroba = format!("<@{}>", user_id);
    
    CreateEmbed::new()
        .title("Logs do servidor.")
        .description(format!(
            "Usuário: {}\nTipo: Silenciado\nData e Hora: {}\nRazão: {}",
            user_mention_arroba, formatted_timestamp, mute_reason
        ))
        .color(0xff0000)

}

pub fn create_delete_message_log_embed(user: &User, delete_msg: &MessageDeleteEvent) -> CreateEmbed {
    let timestamp_utc = Utc::now();
    let brt: Tz = "America/Sao_Paulo".parse().unwrap();
    let formatted_timestamp = timestamp_utc.with_timezone(&brt).format("%d/%m/%Y %H:%M:%S").to_string();

    let user_mention_arroba = format!("<@{}>", user.id);
    
    CreateEmbed::new()
        .title("Logs do servidor.")
        .description(format!(
            "Usuário: {}\nTipo: Mensagem Deletada\nData e Hora: {}\nConteúdo: {}\nID da Mensagem: {}",
            user_mention_arroba,
            formatted_timestamp,
            delete_msg.message_id,
            delete_msg.channel_id
        ))
        .color(0xf10000)
}
