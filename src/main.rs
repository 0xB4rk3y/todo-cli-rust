use std::fs::{OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
}

const FILE: &str = "tasks.json";

fn read_tasks() -> Vec<Task> {
    let mut file = OpenOptions::new().read(true).create(true).open(FILE).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    if content.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&content).unwrap()
    }
}

fn write_tasks(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new().write(true).truncate(true).open(FILE).unwrap();
    let data = serde_json::to_string(tasks).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Komutlar: add <görev>, list, remove <indeks>");
        return;
    }

    let mut tasks = read_tasks();

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Lütfen bir görev girin.");
                return;
            }
            let task = Task {
                description: args[2..].join(" "),
            };
            tasks.push(task);
            write_tasks(&tasks);
            println!("Görev eklendi.");
        }
        "list" => {
            if tasks.is_empty() {
                println!("Hiç görev yok.");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    println!("{} - {}", i + 1, task.description);
                }
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("Silinecek görevin indeksini girin.");
                return;
            }
            if let Ok(index) = args[2].parse::<usize>() {
                if index == 0 || index > tasks.len() {
                    println!("Geçersiz indeks.");
                } else {
                    tasks.remove(index - 1);
                    write_tasks(&tasks);
                    println!("Görev silindi.");
                }
            } else {
                println!("Lütfen geçerli bir sayı girin.");
            }
        }
        _ => println!("Bilinmeyen komut."),
    }
}
