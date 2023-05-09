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
use rand::Rng;

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
        if msg.content == "!help" && !msg.author.bot {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            println!("help call deteceted.");
            if let Err(why) = msg.channel_id.say(&ctx.http, "Not fully developed, please wait for future version!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "김범준" && !msg.author.bot {
            println!("김범준 call deteceted.");
            if let Err(why) = msg.channel_id.say(&ctx.http, "죽어").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "히히" && !msg.author.bot {
            println!("히히 call deteceted.");
            if let Err(why) = msg.channel_id.say(&ctx.http, "히히").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "ㅋㅋ" && !msg.author.bot {
            println!("ㅋㅋ call deteceted.");
            if let Err(why) = msg.channel_id.say(&ctx.http, "ㅋㅋ").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "심심하다" && !msg.author.bot {
            println!("심심하다 call deteceted.");
            if let Err(why) = msg.channel_id.say(&ctx.http, "스타레일 해").await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "뭐먹지" && !msg.author.bot {
            let ran_num = rand::thread_rng().gen_range(0..11);
            let food = match ran_num {
                0 => "짜파게티", 1 => "고르곤졸라 파스타", 2 => "삼겹살",
                3 => "피자", 4 => "옥수수스프", 5 => "냉면", 6 => "토스트",
                7 => "시리얼", 8 => "김치볶음밥", 9 => "부대찌개", _ => "술",
            };
            println!("뭐먹지 call deteceted, recommended {}.", food);
            if let Err(why) = msg.channel_id.say(&ctx.http, food).await {
                println!("Error sending message: {:?}", why);
            }
        }
        else if msg.content == "?" && !msg.author.bot {
            println!("? call deteceted.");
            if let Err(why) = msg.channel_id.say(&ctx.http, "?").await {
                println!("Error sending message: {:?}", why);
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