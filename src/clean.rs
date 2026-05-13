use std::path::Path;
use walkdir::WalkDir;

pub fn find_bloat() {
    println!("--- looking for garbage ---");
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    
    // this will gonna be slow
    for entry in WalkDir::new(home)
        .max_depth(5) // so it doesnt go too deep
        .into_iter()
        .filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name == "node_modules" || name == "target" {
                    let size = get_size(path);
                    if size > 1024 * 1024 * 1024 { // 1GB
                        println!("{}: {:.2} GB", path.display(), size as f64 / (1024.0 * 1024.0 * 1024.0));
                    }
                }
            }
        }
    println!("done.");
}

fn get_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}
