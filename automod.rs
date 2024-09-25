use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, GuildId, RoleId, UserId};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::{CommandResult};
use std::collections::HashMap;
use serenity::all::Guild;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum EventType {
    MessageSend,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Trigger {
    ForbiddenWords(Vec<String>), //palavras proibidas 
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Action {
    MuteUser(UserId), //action de mutar o user
}

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

pub async fn apply_rule(ctx: &Context, msg: &Message, rule: &Rule) -> CommandResult {
    if !rule.enabled {
        return Ok(());
    }

    //canal isento de mute
    if rule.exempt_channels.contains(&msg.channel_id) {
        return Ok(());
    }

    //cargo isento de mute
    if let Some(guild_id) = msg.guild_id {
        let guild = match guild_id.to_guild_cached(&ctx.cache) {
            Some(guild) => guild,
            None => return Ok(()),
        };

        let member = match guild.member(&ctx.http, msg.author.id).await {
            Ok(member) => member,
            Err(_) => return Ok(()),
        };
        
        for role_id in &member.roles {
            if rule.exempt_roles.contains(role_id) {
                return Ok(());
            }
        }
    }

    //garilho
    match &rule.trigger {
        Trigger::ForbiddenWords(words) => {
            for word in words {
                if msg.content.contains(word) {
                    //ação: mutar o usuário
                    for action in &rule.actions {
                        match action {
                            Action::MuteUser(user_id) => {
                                //role de mute no usuário
                                let mute_role_id = RoleId::new(1161729073278636154); //id role mute
                                if let Some(guild_id) = msg.guild_id {
                                    if let Ok(mut member) = guild_id.member(&ctx.http, *user_id).await {
                                        if let Err(why) = member.add_role(&ctx.http, mute_role_id).await {
                                            println!("não deu pra mutar o mamaco: {:?}", why);
                                        } else {
                                            msg.channel_id.say(&ctx.http, format!("você {} foi mutado por ser um filha da puta, kita do serve", msg.author.id)).await.unwrap();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
pub async fn main() {
    let rule = Rule {
        id: 1,
        guild_id: GuildId::new(123456789), //id do serve lixo
        name: String::from("Mute por palavras proibidas"),
        creator_id: UserId::new(987654321),//caso queira por id do criador da rule
        event_type: EventType::MessageSend,
        trigger: Trigger::ForbiddenWords(vec!["motivo:".to_string(), "insulto".to_string()]),
        actions: vec![Action::MuteUser(UserId::new(123456789))], //action de mutar o user
        enabled: true,
        exempt_roles: vec![RoleId::new(987654321)], //cargo imune
        exempt_channels: vec![ChannelId::new(111111111)], //canal sem mute
    };

    let handler = Handler {
        rule: rule.clone(), 
    };
