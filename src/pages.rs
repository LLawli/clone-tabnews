mod api;

use topcoat::{
    Result,
    router::{Router, Slot, layout, module_router, page},
    view::{View, view},
};

pub fn router() -> Router {
    module_router!().build()
}

#[layout]
async fn root_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <html>
            <head>topcoat::dev::script()</head>
            <body>(slot)</body>
        </html>
    })
}

#[page]
async fn root() -> Result<impl View> {
    Ok(view! { <h1>"Inicio"</h1> })
}
