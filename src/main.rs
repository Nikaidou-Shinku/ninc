mod data;
mod tools;
mod commands;

use clap::Parser;
use data::{
  Config,
  Storage,
  CONFIG_FILE,
  STORAGE_FILE,
  args::{Cli, Commands, edu::EduCommands}
};
use tools::{get_user, get_info};
use commands::{login, esrep, ecard, edu, eval::eval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::parse();

  let mut config = match Config::load(CONFIG_FILE).await {
    Ok(config) => { config },
    Err(_) => {
      eprintln!("Failed to load config file, the default value will be used.");
      Config::new()
    }
  };

  let mut storage = match Storage::load(STORAGE_FILE).await {
    Ok(storage) => { storage },
    Err(_) => {
      eprintln!("Failed to load storage file, the default value will be used.");
      Storage::new()
    }
  };

  match args.command {
    Commands::Login(args) => {
      let info = get_info(&config, args.username, args.password);
      if let Some(info) = info {
        if let Err(err) = login(&mut config, &mut storage, &info.0, &info.1, args.save).await {
          eprintln!("Sign in failed!\n{}", err);
        } else {
          println!("Sign in successfully!");
          let userinfo = get_user(&mut storage).await?;
          println!("Welcome, {} {}({})!", userinfo.identity, userinfo.name, userinfo.uid);
        }
      } else {
        eprintln!("Username and password are required.");
      }
    },
    Commands::Esrep(args) => {
      if let Err(err) = esrep(&mut config, &storage, args).await {
        eprintln!("Report failed!\n{}", err);
      } else {
        println!("Report successfully!");
      }
    },
    Commands::Ecard(args) => {
      if let Err(err) = ecard(&config, &storage, args).await {
        eprintln!("Query failed!\n{}", err);
      }
    },
    Commands::Edu(edu_cmd) => {
      match edu_cmd {
        EduCommands::Eval(eval_cmd) => {
          if let Err(err) = eval(&storage, eval_cmd).await {
            eprintln!("Query failed!\n{}", err);
          }
        }
        _ => {
          if let Err(err) = edu(&config, &storage).await {
            eprintln!("Query failed!\n{}", err);
          }
        }
      }
    },
  }

  Ok(())
}
