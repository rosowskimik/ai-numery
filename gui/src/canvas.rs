use eframe::{
    egui::{Direction, Frame, Layout, Response, Sense, Ui},
    emath::RectTransform,
    epaint::{vec2, Color32, Pos2, Rect, Shape, Stroke, Vec2},
};
use ndarray::Array2;

use crate::{components::centered_label, consts::MATRIX_DIMS};

/// Canvas for drawing.
///
/// Holds a matrix with scaled down image.
#[derive(Debug)]
pub struct Canvas {
    lines: Vec<Vec<Pos2>>,
    stroke: Stroke,
    pub pixel_matrix: Array2<u8>,
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
    /// Main draw function.
    pub fn ui(&mut self, ui: &mut Ui) {
        centered_label(ui, "Draw a number");

        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    self.ui_content(ui);
                });
            },
        );
    }

    /// Clears the canvas.
    pub fn clear(&mut self) {
        self.lines.clear();
        self.pixel_matrix.fill(0);
    }

    fn scale_position(&self, pos: Vec2) -> (usize, usize) {
        let (mouse_x, mouse_y) = (pos.x, pos.y);
        let (scale_x, scale_y) = self.scale.unwrap();
        let dims = self.pixel_matrix.dim();
        let (max_x, max_y) = (dims.0 - 1, dims.1 - 1);

        (
            max_x.min((mouse_x / scale_x).floor() as usize),
            max_y.min((mouse_y / scale_y).floor() as usize),
        )
    }

    fn ui_content(&mut self, ui: &mut Ui) -> Response {
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

            let index = self.scale_position(pointer_pos - response.rect.left_top());
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
}
