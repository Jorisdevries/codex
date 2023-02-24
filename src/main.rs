use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Serialize, Deserialize)]
struct JournalEntry {
    timestamp: DateTime<Utc>,
    entry: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: journal <entry> or journal -n <number>");
        std::process::exit(1);
    }

    if args[1] == "-n" {
        let n = args
            .get(2)
            .expect("Expected a number after the -n flag")
            .parse::<usize>()
            .expect("Expected a number after the -n flag");

        print_last_n_entries(n);
    } else {
        let entry = args[1..].join(" ");
        write_journal_entry(&entry);
    }
}

fn get_journal_path() -> String {
    let home_dir = dirs::home_dir().expect("Couldn't find home directory");
    let mut path = home_dir.to_str().unwrap().to_owned();
    path.push_str("/journal.json");
    path
}

fn write_journal_entry(entry: &str) {
    let path = get_journal_path();
    let entry = JournalEntry {
        timestamp: Utc::now(),
        entry: entry.to_owned(),
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("Couldn't open journal file");

    let entry_json =
        serde_json::to_string(&entry).expect("Couldn't serialize journal entry to JSON");

    writeln!(file, "{}", entry_json).expect("Couldn't write to journal file");
}

fn print_last_n_entries(n: usize) {
    let path = get_journal_path();

    let file = File::open(path).expect("Couldn't open journal file");
    let reader = BufReader::new(file);

    let entries: Vec<JournalEntry> = reader
        .lines()
        .map(|line| {
            serde_json::from_str::<JournalEntry>(&line.expect("Couldn't read line from file"))
                .expect("Couldn't deserialize journal entry")
        })
        .collect();

    let num_entries = entries.len();
    let entries_to_print = std::cmp::min(n, num_entries);

    for i in (num_entries - entries_to_print)..num_entries {
        let entry = &entries[i];
        println!(
            "{} - {}",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            entry.entry
        );
    }
}

