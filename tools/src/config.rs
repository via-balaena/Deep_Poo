use std::path::{Path, PathBuf};

use serde::Deserialize;

const DEFAULT_CONFIG_NAME: &str = "cortenforge-tools.toml";
const DEFAULT_TRAIN_TEMPLATE: &str =
    "cargo train_hp --model ${MODEL} --batch-size ${BATCH} --log-every ${LOG_EVERY} ${EXTRA_ARGS}";

#[derive(Debug, Clone)]
pub struct ToolConfig {
    pub sim_bin: PathBuf,
    pub train_bin: PathBuf,
    pub assets_root: PathBuf,
    pub captures_root: PathBuf,
    pub captures_filtered_root: PathBuf,
    pub warehouse_manifest: PathBuf,
    pub logs_root: PathBuf,
    pub metrics_path: PathBuf,
    pub train_log_path: PathBuf,
    pub train_status_paths: Vec<PathBuf>,
    pub datagen_args: Vec<String>,
    pub training_args: Vec<String>,
    pub warehouse_train_template: String,
    pub ui_title: String,
}

impl Default for ToolConfig {
    fn default() -> Self {
        let assets_root = PathBuf::from("assets");
        let logs_root = PathBuf::from("logs");
        Self {
            sim_bin: PathBuf::from("sim_view"),
            train_bin: PathBuf::from("train"),
            captures_root: assets_root.join("datasets/captures"),
            captures_filtered_root: assets_root.join("datasets/captures_filtered"),
            warehouse_manifest: assets_root.join("warehouse/manifest.json"),
            assets_root,
            logs_root: logs_root.clone(),
            metrics_path: logs_root.join("metrics.jsonl"),
            train_log_path: logs_root.join("train.log"),
            train_status_paths: vec![
                PathBuf::from("logs/train_hp_status.json"),
                PathBuf::from("logs/train_status.json"),
            ],
            datagen_args: Vec::new(),
            training_args: Vec::new(),
            warehouse_train_template: DEFAULT_TRAIN_TEMPLATE.to_string(),
            ui_title: "CortenForge Tools".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct ToolConfigFile {
    sim_bin: Option<String>,
    train_bin: Option<String>,
    assets_root: Option<String>,
    captures_root: Option<String>,
    captures_filtered_root: Option<String>,
    warehouse_manifest: Option<String>,
    logs_root: Option<String>,
    metrics_path: Option<String>,
    train_log_path: Option<String>,
    train_status_paths: Option<Vec<String>>,
    datagen: Option<ArgSection>,
    training: Option<ArgSection>,
    warehouse: Option<WarehouseSection>,
    ui: Option<UiSection>,
}

#[derive(Debug, Deserialize, Default)]
struct ArgSection {
    args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
struct WarehouseSection {
    train_template: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct UiSection {
    title: Option<String>,
}

impl ToolConfig {
    pub fn load() -> Self {
        if let Ok(path) = std::env::var("CORTENFORGE_TOOLS_CONFIG") {
            let cfg = Self::from_path(Path::new(&path)).unwrap_or_default();
            cfg.warn_if_invalid();
            return cfg;
        }
        let cfg = Self::from_path(Path::new(DEFAULT_CONFIG_NAME)).unwrap_or_default();
        cfg.warn_if_invalid();
        cfg
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        if !path.exists() {
            return None;
        }
        let raw = std::fs::read_to_string(path).ok()?;
        let file: ToolConfigFile = toml::from_str(&raw).ok()?;
        Some(Self::from_file(file))
    }

    fn from_file(file: ToolConfigFile) -> Self {
        let assets_root = file
            .assets_root
            .map(|v| expand_path(&v))
            .unwrap_or_else(|| PathBuf::from("assets"));
        let logs_root = file
            .logs_root
            .map(|v| expand_path(&v))
            .unwrap_or_else(|| PathBuf::from("logs"));

        let captures_root = file
            .captures_root
            .map(|v| expand_path(&v))
            .unwrap_or_else(|| assets_root.join("datasets/captures"));
        let captures_filtered_root = file
            .captures_filtered_root
            .map(|v| expand_path(&v))
            .unwrap_or_else(|| assets_root.join("datasets/captures_filtered"));
        let warehouse_manifest = file
            .warehouse_manifest
            .map(|v| expand_path(&v))
            .unwrap_or_else(|| assets_root.join("warehouse/manifest.json"));

        let metrics_path = file
            .metrics_path
            .map(|v| expand_path(&v))
            .unwrap_or_else(|| logs_root.join("metrics.jsonl"));
        let train_log_path = file
            .train_log_path
            .map(|v| expand_path(&v))
            .unwrap_or_else(|| logs_root.join("train.log"));
        let train_status_paths = file
            .train_status_paths
            .map(|paths| paths.into_iter().map(|v| expand_path(&v)).collect())
            .unwrap_or_else(|| {
                vec![
                    PathBuf::from("logs/train_hp_status.json"),
                    PathBuf::from("logs/train_status.json"),
                ]
            });

        ToolConfig {
            sim_bin: file
                .sim_bin
                .map(|v| expand_path(&v))
                .unwrap_or_else(|| PathBuf::from("sim_view")),
            train_bin: file
                .train_bin
                .map(|v| expand_path(&v))
                .unwrap_or_else(|| PathBuf::from("train")),
            assets_root,
            captures_root,
            captures_filtered_root,
            warehouse_manifest,
            logs_root,
            metrics_path,
            train_log_path,
            train_status_paths,
            datagen_args: file.datagen.and_then(|d| d.args).unwrap_or_default(),
            training_args: file.training.and_then(|t| t.args).unwrap_or_default(),
            warehouse_train_template: file
                .warehouse
                .and_then(|w| w.train_template)
                .unwrap_or_else(|| DEFAULT_TRAIN_TEMPLATE.to_string()),
            ui_title: file
                .ui
                .and_then(|u| u.title)
                .filter(|t| !t.trim().is_empty())
                .unwrap_or_else(|| "CortenForge Tools".to_string()),
        }
    }

    fn warn_if_invalid(&self) {
        if self.sim_bin.as_os_str().is_empty() {
            eprintln!("tools config: sim_bin is empty; sim tools may fail to launch");
        }
        if self.train_bin.as_os_str().is_empty() {
            eprintln!("tools config: train_bin is empty; training tools may fail to launch");
        }
        if self.warehouse_train_template.trim().is_empty() {
            eprintln!(
                "tools config: warehouse.train_template is empty; warehouse_cmd will fail to run"
            );
        }
        if self.train_status_paths.is_empty() {
            eprintln!("tools config: train_status_paths is empty; TUI status will be disabled");
        }
    }
}

fn expand_path(raw: &str) -> PathBuf {
    let mut out = raw.to_string();
    if let Some(stripped) = out.strip_prefix("~") {
        if let Ok(home) = std::env::var("HOME") {
            out = format!("{home}{stripped}");
        }
    }
    PathBuf::from(expand_env(&out))
}

fn expand_env(input: &str) -> String {
    let mut out = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
            if let Some(end) = input[i + 2..].find('}') {
                let key = &input[i + 2..i + 2 + end];
                if let Ok(val) = std::env::var(key) {
                    out.push_str(&val);
                } else {
                    out.push_str(&format!("${{{}}}", key));
                }
                i += end + 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}
