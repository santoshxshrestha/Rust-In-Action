use std::process::Command;

fn main() {
    let output = Command::new("git")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("git succeeded and stdout was:\n{}",s);
    }else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("git failed and stderr was:\n{}",s);
    }

}
