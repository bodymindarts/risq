use prost_build;

use std::{env, fs, path::PathBuf};

fn main() {
    generate_protocols();
}

fn generate_protocols() {
    let mut config = prost_build::Config::new();
    config
        .type_attribute("NodeAddress", "#[derive(Eq, Hash)]")
        .type_attribute(
            ".",
            "#[cfg_attr(feature = \"dummy-seed\", derive(serde::Serialize, serde::Deserialize))]",
        )
        .extern_path(".risq.custom", "crate::bisq::payload::custom_messages")
        .compile_protos(&protocol_files(), &protocol_includes())
        .expect("Error compiling protobuf definitions");
    for file in generated_files() {
        fs::copy(
            &file,
            // NB: src/generated is presumed to exist; if you delete
            // it, this'll fail.
            format!(
                "src/generated/{}",
                file.file_name().unwrap().to_string_lossy()
            ),
        )
        .expect(
            "Could not copy \
             generated code to \
             src/generated",
        );
    }
}

fn generated_files() -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in fs::read_dir(env::var("OUT_DIR").unwrap()).unwrap() {
        let file = entry.unwrap();
        if file.file_name().to_str().unwrap().ends_with(".rs") {
            if file.metadata().unwrap().is_file() {
                files.push(file.path());
            }
        }
    }
    files
}

fn protocol_includes() -> Vec<String> {
    vec!["proto/custom".to_string(), "proto/bisq".to_string()]
}

fn protocol_files() -> Vec<String> {
    let mut files = vec![];
    for entry in fs::read_dir("proto/bisq").unwrap() {
        let file = entry.unwrap();
        // skip vim temp files
        if file.file_name().to_str().unwrap().starts_with(".") {
            continue;
        }
        if file.metadata().unwrap().is_file() {
            files.push(file.path().to_string_lossy().into_owned());
        }
    }
    files
}
