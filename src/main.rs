use clap::{arg, Parser};
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long)]
    number: Option<u32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let requests = parse(args.path);

    if args.number.is_some() {
        if args.number.unwrap() >= requests.len() as u32 {
            println!("No command at the matching index");
            return;
        }

        let output = async_process::Command::new("bash")
            .arg("-c")
            .arg(requests[args.number.unwrap() as usize].as_str())
            .output()
            .await
            .unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = std::str::from_utf8(&output.stderr).unwrap();

        println!("{}", stdout);
        eprintln!("{}", stderr);
    } else {
        for (i, request) in requests.iter().enumerate() {
            println!("{}: {}", i, request);
        }
    }
}

fn parse(path: String) -> Vec<String> {
    let contents = std::fs::read_to_string(path).unwrap();
    let pattern = Regex::new(r"#{3,}").unwrap();
    let commands: Vec<String> = pattern
        .split(contents.as_str())
        .filter_map(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        })
        .collect();

    return commands;
}
