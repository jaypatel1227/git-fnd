use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::thread;
// use std::sync::{Arc, Mutex};

fn main() {
    let args = Args::parse();
    let multi_thread = false;
    // TODO: Implement the multithreaded approach
    if multi_thread {
        let cores: usize = std::thread::available_parallelism().unwrap().into();
        let search_info = vec![Vec::<PathBuf>::new(); cores];

        for _ in 0..cores {
            thread::spawn(move || {
                // Lock the mutex
                // let mut vector = shared_vector_clone.lock().unwrap();
                // vector.push(new_path); // Add a path
                // Mutex is automatically unlocked when 'vector' goes out of scope
            });
        }
        println!("{}", search_info.len());
    }

    let mut searcher = package_info(&args.path);

    search_recursive(&searcher.base_path, &mut searcher.repos);
    for path in searcher.repos {
        println!("{}", path.to_str().unwrap_or(""));
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

#[derive(Clone, Debug)]
struct SearchInfo {
    repos: Vec<PathBuf>,
    base_path: PathBuf,
}

impl SearchInfo {
    fn add_path(&mut self, repo_path: PathBuf) {
        self.repos.push(repo_path);
    }
}

fn identify_paths(base: PathBuf, cores: u8) -> Vec<Vec<PathBuf>> {
    let search_info = vec![Vec::<PathBuf>::new(); cores.into()];
    let cores_filled = 0;
    if !base.is_dir() {
        return Vec::new();
    }
    while cores_filled < cores {
        // TODO: read the subdirectores until you have some worload split between the threads
    }

    search_info
}

fn package_info(path: &PathBuf) -> SearchInfo {
    SearchInfo {
        repos: Vec::new(),
        base_path: path.to_path_buf(),
    }
}

fn search_recursive<'a>(path: &PathBuf, repos: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let sub_path = entry.path();
                if let Some(file_name) = sub_path.file_name() {
                    if sub_path.is_dir() {
                        if file_name == ".git" {
                            repos.push(path.clone());
                        }
                        search_recursive(&sub_path, repos);
                    }
                }
            }
        }
    }
}
