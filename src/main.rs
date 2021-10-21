mod token;
mod data_parser;
mod indexer;

use std::fs;
use std::path::Path;
use std::{env, error::Error};
use std::thread;

use serenity::client::{Client, Context, EventHandler};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::collections::HashMap;

use serenity::client::validate_token;

use crate::token::token_handle::no_token;
use crate::data_parser::data_get::get_file_name;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        // https://media.discordapp.net/attachments/530428251965751318/900302061042802699/unknown.png?width=1177&height=652
        // https://cdn.discordapp.com/attachments/841699787279171584/886427103300583474/

        //msg.content.contains(pat: P)

        // 841699787279171584/886427103300583474/
        // 530428251965751318/900302061042802699/ 

        // 38
        //let mut var = msg.content.clone();
        //println!("{:?}",var.replace("https://cdn.discordapp.com/attachments/",""));
        
        //println!("{}",msg.content);

        if msg.content.contains("https://cdn.discordapp.com/attachments/") {
            thread::spawn(move || {
                get_file_name(msg.content);
            });
        } else if msg.content.contains("https://tenor.com/view/") {
            thread::spawn(move || {
                get_file_name(msg.content);
            });
       } else if msg.content.contains("https://media.discordapp.net/attachments/") {
            thread::spawn(move || {
                get_file_name(msg.content);
            });
       }

    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Inline config
    let bot_prefix: &str = "!";

    //println!("{:?}",checkGecko().await.unwrap());

    println!("Starting");
    let _token_file: &str = "SECRETS";
    println!("Fetching bot token");
    if Path::new(_token_file).exists() == false {
        println!("ERROR: Cannot find Secrets File!");
        println!("Making one for you!");
        no_token(_token_file);
    }

    println!("Found Token File, Reading");
    let _bot_token: String =
        fs::read_to_string(_token_file).expect("ERROR: Unable to get bot token!");
    println!("bot token found\nValidating");

    if validate_token(&_bot_token).is_ok() == false {
        println!("\nToken is not valid! Panicing!");
        panic!("Token Vaild Error");
    } else {
        println!("Token Valid!")
    }

    println!("Starting Client");
    let mut client = Client::builder(_bot_token)
        .event_handler(Handler)
        //.framework(framework)
        .await
        .expect("Error creating client");

    println!("Starting Listener");
    // start listening - 1 shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
        panic!();
    }
}