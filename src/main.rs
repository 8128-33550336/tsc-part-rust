use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::{self, Command, Stdio},
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

fn write_tsconfig(paths: Vec<String>, isDeclaration: bool) -> std::io::Result<()> {
    let json = if isDeclaration {
        format!(
            r#"{{ "extends": "./tsconfig", "emitDeclarationOnly": true, "include": ["{}"] }}"#,
            paths.join("\", \"")
        )
    } else {
        format!(
            r#"{{ "extends": "./tsconfig", "include": ["{}"] }}"#,
            paths.join("\", \"")
        )
    };
    write(json, &Path::new(TSCONFIG_PATH))
}
fn remove_tsconfig() {
    fs::remove_file(TSCONFIG_PATH).expect("remove tsconfig error");
}
fn main() {
    let mut files: Vec<String> = Vec::new();
    let mut isDeclaration = true;
    for arg in std::env::args() {
        if arg.starts_with("-") {
            for char in arg.chars().skip(1) {
                match char {
                    'd' => {
                        // declaration
                        isDeclaration = true;
                    }
                    'e' => {
                        // emit
                        isDeclaration = false;
                    }
                    _ => {
                        eprintln!("unknown command option \"{}\"", char);
                        process::exit(1);
                    }
                }
            }
        } else {
            files.push(arg);
        }
    }
    write_tsconfig(files, isDeclaration).expect("write tsconfig error");
    tsc();
    remove_tsconfig();
}
