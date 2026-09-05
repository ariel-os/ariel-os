cfg_select! {
    context = "rp2040" => {
        mod identity_rp2040;

        pub use identity_rp2040::*;
    }
    context = "rp235xa" => {
        mod identity_rp235x;

        pub use identity_rp235x::*;
    }
    _ => {
        compile_error!("this RP chip is not supported");
    }
}
