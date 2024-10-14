use serenity::async_trait;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, Instant};

use serenity::all::Member;

struct RaidProtection {
    member_join_times: Arc<Mutex<HashMap<u64, Vec<Instant>>>>, // Armazena quando os membros entraram.
}

#[async_trait]
impl EventHandler for RaidProtection {
    async fn guild_member_addition(&self, _ctx: Context, new_member: Member) {
        let mut joins = self.member_join_times.lock().unwrap();
        let entry = joins.entry(new_member.guild_id.into()).or_insert_with(Vec::new);
        entry.push(Instant::now());

        if entry.len() > 10 && entry.iter().filter(|&time| *time >= Instant::now() - Duration::from_secs(10)).count() > 5 {
            println!("Possível raid detectada!");
           
        }
    }
}

async fn mute_new_members(ctx: &Context, guild_id: u64, user_id: u64) {
    if let Some(guild) = ctx.cache.guild(guild_id) {
        if let Some(mute_role) = guild.role_by_name("Muted") {
            let _ = guild
                .member(&ctx.http, user_id)
                .await
                .unwrap()
                .add_role(&ctx.http, mute_role.id)
                .await;
        }
    } else {
        println!("Não foi possível encontrar o servidor com ID {}.", guild_id);
    }
}

