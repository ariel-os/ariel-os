use picoserve::{AppBuilder, response::File, routing::get_service};

#[cfg(feature = "button-reading")]
use picoserve::{
    response::{IntoResponse, Json},
    routing::get,
};

pub struct AppB;

pub type AppRouter = <AppB as AppBuilder>::PathRouter;

impl AppBuilder for AppB {
    type PathRouter = impl picoserve::routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        let router = picoserve::Router::new().route(
            "/",
            get_service(File::html(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static/index.html",
            )))),
        );

        #[cfg(feature = "button-reading")]
        let router = router.route("/button", get(button));

        router
    }
}

#[cfg(feature = "button-reading")]
pub async fn button() -> impl IntoResponse {
    Json(JsonButton {
        button: crate::BUTTON_INPUT.get().await.is_low(),
    })
}

#[cfg(feature = "button-reading")]
#[derive(serde::Serialize)]
struct JsonButton {
    button: bool,
}
