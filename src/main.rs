use serde::{Serialize, Deserialize};
use std::fs;
use serde_json::{from_str, from_value, to_string_pretty};
use clap::{Parser};

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    service: String,
    login: String,
    password: String,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Parser)]
enum Commands {
    Add {
        /// Enter service here.
        #[arg(long, short)]
        service: String,
        /// Enter your login.
        #[arg(long, short)]
        login: String,
        /// Enter your password.
        #[arg(long, short)]
        password: String,
    },
    Remove{
        /// Enter service here.
        #[arg(long, short)]
        service: String
    },
    List,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { service, login, password } => {
            let value = fs::read_to_string("data/data.json").expect("error");
            let mut entries = from_str::<Vec<Entry>>(&value).expect("error");
            entries.push(Entry{ service, login, password });
            let json = to_string_pretty(&entries).expect("error");
            fs::write("data/data.json", json).expect("error");
        }

        Commands::Remove { service } => {
            let value = fs::read_to_string("data/data.json").expect("error");
            let mut entries = from_str::<Vec<Entry>>(&value).expect("error");
            entries.retain(|entry| entry.service != service);
            let json = to_string_pretty(&entries).expect("error");
            fs::write("data/data.json", json).expect("error");
        }

        Commands::List => {
            let value = fs::read_to_string("data/data.json").expect("error");
            let entries = from_str::<Vec<Entry>>(&value).expect("error");
            println!("{:#?}", entries);
        }
        
    }
}
