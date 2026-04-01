use eframe::egui;
use egui_phosphor::regular;

/// Windows 11 and later use build numbers >= 22000 (see `CurrentBuild` in registry).
#[cfg(target_os = "windows")]
const WINDOWS_11_MIN_BUILD: u32 = 22_000;

/// When `Some(true)`, the OS build indicates Windows 11+. When `Some(false)`, an older Windows
/// was detected. When `None`, the build could not be read.
#[cfg(target_os = "windows")]
fn detect_windows_11_or_later() -> Option<bool> {
    let output = std::process::Command::new("reg")
        .args([
            "query",
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "/v",
            "CurrentBuild",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if line.contains("CurrentBuild") {
            let build = line.split_whitespace().last()?.parse::<u32>().ok()?;
            return Some(build >= WINDOWS_11_MIN_BUILD);
        }
    }
    None
}

/// Returns `true` when the app should force egui's dark theme: on Windows, when the OS is Windows
/// 11 or newer (detected via `CurrentBuild` >= 22000), or when the build cannot be read. Returns
/// `false` on non-Windows hosts or when an older Windows build is detected.
fn force_dark_theme_on_windows() -> bool {
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
    #[cfg(target_os = "windows")]
    {
        if std::env::consts::OS != "windows" {
            return false;
        }
        match detect_windows_11_or_later() {
            Some(true) | None => true,
            Some(false) => false,
        }
    }
}

fn init_logger() -> Result<(), log::SetLoggerError> {
    egui_logger::builder()
        .max_level(log::LevelFilter::Debug)
        .init()
}

// Dock (egui_tiles)

struct DockPane {
    nr: usize,
}

struct DockBehavior;

impl egui_tiles::Behavior<DockPane> for DockBehavior {
    fn tab_title_for_pane(&mut self, pane: &DockPane) -> egui::WidgetText {
        format!("Pane {}", pane.nr).into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut DockPane,
    ) -> egui_tiles::UiResponse {
        ui.vertical(|ui| {
            ui.heading(format!("Dock pane {}", pane.nr));

            if pane.nr == 0 {
                ui.label(format!(
                    "Phosphor icons: {} {}",
                    regular::ALARM,
                    regular::AIRPLANE
                ));
                ui.horizontal(|ui| {
                    let _ = ui.button(regular::ALARM);
                    let _ = ui.button(regular::AIRPLANE);
                });
            } else {
                ui.label("Resize the splitters or drag tabs to dock tiles.");
            }
        });

        if ui
            .add(egui::Button::new("Drag to dock").sense(egui::Sense::drag()))
            .drag_started()
        {
            egui_tiles::UiResponse::DragStarted
        } else {
            egui_tiles::UiResponse::None
        }
    }
}

fn create_dock_tree() -> egui_tiles::Tree<DockPane> {
    let mut next_view_nr = 0;
    let mut gen_pane = || {
        let pane = DockPane { nr: next_view_nr };
        next_view_nr += 1;
        pane
    };

    let mut tiles = egui_tiles::Tiles::default();

    let mut tabs = vec![];
    tabs.push({
        let children = (0..3).map(|_| tiles.insert_pane(gen_pane())).collect();
        tiles.insert_horizontal_tile(children)
    });
    tabs.push({
        let cells = (0..4).map(|_| tiles.insert_pane(gen_pane())).collect();
        tiles.insert_grid_tile(cells)
    });
    tabs.push(tiles.insert_pane(gen_pane()));

    let root = tiles.insert_tab_tile(tabs);

    egui_tiles::Tree::new("main_dock", root, tiles)
}

// Main

fn main() -> eframe::Result<()> {
    init_logger().expect("global logger should be installed once at process startup");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "egui cross-platform template",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            if force_dark_theme_on_windows() {
                cc.egui_ctx.set_theme(egui::Theme::Dark);
            }

            Ok(Box::new(TemplateApp {
                tree: create_dock_tree(),
                logged_startup: false,
            }))
        }),
    )
}

struct TemplateApp {
    tree: egui_tiles::Tree<DockPane>,
    logged_startup: bool,
}

impl eframe::App for TemplateApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.logged_startup {
            log::info!("egui_app started; open the Log window for captured output");
            self.logged_startup = true;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let mut behavior = DockBehavior;
            self.tree.ui(&mut behavior, ui);
        });

        let ctx = ui.ctx();
        egui::Window::new("Log")
            .default_size([1024.0, 360.0])
            .show(ctx, |ui| {
                egui_logger::logger_ui().show(ui);
            });
    }
}

#[cfg(test)]
mod tests {
    use super::create_dock_tree;

    #[test]
    fn dock_tree_has_multiple_tiles_and_root() {
        let tree = create_dock_tree();
        assert!(tree.root.is_some(), "dock tree should have a root tile");
        assert!(tree.tiles.len() > 1, "dock should contain multiple tiles");
    }
}
