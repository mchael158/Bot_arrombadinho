use serenity::{
    async_trait,
    model::{channel::Message, gateway::{GatewayIntents, Ready}, permissions::Permissions, id::{GuildId, UserId}},
    prelude::*,
    builder::CreateEmbed,
    all::ChannelId,
};
use chrono::{Utc}; // tempo
use chrono_tz::Tz; //tempo
use serenity::all::CreateMessage;

mod automod;

// Constantes
const MUTE_COMAND: &str = "-mute";
const BAN_COMMAND: &str = "-ban";
const HELP_MESSAGE: &str = "Aqui podemos te ajudar no que for, reaja com o emoji {🚨} para abrir um ticket.";
const GUILD_ID: u64 = 1145453848039805028;
const CHANNEL_LOGS_ID: u64 = 1159998830239027272;

//cabeça do Handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(rule_triggered) = automod::check_automod_rules(&msg).await {     
            if let Err(why) = automod::execute_automod_actions(ctx, msg, rule_triggered).await {
                println!("Erro ao executar ação do automod: {:?}", why);
            }
            return;
        }
        if msg.content == HELP_MESSAGE {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Erro no comando de ajuda: {:?}", why);
            }
        } else if msg.content.starts_with(BAN_COMMAND) {
            if let Some(guild_id) = msg.guild_id {
                if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
                    if member.permissions(&ctx.cache).unwrap_or_else(|_| Permissions::all()).ban_members() {
                        let mut args = msg.content.split_whitespace();
                        args.next();
        
                        // Obtém o ID do usuário
                        if let Some(user_id_or_mention) = args.next() {
                            let user_id_str = user_id_or_mention.trim_matches(|c| c == '<' || c == '@' || c == '>' || c == '!');
        
                            if let Ok(user_id) = user_id_str.parse::<u64>() {
                                let member_id = UserId::new(user_id);
        
                                //motivo do banimento
                                let reason = args.collect::<Vec<&str>>().join(" ");
                                let ban_reason = if reason.is_empty() { "Motivo do banimento".to_string() } else { reason };
        
                                if let Err(why) = guild_id.ban_with_reason(&ctx.http, member_id, 7, &ban_reason).await {
                                    println!("não foi possível banir o bosta: {:?}", why);
                                } else {
                                    //mensagem de sucesso
                                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("macaco {} banido com sucesso! Motivo: {}", member_id, ban_reason)).await {
                                        println!("Erro ao enviar mensagem de sucesso: {:?}", why);
                                    }
                                    
                                    //envia log de banimento
                                    let timestamp_utc = Utc::now();
                                    let brt: Tz = "America/Sao_Paulo".parse().unwrap();
                                    let formatted_timestamp = timestamp_utc.with_timezone(&brt).format("%d/%m/%Y %H:%M:%S").to_string();
                                    
                                    let user_mention_arroba = format!("<@{}>", user_id);
                                    let embed = CreateEmbed::new()
                                        .title("Logs do servidor.")
                                        .description(format!("Usuário: {}\nfoi: punido\nData e Hora: {}\nRazão: {}",
                                            user_mention_arroba,
                                            formatted_timestamp,
                                            ban_reason))
                                        .color(0x00ff00);

                                    let channel_logs_id = ChannelId::new(CHANNEL_LOGS_ID);
                                    let message_embed = CreateMessage::default().embed(embed);
                                    if let Err(why) = channel_logs_id.send_message(&ctx.http, message_embed).await {
                                        println!("Erro ao enviar logs: {:?}", why);
                                    }
                                }
                            } else {
                                if let Err(why) = msg.channel_id.say(&ctx.http, "ID de usuário inválido.").await {
                                    println!("Erro ao enviar mensagem de erro: {:?}", why);
                                }
                            }
                        } else {
                            if let Err(why) = msg.channel_id.say(&ctx.http, "passa o id do arrombado caralho").await {
                                println!("Erro ao enviar mensagem de erro: {:?}", why);
                            }
                        }
                    } else {
                        if let Err(why) = msg.channel_id.say(&ctx.http, "Você é staff o filha da puta?.").await {
                            println!("Erro ao enviar mensagem de permissão negada: {:?}", why);
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("bucetinha está online");
    }
}

#[tokio::main]
async fn main() {
    let token = "...";
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("erro ao criar o cliente fodido");

    if let Err(why) = client.start().await {
        println!("Erro na porra do client: {:?}", why);
    }
}
