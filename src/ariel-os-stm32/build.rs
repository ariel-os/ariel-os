use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // handle CONFIG_SWI
    {
        let dest_path = Path::new(&out_dir).join("swi.rs");
        if let Ok(var) = env::var("CONFIG_SWI") {
            fs::write(
                &dest_path,
                format!("ariel_os_embassy_common::executor_swi!({var});\n").as_bytes(),
            )
            .expect("write failed");
        } else {
            fs::write(
                &dest_path,
                b"compile_error!(\"swi.rs included but CONFIG_SWI not set!\");\n",
            )
            .expect("write failed");
        }

        println!("cargo::rerun-if-env-changed=CONFIG_SWI");
    }

    peripheral_cfg_from_metapac();
}

// Enable peripheral `cfg` flags, data taken from `stm32-metapac`.
// Similar to https://github.com/embassy-rs/embassy/blob/ef32187ed7349f3883d997b6f1590e11dbc8db81/embassy-stm32/build.rs#L38-L43
fn peripheral_cfg_from_metapac() {
    use std::collections::HashSet;
    fn cfg_only_once(seen: &mut HashSet<String>, cfg: &str) {
        if seen.insert(cfg.to_string()) {
            println!("cargo::rustc-cfg={cfg}");
        }
    }
    let mut seen = HashSet::new();
    for p in stm32_metapac::metadata::METADATA.peripherals {
        if let Some(r) = &p.registers {
            cfg_only_once(&mut seen, r.kind);
            cfg_only_once(&mut seen, &format!("{}_{}", r.kind, r.version));
        }
    }
}
