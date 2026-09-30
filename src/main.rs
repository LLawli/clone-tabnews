#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env.development").ok();
    topcoat::start(clone_tabnews::pages::router())
        .await
        .unwrap();
}
