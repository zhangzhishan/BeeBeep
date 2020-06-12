extern crate clap;
extern crate dotenv;

use clap::{App, Arg};
use std::process::{Command, Stdio};
use std::str;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let matches = App::new("beep")
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let default_command = dotenv::var("DEFAULT_COMMAND").unwrap_or_else(|_| "".to_string());
    let command = matches.value_of("command").unwrap_or(&default_command);

    run(&command).await;
}

async fn run(command: &str) {
    teloxide::enable_logging!();
    log::info!("start beebeep");

    let cmd = Command::new("cmd")
        .args(&["/C", command])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let command_error = cmd
        .wait_with_output()
        .expect("failed to read stdout")
        .stdout;

    let token = dotenv::var("TOKEN").expect("No token");
    let bot = Bot::new(token);
    let channel_id = dotenv::var("CHANNELID").expect("No channel id");
    let mut tele_message = String::from("run finished:");
    tele_message.push_str(command);
    tele_message.push_str("\n");
    tele_message.push_str(str::from_utf8(&command_error).unwrap());
    let msg = bot.send_message(channel_id, tele_message);
    msg.send().await.log_on_error().await;
}
