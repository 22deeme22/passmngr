use serde::{Serialize, Deserialize};
use std::{fs, u8};
use serde_json::{from_str, to_string_pretty};
use clap::{Parser};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce, aead::{Aead, AeadCore, KeyInit, OsRng}
};

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
    /// Add an entry of service (-s), login (-l) and password (-p) specified.
    Add {
        #[arg(short)]
        service: String,
        #[arg(short)]
        login: String,
        #[arg(short)]
        password: String,
    },
    /// Delete the entry of the service specified.
    Remove{
        service: String
    },
    /// List every entry.
    List,
}

fn main() {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
 
    
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { service, login, password } => {

                                                                      
            // Take the vector of entry that is already in the file
            let data = fs::read("data/data.json").expect("1");
            let mut entries = Vec::new();
            if !data.is_empty() {
                entries = decrypt(&data, &cipher);
            } 
            
            // Add the entry that the user wrote to the vector
            entries.push(Entry{ service, login, password});

            let encrypted = encrypt(&entries, &cipher);
           
            fs::write("data/data.json", encrypted).expect("6");
        }

        Commands::Remove { service } => {
            let data = fs::read("data/data.json").expect("1");
            
            let mut entries = decrypt(&data, &cipher);
        
            // Keep every entry where the service doesn't correspond to the one that the user want to remove
            entries.retain(|entry| entry.service != service);

            let encrypted = encrypt(&entries, &cipher);

            fs::write("data/data.json", encrypted).expect("6");
        }

        Commands::List => {
            // Take the vector of entry that is already in the file
            let data = fs::read("data/data.json").expect("1");
            
            let entries = decrypt(&data, &cipher);
            // Print the vector
            println!("{:#?}", entries);
        }
    }
}

fn encrypt(entries: &Vec<Entry>, cipher: &ChaCha20Poly1305) -> Vec<u8> {
    let e_nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    // Rewrote the vector with the value added to the data file
    let json = to_string_pretty(&entries).expect("4");
            
    let encrypted = cipher.encrypt(&e_nonce, json.as_bytes()).expect("55555");

    let mut out = Vec::new();
    out.extend_from_slice(&e_nonce);
    out.extend_from_slice(&encrypted);
    out
}


fn decrypt(data: &Vec<u8>, cipher: &ChaCha20Poly1305) -> Vec<Entry> {
    let (d_nonce_bytes, ciphertext) = data.split_at(12);
    let d_nonce = Nonce::from_slice(d_nonce_bytes);
                
    let decrypted = cipher.decrypt(&d_nonce, ciphertext).expect("22"); 
    let json = String::from_utf8(decrypted).expect("2"); 
    from_str::<Vec<Entry>>(&json).expect("3")
}
