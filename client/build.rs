use std::process::Command;

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    // Capture last 3 git commit short hashes and messages, separated by "||"
    let output = Command::new("git")
        .args(["log", "--oneline", "-1"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .lines()
                .collect::<Vec<_>>()
                .join("||")
        })
        .unwrap_or_default();

    println!("cargo:rustc-env=GIT_RECENT_COMMITS={}", output);
}
