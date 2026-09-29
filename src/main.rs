#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    topcoat::start(clone_tabnews::pages::router())
        .await
        .unwrap();
}
