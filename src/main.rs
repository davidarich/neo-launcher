#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use log::{debug, warn};

mod cache;
mod config;
mod features;
mod views;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let cached_window = cache::load();
    let options = eframe::NativeOptions {
        viewport: {
            let mut vb = egui::ViewportBuilder::default();
            if let Some(c) = &cached_window {
                vb = vb.with_inner_size(c.size()).with_position(c.position());
            }
            vb
        },
        ..Default::default()
    };

    // TODO: add config validation
    match config::load() {
        Ok(cfg) => {
            // TODO: add detection for is running as administrator
            // use winapi crate on windows
            // https://users.rust-lang.org/t/how-do-i-determine-if-i-have-admin-rights-on-windows/35710/7
            if cfg.is_forced_admin {
                debug!("launcher configuration requires elevated privleges");
            }
        }
        Err(e) => {
            warn!("failed to load config: {}", e);
        }
    }

    eframe::run_native(
        "Neo Launcher",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<NeoLauncher>::default())
        }),
    )
}

#[derive(Clone, Debug)]
struct UIState {
    workspace_id: usize,
    active_view: LauncherView,
    last_update: std::time::Instant,
    config_error: Option<String>,
}

impl UIState {
    fn nav_main_menu(&mut self) {
        self.active_view = LauncherView::MainMenu;
    }
    fn nav_workspace_menu(&mut self, workspace_id: usize) {
        self.active_view = LauncherView::WorkspaceMenu;
        self.workspace_id = workspace_id;
    }
    fn nav_config_error(&mut self, error: String) {
        self.active_view = LauncherView::ConfigError;
        self.config_error = Some(error);
    }
}

struct RepaintState {
    started: bool,
    tick_rate: std::time::Duration,
    last_tick: std::time::Instant,
}

struct NeoLauncher {
    rt: tokio::runtime::Runtime,
    config: config::AppConfig,
    ui_state: UIState,
    repaint_state: RepaintState,
    window_cache: Option<cache::WindowCache>,
}

#[derive(Clone, Debug)]
enum LauncherView {
    MainMenu,
    WorkspaceMenu,
    ConfigError,
}

impl Default for NeoLauncher {
    fn default() -> Self {
        let (config, initial_view, config_error) = match config::load() {
            Ok(cfg) => (cfg, LauncherView::MainMenu, None),
            Err(e) => {
                warn!("failed to load config: {}; using default config", e);
                (
                    config::AppConfig::default(),
                    LauncherView::ConfigError,
                    Some(e),
                )
            }
        };
        Self {
            rt: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            config,
            ui_state: UIState {
                active_view: initial_view,
                last_update: std::time::Instant::now(),
                workspace_id: 0,
                config_error,
            },
            repaint_state: RepaintState {
                started: false,
                tick_rate: std::time::Duration::from_secs(60),
                last_tick: std::time::Instant::now(),
            },
            window_cache: None,
        }
    }
}

impl eframe::App for NeoLauncher {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui_state.last_update = std::time::Instant::now();

        // Capture the window geometry so the next launch can restore it.
        if let Some(window) = _frame.winit_window() {
            let scale = window.scale_factor() as f32;
            let size = window.inner_size();
            let current = match window.outer_position() {
                Ok(pos) => cache::WindowCache::capture(
                    size.width as f32 / scale,
                    size.height as f32 / scale,
                    pos.x as f32 / scale,
                    pos.y as f32 / scale,
                ),
                Err(_) => cache::WindowCache::capture(
                    size.width as f32 / scale,
                    size.height as f32 / scale,
                    0.0,
                    0.0,
                ),
            };
            if self.window_cache.as_ref() != Some(&current) {
                if let Err(e) = cache::save(&current) {
                    warn!("failed to save window cache: {}", e);
                }
                self.window_cache = Some(current);
            }
        }

        // BEGIN - ui repaint hack
        // https://github.com/emilk/egui/issues/5136
        if !self.repaint_state.started {
            self.repaint_state.started = true;
            let ui_clone = ui.clone();

            self.rt.spawn(async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_secs_f64(1.0 / 60.0)).await;
                    ui_clone.request_repaint();
                }
            });
        }
        if self.repaint_state.last_tick.elapsed() >= self.repaint_state.tick_rate {
            ui.request_repaint();
            self.repaint_state.last_tick = std::time::Instant::now();
        }
        // END - ui repaint hack

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.set_width(_frame.winit_window().unwrap().inner_size().width as f32 - 24.0);
                ui.set_height(_frame.winit_window().unwrap().inner_size().height as f32 - 24.0);

                if matches!(self.ui_state.active_view, LauncherView::MainMenu) {
                    // Main Menu View
                    views::main::main::render_main_menu(self, ui);
                } else if matches!(self.ui_state.active_view, LauncherView::WorkspaceMenu) {
                    // Workspace Menu View
                    views::main::workspace::render_workspace_menu(self, ui);
                } else if matches!(self.ui_state.active_view, LauncherView::ConfigError) {
                    // Config Error View
                    views::main::config_error::render_config_error(self, ui);
                }
            });
        });
    }
}
