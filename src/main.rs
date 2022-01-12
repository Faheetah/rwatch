use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::time::{SystemTime};
use std::path::{PathBuf, Path};
use std::process::Command;

fn watch(pattern: String, command: String) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(move |res| tx.send(res).unwrap())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    let mut last_time = SystemTime::now();
    let mut last_files: Vec<PathBuf> = Vec::new();
    for res in rx {
        match res {
            Ok(event) => if match_event(&event, &pattern) {
                if SystemTime::now().duration_since(last_time).unwrap().as_millis() > 250 || last_files != event.paths {
                    last_files = event.paths;
                    run_command(&command, &last_files);
                }
                last_time = SystemTime::now();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn match_event(event: &notify::Event, pattern: &String) -> bool {
    let re = Regex::new(pattern).unwrap();
    for path in &event.paths {
        if event.kind.is_modify() && re.is_match(path.to_str().unwrap()) {
            return true
        }
    }
    false
}

fn run_command(command: &String, paths: &Vec<PathBuf>) {
    let joined_paths = paths.iter().fold(String::new(), |s, p| [s, p.to_str().unwrap().to_string()].join(" ").strip_prefix(" ").unwrap().to_string());
    let command = str::replace(command, "{}", &joined_paths);
    let output = Command::new("bash").args(vec!["-c", &command]).output().unwrap();
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());
}

fn main() {
    let pattern = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a pattern");
    let command = std::env::args()
        .nth(2)
        .expect("Argument 2 needs to be a command to run");
    println!("watching {} to run {}", pattern, command);
    if let Err(e) = watch(pattern, command) {
        println!("error: {:?}", e)
    }
}
