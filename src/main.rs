use std::path::{Path, PathBuf};
use std::fs;
use std::time::Instant;
use walkdir::{self, WalkDir};

////////// Just counting

// count files using walkdir::WalkDir
fn count_files_walking(path: &Path) -> u32 {
    let mut n: u32 = 0;
    for e in WalkDir::new(path) {
        if let Ok(_) = e {
            n += 1;
        }
    }
    n
}
// counting files without walkdir::WalkDir
fn count_files_running(path: &Path) -> u32 {
    let mut n: u32 = 1;
    let mut dirs: Vec<PathBuf> = Vec::new();
    dirs.push(path.to_path_buf());
    while let Some(dir) = dirs.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for e in entries {
                if let Ok(e) = e {
                    n += 1;
                    if let Ok(ft) = e.file_type() {
                        if ft.is_dir() {
                            dirs.push(e.path());
                        }
                    }
                }
            }
        }
    }
    n
}

////////// Counting visible files

// taken from WalkDir's readme
fn is_wde_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
fn count_visible_files_walking(path: &Path) -> u32 {
    let mut n: u32 = 1;
    let walker = WalkDir::new(path).into_iter();
    for e in walker.filter_entry(|e| !is_wde_hidden(e)) {
        if let Ok(_) = e {
            n += 1;
        }
    }
    n
}
fn is_de_hidden(entry: &fs::DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
fn count_visible_files_running(path: &Path) -> u32 {
    let mut n: u32 = 0;
    let mut dirs: Vec<PathBuf> = Vec::new();
    dirs.push(path.to_path_buf());
    while let Some(dir) = dirs.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for e in entries {
                if let Ok(e) = e {
                    if is_de_hidden(&e) {
                        continue;
                    }
                    n += 1;
                    if let Ok(ft) = e.file_type() {
                        if ft.is_dir() {
                            dirs.push(e.path());
                        }
                    }
                }
            }
        }
    }
    n
}

fn measure(name: &str, dir: &Path, fun: fn(&Path)->u32){
    let start = Instant::now();
    fun(dir);
    println!("{} took {:?}", name, start.elapsed());
}

fn warm_up(path: &Path){
    println!("warming up");
    println!("  count_files_walking. Result={}", count_files_walking(path));
    println!("  count_files_running. Result={}", count_files_running(path));
    println!("  count_visible_files_walking. Result={}", count_visible_files_walking(path));
    println!("  count_visible_files_running. Result={}", count_visible_files_running(path));
}
fn bench(dir: &Path){
    println!("measuring");
    measure("  counting files, walking,", &dir, count_files_walking);
    measure("  counting files, running,", &dir, count_files_running);
    measure("  counting visible files, walking,", &dir, count_visible_files_walking);
    measure("  counting visible files, running,", &dir, count_visible_files_running);
}

fn main() {
    let dir = PathBuf::from("/");
    warm_up(&dir);
    bench(&dir);
}

