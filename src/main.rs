use eframe::egui;
use egui_phosphor::regular;

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
