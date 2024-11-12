use crate::journal::{JournalEntry, Storage};
use clap::{Parser, Subcommand};
use std::io;
use std::io::Result;

#[derive(Parser, Debug)]
#[command(name = "skald")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Write { title: Option<String> },
    Read { title: String },
    List,
    Delete { title: String },
}

impl Commands {
    pub fn execute(&self, storage: &mut Storage) -> Result<()> {
        match self {
            Commands::Write { title } => {
                let title: String = title.clone().unwrap_or_else(|| {
                    println!("Enter title:");
                    let mut title: String = String::new();
                    io::stdin().read_line(&mut title).unwrap();
                    title.trim().to_string()
                });

                let content: String = get_user_input("Enter content:");

                let entry: JournalEntry = JournalEntry::new(&title, &content);
                storage.save_entry(&entry)?;
                Ok(())
            }
            Commands::Read { title } => {
                let entry: JournalEntry = storage.load_entry(title)?;
                println!("Title: {}", entry.title());
                println!("Content: {}", entry.content());
                Ok(())
            }
            Commands::List => {
                let entries: Vec<String> = storage.list_entries()?;
                for entry in entries {
                    println!("* {}", entry);
                }
                Ok(())
            }
            Commands::Delete { title } => {
                storage.delete_entry(title)?;
                println!("Entry deleted: {}", title);
                Ok(())
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input: String = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
