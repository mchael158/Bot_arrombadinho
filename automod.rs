use std::string;

use chrono::{Utc};
use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, RoleId, UserId};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::CommandResult;
use tokio::time::Duration;
use serenity::builder::EditMember;
use tokio::time::sleep;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum EventType {
    MessageSend,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Trigger {
    ForbiddenWords(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Action {
    MuteUser(UserId),
    WarnUser(UserId, String), 
    TimeoutUser(UserId), 
}

// Constantes
const MUTE_ROLE: u64 = ;
const CHANNEL_BAGUNCA: u64 = ;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Rule {
    pub id: u64,
    pub guild_id: GuildId,
    pub name: String,
    pub creator_id: UserId,
    pub event_type: EventType,
    #[serde(flatten)]
    pub trigger: Trigger,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub exempt_roles: Vec<RoleId>,
    pub exempt_channels: Vec<ChannelId>,
}

pub async fn check_automod_rules(msg: &Message) -> Option<Rule> {
    let forbidden_words = vec!["motivo:".to_string(), "cachorro".to_string()]; // Palavras proibidas

    // Cria a regra de automod
    let rule = Rule {
        id: 1,
        guild_id: msg.guild_id.unwrap(),
        name: String::from("palavras proibidas"),
        creator_id: msg.author.id,
        event_type: EventType::MessageSend,
        trigger: Trigger::ForbiddenWords(forbidden_words.clone()),
        actions: vec![
            Action::MuteUser(msg.author.id),
            Action::WarnUser(msg.author.id, "Você recebeu um warn".to_string()),
            Action::TimeoutUser(msg.author.id),
        ],
        enabled: true,
        exempt_roles: vec![],
        exempt_channels: vec![],
    };

    
    if let Trigger::ForbiddenWords(words) = &rule.trigger {
        for word in words {
            if msg.content.contains(word) {
                return Some(rule);
            }
        }
    }

    None
}

// Função do automod
pub async fn execute_automod_actions(ctx: Context, msg: Message, rule: Rule) -> CommandResult {
    for action in &rule.actions {
        match action {
            Action::TimeoutUser(user_id) => {
                if let Some(guild_id) = msg.guild_id {
                    let timeout_duration = Duration::from_secs(3600);
                    let timeout_until = Utc::now() + chrono::Duration::from_std(timeout_duration).unwrap();
                    let timeout_until_timestamp = Timestamp::from_unix_timestamp(timeout_until.timestamp()).unwrap();

      
                    let edit_member = EditMember::default();
                    let _ = edit_member.clone().disable_communication_until(timeout_until_timestamp.to_string());

                    if let Err(why) = guild_id.edit_member(&ctx.http, *user_id, edit_member).await {
                        println!("Erro ao aplicar o timeout: {:?}", why);
                    } else {
                        let user_mention_arroba = format!("<@{}>", user_id);
                        let chanel_id_bagunca = ChannelId::new(CHANNEL_BAGUNCA);
                        let timeout_message = format!("{}.", user_mention_arroba);
                        chanel_id_bagunca.say(&ctx.http, timeout_message).await.unwrap();

                        let ctx_clone = ctx.clone();
                        let user_id_clone = *user_id;

                        tokio::spawn(async move {
                            tokio::time::sleep(timeout_duration).await;

                            //olha se o membro existe
                            if let Ok(_member) = guild_id.member(&ctx_clone.http, user_id_clone).await {
                                //informa que o usuario foi desmutado
                                let _ = chanel_id_bagunca.say(&ctx_clone.http, format!("{} você foi removido do mute.", user_mention_arroba)).await;
                            }
                        });
                    }
                }
            },

            Action::WarnUser(user_id, warning_message) => {
                let chanel_id_bagunca = ChannelId::new(CHANNEL_BAGUNCA);
                let user_mention_arroba = format!("<@{}>", user_id);
                if let Some(_guild_id) = msg.guild_id {
                    if let Err(why) = chanel_id_bagunca.say(&ctx.http, format!("{}: {}", user_mention_arroba, warning_message)).await {
                        println!("Erro ao enviar aviso ao usuário: {:?}", why);
                    }
                }
            },
            Action::MuteUser(user_id) => {
                let mute_role_id = RoleId::new(MUTE_ROLE); // ID do cargo de mute
                if let Some(guild_id) = msg.guild_id {
                    if let Ok(member) = guild_id.member(&ctx.http, *user_id).await {
                        if let Err(why) = member.add_role(&ctx.http, mute_role_id).await {
                            println!("Erro ao mutar o usuário: {:?}", why);
                        } else {
                            let user_mention_arroba = format!("<@{}>", user_id);
                            let mute_time_secs = 60;
                            let mute_time = Duration::from_secs(mute_time_secs);
            
                            let mute_time_in_minutes = mute_time_secs / 60;
                            let mute_message = if mute_time_in_minutes >= 60 {
                                let mute_in_hours = mute_time_in_minutes / 60;
                                format!("{} você foi mutado por {} horas.", user_mention_arroba, mute_in_hours)
                            } else {
                                format!("{} você foi mutado por {} minutos.", user_mention_arroba, mute_time_in_minutes)
                            };
            
                            let chanel_id_bagunca = ChannelId::new(CHANNEL_BAGUNCA);
                            chanel_id_bagunca.say(&ctx.http, mute_message).await.unwrap();
            
                            let ctx_clone = ctx.clone();
                            let user_id_clone = *user_id;
                            tokio::spawn(async move {
                                sleep(mute_time);
                                if let Ok(_) = guild_id.member(&ctx_clone.http, user_id_clone).await {
                                    if let Err(err) = member.remove_role(&ctx_clone.http, mute_role_id).await {
                                        println!("Não foi possível remover o mute do usuário: {:?}", err);
                                    } else {
                                        let _ = chanel_id_bagunca.say(&ctx_clone.http, format!("Usuário {} foi desmutado.", user_mention_arroba)).await;
                                    }
                                }
                            });
                        }
                    }
                }
            },

            
        }
    }
    Ok(())
}
