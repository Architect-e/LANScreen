use eframe::egui;

fn main() -> eframe::Result<()> {
	eframe::run_native(
		"LANScreen",
		eframe::NativeOptions::default(),
		Box::new(|_creation_context| Ok(Box::new(LanScreen))),
	)
}   

struct LanScreen;

impl eframe::App for LanScreen {
	fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default()
			.frame(egui::Frame::default().fill(egui::Color32::BLACK))
			.show(context, |ui| {
				ui.vertical_centered(|ui| {
					ui.add_space(ui.available_height() * 0.4);
					ui.heading(egui::RichText::new("LANScreen").color(egui::Color32::WHITE));
					ui.label(egui::RichText::new("Ready").color(egui::Color32::GRAY));
				});
			});
	}
}
