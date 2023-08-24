#![windows_subsystem = "windows"]

use std::env;
use std::os::windows::process::CommandExt;
use std::process;

use urlencoding::encode;

fn launch_search(url: &str) {
    process::Command::new("cmd")
        .creation_flags(0x08000000)
        .arg("/c")
        .arg("start")
        .arg(url)
        .output()
        .unwrap();
}

fn google_search(keyword: &str) {
    let url = format!("https://www.google.com/search?q={}", keyword);
    launch_search(&url.as_str());
}

fn duckduckgo_search(keyword: &str) {
    let url = format!("https://www.duckduckgo.com/?q={}", keyword);
    launch_search(&url.as_str());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let service = if args.len() > 1 {
        if args.len() >= 2 {
            &args[1]
        } else {
            "g"
        }
    } else {
        println!("Error, not enough params provided!");
        process::exit(-1);
    };

    let keyword = args[2..args.len()].join(" ");
    let keyword = encode(&keyword.as_str());

    match service {
        service if ["g", "google"].contains(&service) => google_search(&keyword),
        service if ["d", "duckduckgo"].contains(&service) => duckduckgo_search(&keyword),
        _ => google_search(&encode(&args[1..args.len()].join(" "))),
    }
}
