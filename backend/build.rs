use std::{env, path::PathBuf, process};
use wasmbl::bundler::WebBundlerOpt;

fn main() {
    bundle_frontend();
}

fn bundle_frontend() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("expected OUT_DIR to be set by Cargo"));

    let opt = WebBundlerOpt {
        src_dir: "../frontend".into(),
        dist_dir: out_dir.join("ui"),
        tmp_dir: out_dir.join("tmp"),
        base_url: Some("/".into()),
        wasm_version: env::var("CARGO_PKG_VERSION")
            .expect("expected CARGO_PKG_VERSION to be set by Cargo"),
        release: env::var("PROFILE").expect("expected PROFILE to be set by Cargo") != "debug",
        workspace_root: "..".into(),
        additional_watch_dirs: Vec::new(),
    };

    if let Err(why) = wasmbl::bundler::run(opt) {
        println!("Failed to build frontend. Error: {}", why);
        process::exit(1);
    }
}
