//! Minimal TUI scaffold kept separate from core. Built only with the `tui` feature.
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use serde_json::json;

use cortenforge_tools::{services, ToolConfig};

#[derive(Clone, Copy)]
struct Theme {
    title_fg: Color,
    title_bg: Color,
    border: Color,
    highlight: Color,
    status_fg: Color,
    status_bg: Color,
    controls_fg: Color,
    controls_bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            title_fg: Color::Rgb(235, 240, 255),    // soft white
            title_bg: Color::Rgb(20, 25, 36),       // midnight slate
            border: Color::Rgb(88, 200, 196),       // suave teal edge
            highlight: Color::Rgb(168, 132, 255),   // elegant violet pop
            status_fg: Color::Rgb(230, 230, 230),   // calm text
            status_bg: Color::Rgb(12, 18, 28),      // deep night blue
            controls_fg: Color::Rgb(214, 219, 230), // gentle contrast
            controls_bg: Color::Rgb(28, 34, 48),    // smooth panel
        }
    }
}

struct AppState {
    cfg: ToolConfig,
    runs: Vec<services::RunInfo>,
    status: String,
    selected: usize,
    logs: Vec<String>,
    metrics: Vec<String>,
    datagen_pid: Option<u32>,
    train_pid: Option<u32>,
    train_status: Option<serde_json::Value>,
}

fn main() -> io::Result<()> {
    if let Err(err) = run_app() {
        let _ = disable_raw_mode();
        eprintln!("TUI error: {err}");
    }
    Ok(())
}

