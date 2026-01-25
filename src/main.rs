use clap::{Parser};

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
        service: String,
        login: String,
        password: String,
    },
    Remove{
        service: String
    },
    List,
}

fn main() {
    let cli = Cli::parse();
    let mut entries: Vec<Entry> = Vec::new();
    match cli.command {
        Commands::Add { service, login, password } => {
            entries.push(Entry { service, login, password });
        }

        Commands::Remove { service } => {
            for i in 0..=entries.len()-1 {
                match entries.get(i) {
                   Some(entry) => { 
                       if service == entry.service {entries.remove(i);}
                       else {println!("Doesn't have this service in the manager.");}
                       
                   }
                   None => {continue;}
                }
            }
        }

        Commands::List => {
            for entry in entries{
                println!("{}", entry.service)
            }
        }
        
    }
}
