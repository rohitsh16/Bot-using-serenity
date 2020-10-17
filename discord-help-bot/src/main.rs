
use std::env;          // for env variables

use serenity :: {
    async_trait,        // for async features
    model::{channel::Message, gateway::Ready},      // two structs
    prelude::*,     
};

const HELLO_MSG: &str = "Hello Everyone!! Welcome to the Channel. 
                        Head to : https://github.com/rohitsh16/Bot-using-serenity
                        and do recommend us any features us you want to add :smile:.";

const HELLO_COMMAND: &str = "!hello";

struct Handler;     

#[async_trait]      // macro
impl EventHandler for Handler {
    async fn message (&self, ctx: Context, msg: Message) {      // async function messsage
        if msg.content == HELLO_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELLO_MSG).await {
                println!("Error Sending Message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {        // prints the name of the bot
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the Environment");

    let mut client = Client::new(&token)
    .event_handler(Handler)
    .await.expect("Error Creating a Client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}