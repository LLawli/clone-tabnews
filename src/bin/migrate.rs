#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env.development").ok();
    match clone_tabnews::infra::migrations::run().await {
        Ok(applied) if applied.is_empty() => println!("Nenhuma migration pendente"),
        Ok(applied) => {
            for m in applied {
                println!("aplicada: {m}");
            }
        }
        Err(e) => {
            eprintln!("Falha nas migrations: {e}");
            std::process::exit(1);
        }
    }
}
