use std::fs;
use std::path::PathBuf;

use cortenforge_tools::ToolConfig;

fn write_temp_config(contents: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "cortenforge-tools-test-{}.toml",
        std::process::id()
    ));
    fs::write(&path, contents).expect("write temp config");
    path
}

#[test]
fn loads_minimal_config() {
    let path = write_temp_config("sim_bin = \"sim_view\"\n");
    let cfg = ToolConfig::from_path(&path).expect("load config");
    assert_eq!(cfg.sim_bin, PathBuf::from("sim_view"));
    let _ = fs::remove_file(&path);
}
