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

// Main

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]),
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

            Ok(Box::<TemplateApp>::default())
        }),
    )
}

#[derive(Default)]
struct TemplateApp;

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui template app");
            ui.label(format!(
                "If you can see this window, `cargo run` works. {} {}",
                regular::ALARM,
                regular::AIRPLANE
            ));
            ui.horizontal(|ui| {
                let _ = ui.button(regular::ALARM);
                let _ = ui.button(regular::AIRPLANE);
            });
        });
    }
}
