extern crate clap;
extern crate dotenv;

use clap::{load_yaml, App};
use std::env;
use std::process::{Command, Stdio};
use std::str;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push(".env");
    dotenv::from_path(path.as_path()).ok();

    if matches.is_present("debug") {
        println!("path {}", path.to_str().unwrap());
        let results: Vec<(String, String)> = dotenv::vars().collect();
        for (key, value) in results {
            println!("{} -- {}", key, value);
        }
    }

    let default_command = dotenv::var("DEFAULT_COMMAND").unwrap_or_else(|_| "".to_string());
    let command = matches.value_of("command").unwrap_or(&default_command);

    let is_telegram = matches.is_present("telegram");

    let noti_message = run(&command);
    if is_telegram {
        notify_telegram(noti_message).await;
    } else {
        // Notify using system.
        notifica::notify("Run finished", &noti_message).ok();
    }
}

fn run(command: &str) -> String {
    teloxide::enable_logging!();
    log::info!("start beebeep");

    let cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap()
    };

    let command_error = cmd
        .wait_with_output()
        .expect("failed to read stdout")
        .stdout;

    let mut noti_message = String::from("run finished:");
    noti_message.push_str(command);
    noti_message.push_str("\n");
    noti_message.push_str(str::from_utf8(&command_error).unwrap());
    noti_message
}

// Notify by telegram.
async fn notify_telegram(noti_message: String) {
    let token = dotenv::var("TOKEN").expect("No token");
    let bot = Bot::new(token);
    let channel_id = dotenv::var("CHANNELID").expect("No channel id");
    let msg = bot.send_message(channel_id, noti_message);
    msg.send().await.log_on_error().await;
}
