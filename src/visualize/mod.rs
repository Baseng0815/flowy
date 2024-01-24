use eframe::egui;
use egui::{Painter, Sense, Slider};
use epaint::{Shape, Color32, Rounding, Pos2, pos2, Vec2, emath::RectTransform, Rect, Stroke, RectShape, vec2};

use crate::simulator::{simulator::Simulator, math::vector2};

pub struct FlowyApp {
    simulator: Simulator,
    grid_line_size: f32
}

impl FlowyApp {
    pub fn new(simulator: Simulator) -> Self {
        Self {
            simulator,
            grid_line_size: 0.1
        }
    }

    fn draw_grid_lines(&self, painter: &Painter, to_screen: &RectTransform) {
        let cc = self.simulator.grid.cell_count;

        for li in 0..=cc {
            let relative = li as f32 / cc as f32;

            let min_h = to_screen.transform_pos(pos2(0.0, relative));
            let max_h = to_screen.transform_pos(pos2(1.0, relative));

            let min_v = to_screen.transform_pos(pos2(relative, 0.0));
            let max_v = to_screen.transform_pos(pos2(relative, 1.0));

            painter.line_segment([min_h, max_h], Stroke::new(self.grid_line_size, Color32::DARK_GREEN));
            painter.line_segment([min_v, max_v], Stroke::new(self.grid_line_size, Color32::DARK_GREEN));
        }
    }

    fn draw_grid_velocity_gradient(&self, painter: &Painter, to_screen: &RectTransform) {
        let cc = self.simulator.grid.cell_count;

        let rect_size = vec2(1.0 / cc as f32, 1.0 / cc as f32);

        for x in 0..self.simulator.grid.cell_count {
            for y in 0..self.simulator.grid.cell_count {
                // TODO debug; remove
                let vx = (self.simulator.grid.velocities_x[x as usize] + self.simulator.grid.velocities_x[x as usize + 1]) / 2.0;
                let vy = (self.simulator.grid.velocities_y[y as usize] + self.simulator.grid.velocities_y[y as usize + 1]) / 2.0;
                let len = vector2(vx, vy).len_squared() / 2.0f64.sqrt();
                let color = Color32::from_gray((len * 255 as f64) as u8);

                let rect = Rect::from_min_size(pos2(x as f32 / cc as f32, y as f32 / cc as f32), rect_size);
                let rect_screencoords = to_screen.transform_rect(rect);
                painter.rect_filled(rect_screencoords, Rounding::ZERO, color);
            }
        }
    }
}

impl eframe::App for FlowyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("settings_panel").show(ctx, |ui| {
            ui.heading("Settings");
            ui.label("Grid line size");
            ui.add(Slider::new(&mut self.grid_line_size, 0.01..=1.0).text("lmao"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simulation");

            let w = ui.available_width();
            let h = ui.available_height();
            let (response, painter) = ui.allocate_painter(Vec2::new(w, h), Sense::hover());

            let to_screen = RectTransform::from_to(Rect { min: pos2(-0.02, -0.02), max: pos2(1.02, 1.02) }, response.rect);

            self.draw_grid_lines(&painter, &to_screen);
            self.draw_grid_velocity_gradient(&painter, &to_screen)
        });
    }
}