fn run_app() -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let cfg = ToolConfig::load();
    let mut state = AppState {
        cfg,
        runs: Vec::new(),
        status: "Press q to quit".into(),
        selected: 0,
        logs: Vec::new(),
        metrics: Vec::new(),
        datagen_pid: None,
        train_pid: None,
        train_status: None,
    };

    loop {
        terminal.draw(|f| draw_ui(f, &state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if handle_key(key.code, &mut state)? {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            tick(&mut state);
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn handle_key(code: KeyCode, state: &mut AppState) -> io::Result<bool> {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
        KeyCode::Char('r') => match services::list_runs(&state.cfg.captures_root) {
            Ok(runs) => {
                state.runs = runs;
                state.status = "Refreshed runs".into();
                state.selected = 0;
            }
            Err(err) => state.status = format!("List runs failed: {err}"),
        },
        KeyCode::Char('d') => {
            let opts = services::DatagenOptions {
                output_root: state.cfg.captures_root.clone(),
                seed: None,
                max_frames: None,
                headless: true,
                prune_empty: true,
                prune_output_root: Some(state.cfg.captures_filtered_root.clone()),
            };
            match services::datagen_command_with_config(&state.cfg, &opts)
                .and_then(|cmd| services::spawn(&cmd))
            {
                Ok(child) => {
                    state.datagen_pid = Some(child.id());
                    state.status = format!(
                        "Started datagen (pid {}), pruning -> {}",
                        child.id(),
                        state.cfg.captures_filtered_root.display()
                    );
                }
                Err(err) => state.status = format!("Datagen start failed: {err}"),
            }
        }
        KeyCode::Char('t') => {
            let status_path = state
                .cfg
                .train_status_paths
                .first()
                .cloned()
                .unwrap_or_else(|| Path::new("logs/train_status.json").to_path_buf());
            let opts = services::TrainOptions {
                input_root: state.cfg.captures_filtered_root.clone(),
                val_ratio: 0.2,
                batch_size: 2,
                epochs: 1,
                seed: Some(42),
                drop_last: false,
                real_val_dir: None,
                status_file: Some(status_path),
            };
            match services::train_command_with_config(&state.cfg, &opts)
                .and_then(|cmd| services::spawn(&cmd))
            {
                Ok(child) => {
                    state.train_pid = Some(child.id());
                    state.status = format!("Started train (pid {})", child.id());
                    state.train_status =
                        Some(json!({"status":"running","epoch":0,"epochs":opts.epochs}));
                }
                Err(err) => state.status = format!("Train start failed: {err}"),
            }
        }
        KeyCode::Char('m') => {
            match services::read_metrics(&state.cfg.metrics_path, Some(1)) {
                Ok(mut rows) if !rows.is_empty() => {
                    let last = rows.pop().unwrap();
                    state.status = format!("Last metric: {}", last);
                    state.metrics = vec![last.to_string()];
                }
                Ok(_) => state.status = "No metrics found".into(),
                Err(err) => state.status = format!("Read metrics failed: {err}"),
            }
        }
        KeyCode::Char('l') => match services::read_log_tail(&state.cfg.train_log_path, 5) {
            Ok(lines) => {
                state.logs = lines;
                state.status = "Tailed logs (last 5 lines)".into();
            }
            Err(err) => state.status = format!("Read log failed: {err}"),
        },
        KeyCode::Up => {
            if state.selected > 0 {
                state.selected -= 1;
            }
        }
        KeyCode::Down => {
            if state.selected + 1 < state.runs.len() {
                state.selected += 1;
            }
        }
        _ => {}
    }
    Ok(false)
}

fn tick(state: &mut AppState) {
    if state.status.is_empty() {
        state.status = "Press q to quit".into();
    }
    if let Ok(mut rows) = services::read_metrics(&state.cfg.metrics_path, Some(1)) {
        if let Some(last) = rows.pop() {
            let epoch = last.get("epoch").and_then(|v| v.as_u64()).unwrap_or(0);
            let val = last
                .get("val_metrics")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .cloned()
                .unwrap_or(last.clone());
            state.metrics = vec![format!("epoch {epoch}: {val}")];
        }
    }
    if let Ok(lines) = services::read_log_tail(&state.cfg.train_log_path, 5) {
        state.logs = lines;
    }
    if let Some(status) = read_train_status(&state.cfg) {
        state.train_status = Some(status);
    }
}

fn read_train_status(cfg: &ToolConfig) -> Option<serde_json::Value> {
    for path in &cfg.train_status_paths {
        if let Some(status) = services::read_status(path) {
            return Some(status);
        }
    }
    None
}

fn draw_ui(f: &mut ratatui::Frame<'_>, state: &AppState) {
    let theme = Theme::default();
    let root = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1), // title
                Constraint::Min(5),    // main content
                Constraint::Length(2), // controls/help
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new(state.cfg.ui_title.as_str())
        .style(Style::default().fg(theme.title_fg).bg(theme.title_bg))
        .alignment(Alignment::Center);
    f.render_widget(title, root[0]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(8)].as_ref())
        .split(root[1]);

    let items: Vec<ListItem> = state
        .runs
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let label = r.path.display().to_string();
            let label = if i == state.selected {
                format!("▶ {label}")
            } else {
                format!("  {label}")
            };
            let style = if i == state.selected {
                Style::default()
                    .fg(theme.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(label).style(style)
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
            .title("Runs"),
    );
    f.render_widget(list, chunks[0]);

    let mut status_lines = vec![state.status.clone()];
    if let Some(pid) = state.datagen_pid {
        let alive = services::is_process_running(pid);
        status_lines.push(format!(
            "datagen pid: {pid} [{}]",
            if alive { "running" } else { "stopped" }
        ));
    }
    if let Some(pid) = state.train_pid {
        let alive = services::is_process_running(pid);
        status_lines.push(format!(
            "train pid: {pid} [{}]",
            if alive { "running" } else { "stopped" }
        ));
    }
    if !state.logs.is_empty() {
        status_lines.push("Log tail:".into());
        status_lines.extend(state.logs.clone());
    }
    if !state.metrics.is_empty() {
        status_lines.push("Metrics:".into());
        status_lines.extend(state.metrics.clone());
    }
    if let Some(s) = &state.train_status {
        status_lines.push("Train status:".into());
        if let Some(epoch) = s.get("epoch").and_then(|v| v.as_u64()) {
            let epochs = s.get("epochs").and_then(|v| v.as_u64()).unwrap_or(0);
            let step = s.get("step").and_then(|v| v.as_u64()).unwrap_or(0);
            let lr = s.get("lr").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let loss = s.get("loss").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let status = s
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            status_lines.push(format!(
                "{} epoch {}/{} step {} loss {:.4} lr {:.3e}",
                status, epoch, epochs, step, loss, lr
            ));
        } else {
            status_lines.push(format!("{}", s));
        }
    }
    if let Some(detail) = selected_run_detail(state) {
        status_lines.push("Selected run:".into());
        status_lines.extend(detail);
    }
    let help = Paragraph::new(status_lines.join("\n"))
        .style(Style::default().fg(theme.status_fg).bg(theme.status_bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
                .title("Status"),
        );
    f.render_widget(help, chunks[1]);

    let controls = Paragraph::new(
        "Controls: [r] refresh runs  [d] datagen  [t] train  [m] metrics tail  [l] log tail  [↑/↓] select  [q]/Esc quit",
    )
    .style(Style::default().fg(theme.controls_fg).bg(theme.controls_bg))
    .alignment(Alignment::Center);
    f.render_widget(controls, root[2]);
}

fn selected_run_detail(state: &AppState) -> Option<Vec<String>> {
    let run = state.runs.get(state.selected)?;
    let mut lines = Vec::new();
    lines.push(format!("path: {}", run.path.display()));
    lines.push(format!(
        "counts: labels={} images={} overlays={}",
        run.label_count, run.image_count, run.overlay_count
    ));
    if let Some(man) = &run.manifest {
        if let Some(seed) = man.seed {
            lines.push(format!("seed: {seed}"));
        }
        if let Some(max) = man.max_frames {
            lines.push(format!("max_frames: {max}"));
            lines.push(format!(
                "progress: {}/{} frames",
                run.label_count.min(max as usize),
                max
            ));
        }
        if let Some(ts) = man.started_at_unix {
            lines.push(format!("started_at_unix: {:.0}", ts));
        }
    }
    Some(lines)
}
