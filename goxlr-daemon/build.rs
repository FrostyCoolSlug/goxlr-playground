use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // This could probably use some work, but for now it does the job
    let mut command = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("npm")
    };

    if cfg!(target_os = "windows") {
        command.args(["/C", "npm"]);
    }

    command
        .arg("install")
        .current_dir("../goxlr-webui")
        .output()
        .expect("Failed to Run npm install");

    let mut command = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("npm")
    };

    if cfg!(target_os = "windows") {
        command.args(["/C", "npm"]);
    }
    command
        .arg("run")
        .arg("build")
        .current_dir("../goxlr-webui")
        .output()
        .expect("Failed to run npm build");

    let content = Path::new("./web-content");
    if content.exists() {
        fs::remove_dir_all(content).expect("Error Deleting Directory!");
    } else if cfg!(not(target_os = "windows")) {
        fs::create_dir(content).expect("Er?");
    }
    fs::rename("../goxlr-webui/dist/", content).expect("BLARP");
}
