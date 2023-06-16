use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

const TSCONFIG_PATH: &str = "tmpTsconfig.json";

fn write(content: String, path: &Path) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}

fn tsc() {
    Command::new("tsc")
        .arg("--project")
        .arg(TSCONFIG_PATH)
        .stderr(Stdio::piped())
        .spawn()
        .expect("tsc error")
        .wait()
        .expect("tsc error");
}

fn write_tsconfig(paths: Vec<String>) -> std::io::Result<()> {
    let json = format!(
        "{}{}{}",
        "{ \"extends\": \"./tsconfig\", \"include\": [\"",
        paths.join("\", \""),
        "\"] }"
    );
    write(json, &Path::new(TSCONFIG_PATH))
}
fn remove_tsconfig() {
    fs::remove_file(TSCONFIG_PATH).expect("remove tsconfig error");
}
fn main() {
    let files: Vec<String> = std::env::args().collect();
    write_tsconfig(files).expect("write tsconfig error");
    tsc();
    remove_tsconfig();
}
