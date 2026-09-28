use topcoat::{
    Result,
    router::{Router, RouterBuilderDiscoverExt, Slot, layout, page},
    view::{View, component, view},
};

#[tokio::main]
async fn main() {
    topcoat::start(Router::builder().discover().build())
        .await
        .unwrap();
}

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Inicio"</title>
                topcoat::dev::script()
            </head>
            <body>(slot)</body>
        </html>
    })
}

#[page("/")]
async fn home() -> Result<impl View> {
    Ok(view! { <h1>"Inicio"</h1> })
}

#[page("/hello")]
async fn hello_world() -> Result<impl View> {
    Ok(view! { hello(name: "World") })
}

#[component]
async fn hello(name: &str) -> Result<impl View> {
    Ok(view! { <h1>"Hello, "(name) "!"</h1> })
}
