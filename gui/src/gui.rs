use eframe::{
    egui::{
        style::Margin, CentralPanel, Context, Frame as EFrame, Layout, SidePanel, TopBottomPanel,
    },
    emath::Align,
    epaint::vec2,
    epi::{App, Frame},
    run_native, NativeOptions,
};

use kohonen::Kohonen;

use crate::{
    canvas::Canvas,
    components::{centered_label, control_button, select_list},
    consts::{APP_NAME, CANVAS_SIZE, DATA_PATH, MATRIX_DIMS, WINDOW_DIMS},
};

/// Main Window
#[derive(Debug)]
pub struct MainWindow {
    canvas: Canvas,
    kohonen: Kohonen<u8>,
    current_number: u8,
    current_guess: Option<(u8, f32)>,
}

impl MainWindow {
    /// Starts the main loop
    pub fn run() -> ! {
        let kohonen = match Kohonen::load_from(DATA_PATH) {
            Ok(k) => k,
            Err(_) => Kohonen::init(MATRIX_DIMS, 0..=9),
        };
        let app = Box::new(Self {
            canvas: Canvas::default(),
            kohonen,
            current_number: 0,
            current_guess: None,
        });

        let (x, y) = WINDOW_DIMS;
        let options = NativeOptions {
            initial_window_size: Some(vec2(x, y)),
            resizable: false,
            ..Default::default()
        };
        // options
        run_native(app, options);
    }
}

impl App for MainWindow {
    /// Called every frame
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        TopBottomPanel::top("Canvas").show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let width = ctx.available_rect().width();
                ui.set_width(width * CANVAS_SIZE);
                ui.set_height(width * CANVAS_SIZE);
                self.canvas.ui(ui);
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            centered_label(
                ui,
                self.current_guess.map_or_else(String::new, |(v, c)| {
                    format!("I'm guessing that's a {} ({:.2}%)", v, c * 100.0)
                }),
            );

            SidePanel::left("Buttons")
                .max_width(ui.available_width() * 0.5)
                .frame(
                    EFrame::default()
                        .stroke(Default::default())
                        .margin(Margin::same(6.0)),
                )
                .resizable(false)
                .show_inside(ui, |ui| {
                    centered_label(ui, "Controls");

                    ui.with_layout(Layout::left_to_right(), |ui| {
                        if control_button(ui, "Clear").clicked() {
                            self.current_guess = None;
                            self.canvas.clear();
                        }
                        if control_button(ui, "Teach").clicked() {
                            self.kohonen
                                .teach(self.current_number, &self.canvas.pixel_matrix);
                            // dbg!(&self.canvas.pixel_matrix);
                            self.kohonen
                                .save_to(DATA_PATH)
                                .expect("Failed to save data");
                        }
                        if control_button(ui, "Guess").clicked() {
                            let (&v, c) = self.kohonen.guess(&self.canvas.pixel_matrix);
                            self.current_guess = Some((v, c));
                        }
                    });
                });
            CentralPanel::default().show_inside(ui, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    centered_label(ui, "Pick a number")
                });

                select_list(
                    ui,
                    Layout::left_to_right(),
                    &mut self.current_number,
                    &mut (0..=9),
                )
            });
        });
    }

    /// App name
    fn name(&self) -> &str {
        APP_NAME
    }
}
