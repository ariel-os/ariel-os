cfg_select! {
    feature = "coap" => {
        pub mod coap;
        pub use coap::*;
    }
    feature = "http" => {
        pub mod http;
        pub use http::*;
    }
    _ => {
        compile_error!("unsupported transport");
    }
}
