use std::fmt::Display;

use eframe::{
    egui::{
        style::Margin, CentralPanel, Context, Direction, Frame as EFrame, InnerResponse, Layout,
        Response, RichText, Sense, SidePanel, TopBottomPanel, Ui,
    },
    emath::{Align, RectTransform},
    epaint::{vec2, Color32, Pos2, Rect, Shape, Stroke, Vec2},
    epi::{App, Frame},
    run_native, NativeOptions,
};

use kohonen::Kohonen;
use ndarray::Array2;

use crate::{CANVAS_SIZE, DATA_PATH, MATRIX_DIMS, WINDOW_DIMS};

fn centered_label(ui: &mut Ui, text: impl Into<String>) -> Response {
    let text = RichText::new(text).size(30.0);
    ui.with_layout(Layout::top_down(Align::Center), |ui| ui.label(text))
        .response
}

fn control_button(ui: &mut Ui, text: impl Into<String>) -> Response {
    let text = RichText::new(text).size(26.0);
    ui.spacing_mut().button_padding *= 2.5;
    let res = ui.button(text);
    ui.reset_style();

    res
}

fn select_list<T>(
    ui: &mut Ui,
    layout: Layout,
    target: &mut T,
    entries: &mut dyn Iterator<Item = T>,
) -> InnerResponse<Vec<Response>>
where
    T: Display + PartialEq,
{
    ui.with_layout(layout, |ui| {
        entries
            .map(|entry| {
                let text = RichText::new(format!("{}", entry)).size(26.0);
                ui.selectable_value(target, entry, text)
            })
            .collect()
    })
}

#[derive(Debug)]
pub struct MainWindow {
    canvas: Canvas,
    kohonen: Kohonen<u8>,
    current_number: u8,
    current_guess: Option<u8>,
}

impl MainWindow {
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
                self.current_guess
                    .map_or_else(String::new, |g| format!("I'm guessing that's a {}", g)),
            );

            SidePanel::left("Buttons")
                .max_width(ui.available_width() * 0.5)
                .frame(
                    EFrame::default()
                        .stroke(Default::default())
                        .margin(Margin::same(6.0)),
                )
                .show_inside(ui, |ui| {
                    centered_label(ui, "Controls");

                    ui.with_layout(Layout::left_to_right(), |ui| {
                        if control_button(ui, "Reset").clicked() {
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
                            let (guess, confidence) = self.kohonen.guess(&self.canvas.pixel_matrix);
                            dbg!(confidence);
                            self.current_guess = Some(*guess);
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

    fn name(&self) -> &str {
        "Number Recognizer"
    }
}

#[derive(Debug)]
pub struct Canvas {
    lines: Vec<Vec<Pos2>>,
    stroke: Stroke,
    pixel_matrix: Array2<u8>,
    scale: Option<(f32, f32)>,
}

impl Default for Canvas {
    fn default() -> Self {
        let (x, y) = MATRIX_DIMS;
        Self {
            lines: Default::default(),
            stroke: Stroke::new(2.0, Color32::WHITE),
            pixel_matrix: Array2::zeros((x, y)),
            scale: None,
        }
    }
}

impl Canvas {
    pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
        let rect = ui.max_rect();
        let (mut response, painter) =
            ui.allocate_painter(vec2(rect.width(), rect.height()), Sense::drag());

        if self.scale.is_none() {
            let (dim_x, dim_y) = self.pixel_matrix.dim();
            let (width, height) = (response.rect.width(), response.rect.height());
            self.scale = Some((
                (width / dim_x as f32).floor(),
                (height / dim_y as f32).floor(),
            ));
        }

        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;

            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }

            let index = self.map_position(pointer_pos - response.rect.left_top());
            self.pixel_matrix[index] = 1;
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        let mut shapes = vec![];
        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<Pos2> = line.iter().map(|&p| to_screen * p).collect();
                shapes.push(Shape::line(points, self.stroke));
            }
        }
        painter.extend(shapes);

        response
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        centered_label(ui, "Draw a number");

        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                EFrame::dark_canvas(ui.style()).show(ui, |ui| {
                    self.ui_content(ui);
                });
            },
        );
    }

    pub fn clear(&mut self) {
        self.lines.clear();
        self.pixel_matrix.fill(0);
    }

    pub fn map_position(&self, pos: Vec2) -> (usize, usize) {
        let (mouse_x, mouse_y) = (pos.x, pos.y);
        let (scale_x, scale_y) = self.scale.unwrap();
        let dims = self.pixel_matrix.dim();
        let (max_x, max_y) = (dims.0 - 1, dims.1 - 1);

        (
            max_x.min((mouse_x / scale_x).floor() as usize),
            max_y.min((mouse_y / scale_y).floor() as usize),
        )
    }
}
