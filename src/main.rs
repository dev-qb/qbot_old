mod commands;
mod message_match;

use serenity::{
    async_trait,
    model::{
        guild::Guild,
        channel::{Message, ChannelType, GuildChannel},
        gateway::Ready,
    },
    framework::standard::StandardFramework,
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // event handler for channel creation
    async fn channel_create(&self, ctx: Context, created_channel: &GuildChannel) {
        println!("Channel creation detected: {:?}", created_channel.name);
        if let Err(why) = created_channel.id.say(&ctx.http, "New Channel!").await {
            println!("Error sending message: {:?}", why);
        }
    }

    // not developed
    async fn channel_delete(&self, ctx: Context, deleted_channel: &GuildChannel) {
        println!("Channel deletion detected.");
        if deleted_channel.name() == "qbot" {
            // Create a new text channel named "qbot"
            if let Ok(channel) = deleted_channel.guild_id.create_channel(&ctx.http, |c| {
                c.name("qbot").kind(ChannelType::Text)
            }).await {
                println!("Recreated channel {:?}", channel);
            }
        }
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: bool) {
        println!("Bot addition to server detected: {:?}", guild.name);
    }

    // Set a handler for the `message` event
    // Event handlers are dispatched through a threadpool, and so multiple events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            let answer = message_match::match_message(msg.content);
            match answer {
                None => return,
                Some(p) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, p).await {
                    println!("Error sending message: {:?}", why);
                    };
                    println!("Sending '{}'", p);
                },
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // loading environment variables(ex. bot token)
    dotenv::dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = std::env::var("DISCORD_TOKEN").expect("Token not loadable from the environment!");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).framework(StandardFramework::new()).event_handler(Handler).await.expect("Error creating client!");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}