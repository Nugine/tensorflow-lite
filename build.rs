use std::env;
use std::fs;

macro_rules! env_name {
    ($name: ident) => {
        const $name: &str = stringify!($name);
    };
}

fn optional_env(name: &str) -> Option<String> {
    env::var(name).ok()
}

env_name!(TFLITE_RS_LINK);
env_name!(TFLITE_RS_LIB_DIR);

fn get_link() -> &'static str {
    match optional_env(TFLITE_RS_LINK).as_deref() {
        Some("static") => "static",
        Some("dylib") => "dylib",
        None => "dylib",
        Some(s) => panic!(
            r#"The value of env {0} must be "static" or "dylib": {0}={1}"#,
            TFLITE_RS_LINK, s
        ),
    }
}

fn static_link_all_in_dir(lib_dir: &str) {
    let dir = fs::read_dir(lib_dir)
        .unwrap_or_else(|err| panic!("failed to read directory [{}]: {}", lib_dir, err));

    for entry in dir {
        let entry = entry.unwrap();

        if entry.file_type().unwrap().is_file() {
            let file_name = entry
                .file_name()
                .into_string()
                .expect("expected utf-8 path");

            if file_name.is_ascii() && file_name.starts_with("lib") && file_name.ends_with(".a") {
                let len = file_name.len();
                let lib_name = &file_name[3..len - 2];
                assert!(!lib_name.is_empty());
                println!("cargo:rustc-link-lib=static={}", lib_name);
            }
        }
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed={}", TFLITE_RS_LINK);
    println!("cargo:rerun-if-env-changed={}", TFLITE_RS_LIB_DIR);

    let link = get_link();

    println!("cargo:rustc-link-lib={}=tensorflowlite_c", link);

    if let Some(ref lib_dir) = optional_env(TFLITE_RS_LIB_DIR) {
        println!("cargo:rerun-if-changed={}", lib_dir);
        println!("cargo:rustc-link-search=native={}", lib_dir);

        if link == "static" {
            static_link_all_in_dir(lib_dir)
        }
    }
}
