// build.rs
use std::fs;

const MIGRATIONS_DIR: &str = "src/infra/migrations";

fn main() {
    println!("cargo:rerun-if-changed={MIGRATIONS_DIR}");

    let Ok(entries) = fs::read_dir(MIGRATIONS_DIR) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "sql") {
            continue;
        }
        let sql = fs::read_to_string(&path).expect("falha ao ler migration");
        let has_statement = sql
            .lines()
            .map(str::trim)
            .any(|l| !l.is_empty() && !l.starts_with("--"));
        if !has_statement {
            panic!("migration vazia: {}", path.display());
        }
    }
}
