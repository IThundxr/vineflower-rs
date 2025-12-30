use cargo_metadata::MetadataCommand;
use std::error::Error;
use std::path::Path;
use std::{env, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let metadata = MetadataCommand::new()
        .manifest_path(env!("CARGO_MANIFEST_PATH"))
        .exec()?;

    let dependency_metadata = &metadata.root_package().unwrap().metadata["dependency"];

    let maven_url = dependency_metadata["mavenUrl"].as_str().unwrap();
    let group_id = dependency_metadata["groupId"]
        .as_str()
        .unwrap()
        .replace(".", "/");
    let artifact_id = dependency_metadata["artifactId"].as_str().unwrap();
    let version = dependency_metadata["version"].as_str().unwrap();

    let file_name = format!("{artifact_id}-{version}.jar");
    let path = format!("{maven_url}/{group_id}/{artifact_id}/{version}/{file_name}");

    let data = reqwest::blocking::get(path)?.bytes()?;

    let out_jar = out_dir.join(file_name);

    fs::write(&out_jar, data)?;

    println!(
        "cargo:rustc-env=BUNDLED_JAR_PATH={}",
        out_jar.to_string_lossy()
    );

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
