cfg_select! {
    feature = "http" => {
        pub mod http;
        pub use http::*;
    }
    _ => {
        compile_error!("unsupported transport");
    }
}
