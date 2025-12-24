//! Minimal TUI scaffold kept separate from core. Built only with the `tui` feature.
use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use colon_sim::service;

#[derive(Default)]
struct AppState {
    runs: Vec<service::RunInfo>,
    status: String,
    selected: usize,
    logs: Vec<String>,
    metrics: Vec<String>,
    datagen_pid: Option<u32>,
    train_pid: Option<u32>,
}

fn main() -> io::Result<()> {
    if let Err(err) = run_app() {
        // ensure raw mode is cleared on failure
        let _ = disable_raw_mode();
        eprintln!("TUI error: {err}");
    }
    Ok(())
}

fn run_app() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    let mut state = AppState {
        runs: vec![
            "(placeholder) run_123".into(),
            "(placeholder) run_456".into(),
        ],
        status: "Press q to quit".into(),
        selected: 0,
        logs: Vec::new(),
        metrics: Vec::new(),
        datagen_pid: None,
        train_pid: None,
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
    Ok(())
}

fn handle_key(code: KeyCode, state: &mut AppState) -> io::Result<bool> {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
        KeyCode::Char('r') => {
            match service::list_runs(std::path::Path::new("assets/datasets/captures")) {
                Ok(runs) => {
                    state.runs = runs
                        .into_iter()
                        .map(|r| r.path.display().to_string())
                        .collect();
                    state.status = "Refreshed runs".into();
                }
                Err(err) => state.status = format!("List runs failed: {err}"),
            }
        }
        KeyCode::Char('d') => {
            let opts = service::DatagenOptions {
                output_root: std::path::Path::new("assets/datasets/captures").to_path_buf(),
                seed: None,
                max_frames: None,
                headless: true,
                prune_empty: false,
                prune_output_root: None,
            };
            match service::datagen_command(&opts).and_then(|cmd| service::spawn(&cmd)) {
                Ok(child) => {
                    state.datagen_pid = Some(child.id());
                    state.status = format!("Started datagen (pid {})", child.id());
                }
                Err(err) => state.status = format!("Datagen start failed: {err}"),
            }
        }
        KeyCode::Char('m') => {
            match service::read_metrics(std::path::Path::new("checkpoints/metrics.jsonl"), Some(1))
            {
                Ok(mut rows) if !rows.is_empty() => {
                    let last = rows.pop().unwrap();
                    state.status = format!("Last metric: {}", last);
                    state.metrics = vec![last.to_string()];
                }
                Ok(_) => state.status = "No metrics found".into(),
                Err(err) => state.status = format!("Read metrics failed: {err}"),
            }
        }
        KeyCode::Char('l') => {
            match service::read_log_tail(std::path::Path::new("logs/train.log"), 5) {
                Ok(lines) => {
                    state.logs = lines;
                    state.status = "Tailed logs (last 5 lines)".into();
                }
                Err(err) => state.status = format!("Read log failed: {err}"),
            }
        }
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
    if let Ok(mut rows) =
        service::read_metrics(std::path::Path::new("checkpoints/metrics.jsonl"), Some(1))
    {
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
    if let Ok(lines) = service::read_log_tail(std::path::Path::new("logs/train.log"), 5) {
        state.logs = lines;
    }
}

fn draw_ui(f: &mut ratatui::Frame<CrosstermBackend<std::io::Stdout>>, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(5), Constraint::Length(8)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = state
        .runs
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let label = r.path.display().to_string();
            let label = if i == state.selected {
                format!("> {label}")
            } else {
                format!("  {label}")
            };
            ListItem::new(Spans::from(Span::raw(label)))
        })
        .collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Runs"));
    f.render_widget(list, chunks[0]);

    let mut status_lines = vec![state.status.clone()];
    if let Some(pid) = state.datagen_pid {
        let alive = service::is_process_running(pid);
        status_lines.push(format!(
            "datagen pid: {pid} [{}]",
            if alive { "running" } else { "stopped" }
        ));
    }
    if let Some(pid) = state.train_pid {
        let alive = service::is_process_running(pid);
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
    if let Some(detail) = selected_run_detail(state) {
        status_lines.push("Selected run:".into());
        status_lines.extend(detail);
    }
    let help = Paragraph::new(status_lines.join("\n"))
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(help, chunks[1]);
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
