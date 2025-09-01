#! /bin/sh

# See https://doc.rust-lang.org/rustc/instrument-coverage.html

laze build -s nightly -DCARGO_ARGS+='--locked' --builders host  --global -m --keep-going=0 -s coverage coverage 
laze -C examples/hello-world build -b native -s coverage run

# add doctests to the objects

for file in target/debug/doctestbins/*/rust_out build/bin/native/cargo/x86_64-unknown-linux-gnu/release/hello-world ;
do 
  if [ -f "$file" ]; then
    printf "%s %s " -object $file >> build/profile/objects;
  fi
done

RUSTUP_TOOLCHAIN=$(scripts/rust-toolchain.sh)
sysroot=$(rustc --print sysroot)

profdata=$(find $sysroot -name "llvm-profdata")
cov=$(find $sysroot -name "llvm-cov")


objects=$(cat build/profile/objects)

${profdata} merge -sparse build/profile/*.profraw -o build/profile/output.profdata  

${cov} export ${objects} -Xdemangler=rustfilt -instr-profile=build/profile/output.profdata --ignore-filename-regex='/.cargo' --ignore-filename-regex='rustc/' --ignore-filename-regex='/.rustup' --ignore-filename-regex='/target' -format=lcov > build/profile/coverage.txt
# ${cov} show ${objects} -Xdemangler=rustfilt -instr-profile=build/profile/output.profdata --ignore-filename-regex='/.cargo' --ignore-filename-regex='rustc/' --ignore-filename-regex='/.rustup' --ignore-filename-regex='/target'  -format=html -output-dir build/profile_html