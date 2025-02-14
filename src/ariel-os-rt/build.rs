use rustflags::Flag;
use std::env;
use std::path::PathBuf;

fn main() {
    let flags = rustflags::from_env();
    let contexts = flags
        .filter_map(|flag| match flag {
            Flag::Cfg {
                name,
                value: Some(v),
            } if name == "context" => Some(v),
            _ => None,
        })
        .collect::<Vec<_>>();

    let context = |name: &str| contexts.contains(&name.to_string());

    let context_any = |list: &'static [&'static str]| list.iter().find(|entry| context(entry));

    // Put the linker scripts somewhere the linker can find them
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if let Some(context) = context_any(&["cortex-m", "riscv"]) {
        let insert_before = match *context {
            "riscv" => ".trap",
            "cortex-m" => ".data",
            _ => unreachable!(),
        };

        let region = match *context {
            "riscv" => "RWDATA",
            "cortex-m" => "RAM",
            _ => unreachable!(),
        };

        let mut isr_stack_template = std::fs::read_to_string("isr_stack.ld.in").unwrap();
        isr_stack_template = isr_stack_template.replace("${INSERT_BEFORE}", insert_before);
        isr_stack_template = isr_stack_template.replace("${STACK_REGION}", region);
        std::fs::write(out.join("isr_stack.x"), &isr_stack_template).unwrap();
        println!("cargo:rerun-if-changed=isr_stack.ld.in");
    }

    std::fs::copy("linkme.x", out.join("linkme.x")).unwrap();
    std::fs::copy("eheap.x", out.join("eheap.x")).unwrap();
    std::fs::copy("keep-stack-sizes.x", out.join("keep-stack-sizes.x")).unwrap();

    println!("cargo:rerun-if-changed=linkme.x");
    println!("cargo:rerun-if-changed=eheap.x");
    println!("cargo:rerun-if-changed=keep-stack-sizes.x");

    println!("cargo:rustc-link-search={}", out.display());
}
