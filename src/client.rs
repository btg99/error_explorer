/// The client module serves the static files that makes up the client
/// written in Elm. Elm is a pure functional language that compiles to
/// javascript (app/app.js). 

use rocket::fs::{relative, NamedFile};
use rocket::{get, routes, Route};

pub fn routes() -> Vec<Route> {
    routes![app_html, app_js, styles_css]
}

/// All unknown routes get redirected to the app.html file since navigation
/// is mostly handled by the client.
#[get("/<_..>")]
async fn app_html() -> Option<NamedFile> {
    NamedFile::open(relative!("app/app.html")).await.ok()
}

#[get("/app.js")]
async fn app_js() -> Option<NamedFile> {
    NamedFile::open(relative!("app/app.js")).await.ok()
}

#[get("/styles.css")]
async fn styles_css() -> Option<NamedFile> {
    NamedFile::open(relative!("app/styles.css")).await.ok()
}
