extern crate dirs;
use openapi::apis::{configuration::Configuration, default_api as twilio_api};
use std::io::Read;
use std::fs::File;

#[allow(unused)]
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Call { from: String, to: String },
    Text { from: String, to: String },
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // 3 keys are required to run 
    let mut account_sid = String::new();
    let mut api_key_sid = String::new();
    let mut api_key_secret = String::new();

    // dirs::home_dir() gets the home directory for any OS
    // error may occur on Windows.
    let home = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
    let path_to_key = format!("{}/ServiceTitan/Keys/keys.config", home);
    let mut file = File::open(path_to_key)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // loop through keys.config to get keys
    for line in content.lines() {
        if line.contains("twilioapikeys/prod/accountsid") {
            account_sid = line.split("\"").nth(3).unwrap().to_string();
        }else if line.contains("twilioapikeys/prod/apikeysid") {
            api_key_sid = line.split("\"").nth(3).unwrap().to_string();
        }else if line.contains("twilioapikeys/prod/apikeysecret") {
            api_key_secret = line.split("\"").nth(3).unwrap().to_string();
        }
    }

    // arg1 = subcommand, arg2 = from, arg3 = to
    let args = Cli::parse(); 

    // authenticate
    let mut twilio_config = Configuration::default();
    twilio_config.basic_auth = Some((api_key_sid, Some(api_key_secret)));

    match &args.command {
        Commands::Call { from, to } => {
            println!("Twilio call test: {} -> {}", &from, &to);
            test_call(&twilio_config, &account_sid, &from, &to).await;
        },
        Commands::Text { from, to } => {
            println!("Twilio SMS test: {} -> {}", &from, &to);
            test_sms(&twilio_config, &account_sid, &from, &to).await;
        },
    }

    Ok(())
}

// cmm text <from> <to>
async fn test_sms(twilio_config: &Configuration, account_sid: &str, from: &str, to: &str) {

    let body = "Twilio SMS test: SUCCESS".to_string();

    let message = twilio_api::create_message(
        &twilio_config,
        &account_sid,
        &to,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&from),
        None,
        Some(&body),
        None,
    )
    .await;

    //let result = message.expect("Twilio SMS test: FAIL");

    match message {
        Ok(_) => println!("Twilio SMS test: SUCCESS"),
        Err(_) => println!("Twilio SMS test: FAIL"),
    };
}

// cmm call <from> <to>
async fn test_call(twilio_config: &Configuration, account_sid: &str, from: &str, to: &str) {

    let url = "http://demo.twilio.com/docs/voice.xml".to_string();

    let call = twilio_api::create_call(
        &twilio_config,
        &account_sid,
        &to,
        &from,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&url), // url required
        None,
        None,
    ).await;

    //let result = call.expect("Twilio call test failed...");

    match call {
        Ok(_) => println!("Twilio call test: SUCCESS"),
        Err(_) => println!("Twilio SMcallS test: FAIL"),
    };
}