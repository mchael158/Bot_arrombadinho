use serenity::{
    async_trait,
    model::{channel::Message, gateway::{GatewayIntents, Ready}},
    prelude::*,
};
use serenity::all::GuildId;

mod automod;
mod banuser;
mod embediss;
mod constantes;

use constantes::{GUILD_i_ID, CHANNEL_BAGUNCA, BAN_COMMAND};
// Cabeça do Handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        //------------------------------------------------------//
        if let Some(rule_triggered) = automod::check_automod_rules(&msg).await {     
            if let Err(why) = automod::execute_automod_actions(ctx.clone(), msg.clone(), rule_triggered).await {
                println!("Erro ao executar ação do automod: {:?}", why);
            }
            return;
        }
        
        if msg.content == HELP_MESSAGE {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Erro no comando de ajuda: {:?}", why);
            }
        } else if msg.content.starts_with(BAN_COMMAND) {
            if let Some(_guild_id) = msg.guild_id {
                let guild_id = GuildId::new(GUILD_i_ID);
                if let Err(why) = banuser::ban_user(&ctx, &msg, guild_id).await {
                    println!("Erro ao tentar banir o usuário: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("Tyranus está online");
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
        .expect("Erro ao criar o cliente");

    if let Err(why) = client.start().await {
        println!("Erro no cliente: {:?}", why);
    }
}
