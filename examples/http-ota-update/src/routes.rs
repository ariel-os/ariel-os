use core::cell::RefCell;

use ariel_os::log::{info, Debug2Format};
use picoserve::response::with_state::WithStateUpdate;
use picoserve::response::{self, IntoResponse, IntoResponseWithState, Response, StatusCode};
use picoserve::{AppBuilder as AppBuilderTrait, routing::{get, put}};
use picoserve::extract::State;

use crate::suit;

use alloc::vec::Vec;

pub struct Version {
   pub version: i32,
   pub suit_sequence_number: u64,
}

pub struct AppState {
    pub value: RefCell<i32>,
    pub current_sequence: RefCell<u64>,
}

pub enum ResultBasedResponse<State, T: IntoResponseWithState<State>> {
    Ok((T, core::marker::PhantomData<State>)),
    Err,
}

impl<State, T: IntoResponseWithState<State>> ResultBasedResponse<State, T> {
    fn from_response(response: T) -> Self {
        Self::Ok((response, core::marker::PhantomData))
    }
}

impl<State, T: IntoResponseWithState<State>> IntoResponseWithState<State> for ResultBasedResponse<State, T> {
    async fn write_to_with_state<R: picoserve::io::Read, W: response::ResponseWriter<Error = R::Error>>(
        self,
        state: &State,
        connection: response::Connection<'_, R>,
        response_writer: W,
    ) -> Result<picoserve::ResponseSent, W::Error>
    {
        match self {
            ResultBasedResponse::Ok(r) => r.0.write_to_with_state(state, connection, response_writer).await,
            ResultBasedResponse::Err => {
                Response::empty(StatusCode::BAD_REQUEST).write_to_with_state(state, connection, response_writer).await
            }
        }
    }
}


impl picoserve::extract::FromRef<AppState> for Version {
    fn from_ref<'a>(AppState { value, current_sequence }: &'a AppState) -> Version {
        Self { version: *value.borrow(), suit_sequence_number: *current_sequence.borrow() }
    }
}

async fn get_version(State(version): State<Version>) -> impl IntoResponse {
    picoserve::response::Json(version.version)
}

pub async fn put_manifest(State(version): State<Version>, request_body: Vec<u8>) -> impl IntoResponseWithState<AppState> {
    info!("Received a SUIT Manifest");

    let (manifest, sequence_number) = match suit::build_and_authenticate_manifest(&request_body) {
        Ok((manifest, sequence_number)) => {
            info!("Found update with sequence number: {}", sequence_number);
            (manifest, sequence_number)
        }
        Err(e) => {
            info!("SUIT update rejected: {:?}", Debug2Format(&e));
            return ResultBasedResponse::Err;
        }
    };

    if version.suit_sequence_number >= sequence_number {
        info!("Rejecting SUIT update because the recevied sequence number ({}) is below of equal to the last accepted sequence number ({})", sequence_number, version.suit_sequence_number);
        return ResultBasedResponse::Err;
    }

    let payload = match  suit::fetch_and_verify_update(manifest).await {
        Ok(payload) => payload,
        Err(e) => {
            info!("Fetching and Validating SUIT payload failed: {:?}", Debug2Format(&e));
            return ResultBasedResponse::Err;
        }
    };

    let new_version = match str::from_utf8(&payload).map(|s | s.trim().parse::<i32>()){
            Ok(Ok(v)) => {
                info!("New version number: {:?}", v);
                v
            }
            _ => {
                info!("Bad Payload");
                return ResultBasedResponse::Err;
            }
    };

    ResultBasedResponse::from_response(
        Response::empty(StatusCode::CREATED)
            .with_state_update(async move |state: &AppState| {
                *state.value.borrow_mut() = new_version;
                *state.current_sequence.borrow_mut() = sequence_number
            })
    )

}

pub struct AppBuilder;

pub type AppRouter = <AppBuilder as AppBuilderTrait>::PathRouter;

impl AppBuilderTrait for AppBuilder {
    type PathRouter = impl picoserve::routing::PathRouter;

    fn build_app<'a>(self) -> picoserve::Router<Self::PathRouter> {

        let router = picoserve::Router::new().route(
            "/version",
            get(get_version),
        )
        .route(
            "/suit",
            put(put_manifest)
        )
        .with_state(AppState { value: 1.into(), current_sequence: 0.into() });
        router
    }
}
