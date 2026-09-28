mod pages;

#[tokio::main]
async fn main() {
    topcoat::start(pages::router()).await.unwrap();
}
