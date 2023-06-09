use anyhow::Result;
use clap::{arg, Arg, Command};

mod config;
mod gpt;

use config::Config;
use gpt::GPT;

fn cli() -> Command {
    Command::new("gpt")
        .about("A Chat GPT CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("Initialise GPT cli config")
                .arg(arg!(<KEY> "API_KEY"))
                .arg_required_else_help(true),
        )
        .arg(Arg::new("reset").long("reset"))
        .subcommand(
            Command::new("new")
                .about("New Chat GPT Question")
                .arg(arg!(<QUESTION> "The question to ask"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("change-model")
                .about("Change the model that is being used")
                .arg(arg!(<MODEL> "The model id"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let mut _initialized = config.has_api_key();
    let mut gpt = GPT::new(config);

    let matches = cli().get_matches();

    if let Some((subcommand, sub_matches)) = matches.subcommand() {
        match subcommand {
            "init" => {
                let key = sub_matches.get_one::<String>("KEY").unwrap();
                gpt.update_api_key(key);
                println!("New API key now being used");
            }
            "reset" => {
                gpt.reset_chat_log();
                println!("New Chat started");
            }
            "new" => {
                let question = sub_matches.get_one::<String>("QUESTION").unwrap();
                let res = gpt.ask_question(&question).await?;
                println!("{:?}", res.choices[0].message.content);
            }
            "change-model" => {
                let model = sub_matches.get_one::<String>("MODEL").unwrap();
                gpt.change_model(model);
                println!("Model changed to {}", model);
            }
            _ => cli().print_help()?,
        }
    }

    gpt.save()?;

    Ok(())
}
