use eframe::egui;
use egui::{Painter, Sense, Slider};
use epaint::{Shape, Color32, Rounding, Pos2, pos2, Vec2, emath::RectTransform, Rect, Stroke, RectShape, vec2};

use crate::simulator::{simulator::Simulator, math::vector2};

pub struct FlowyApp {
    simulator: Simulator,
    line_width: f32,
    visualization_scaling_factor: f32,

    draw_grid: bool,
    draw_velocity_vectors: bool,
    draw_velocity_greyscale: bool
}

impl FlowyApp {
    pub fn new(simulator: Simulator) -> Self {
        Self {
            simulator,
            line_width: 0.5,
            visualization_scaling_factor: 1.0,

            draw_grid: true,
            draw_velocity_vectors: true,
            draw_velocity_greyscale: true
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

            painter.line_segment([min_h, max_h], Stroke::new(self.line_width, Color32::DARK_GREEN));
            painter.line_segment([min_v, max_v], Stroke::new(self.line_width, Color32::DARK_GREEN));
        }
    }

    fn draw_grid_velocities_greyscale(&self, painter: &Painter, to_screen: &RectTransform) {
        let cc = self.simulator.grid.cell_count;

        let rect_size = vec2(1.0 / cc as f32, 1.0 / cc as f32);

        for y in 0..cc {
            for x in 0..cc {
                let vx = (self.simulator.grid.vel_x(x, y) + self.simulator.grid.vel_x(x + 1, y)) / 2.0;
                let vy = (self.simulator.grid.vel_y(x, y) + self.simulator.grid.vel_y(x, y + 1)) / 2.0;
                let len = (vector2(vx, vy).len_squared() / 2.0f64.sqrt()) as f32 * self.visualization_scaling_factor;
                let color = Color32::from_gray((len * 255 as f32) as u8);

                let rect = Rect::from_min_size(pos2(x as f32 / cc as f32, y as f32 / cc as f32), rect_size);
                let rect_screencoords = to_screen.transform_rect(rect);
                painter.rect_filled(rect_screencoords, Rounding::ZERO, color);
            }
        }
    }

    fn draw_grid_velocities_vectors(&self, painter: &Painter, to_screen: &RectTransform) {
        let cc = self.simulator.grid.cell_count;
        let half_grid = 1.0 / (2.0 * cc as f32);

        for y in 0..=cc {
            for x in 0..=cc {
                let vx = self.simulator.grid.vel_x(x, y);
                let vy = self.simulator.grid.vel_y(x, y);
                let lenx = vx as f32 * self.visualization_scaling_factor/ ((cc + 1) as f32 * self.simulator.grid.cell_size as f32);
                let leny = vy as f32 * self.visualization_scaling_factor/ ((cc + 1) as f32 * self.simulator.grid.cell_size as f32);

                let minx = pos2(x as f32 / cc as f32, y as f32 / cc as f32 + half_grid);
                let miny = pos2(x as f32 / cc as f32 + half_grid, y as f32 / cc as f32);

                let minxt = to_screen.transform_pos(minx);
                let minyt = to_screen.transform_pos(miny);
                let maxxt = to_screen.transform_pos(minx + vec2(lenx, 0.0));
                let maxyt = to_screen.transform_pos(miny + vec2(0.0, leny));

                painter.line_segment([minxt, maxxt], Stroke::new(self.line_width, Color32::GREEN));
                painter.line_segment([minyt, maxyt], Stroke::new(self.line_width, Color32::GREEN));
            }
        }
    }
}

impl eframe::App for FlowyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("settings_panel").show(ctx, |ui| {
            ui.heading("Settings");

            ui.add(Slider::new(&mut self.line_width, 0.01..=5.0).text("Line width"));

            ui.add(Slider::new(&mut self.visualization_scaling_factor, 0.01..=10.0).text("Visualization scaling factor"));

            ui.checkbox(&mut self.draw_grid, "Draw grid");
            ui.checkbox(&mut self.draw_velocity_greyscale, "Draw velocity (greyscale)");
            ui.checkbox(&mut self.draw_velocity_vectors, "Draw velocity (vectors)");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simulation");

            let w = ui.available_width();
            let h = ui.available_height();
            let (response, painter) = ui.allocate_painter(Vec2::new(w, h), Sense::hover());

            let to_screen = RectTransform::from_to(Rect { min: pos2(-0.02, -0.02), max: pos2(1.02, 1.02) }, response.rect);

            if self.draw_velocity_greyscale {
                self.draw_grid_velocities_greyscale(&painter, &to_screen);
            }

            if self.draw_velocity_vectors {
                self.draw_grid_velocities_vectors(&painter, &to_screen);
            }

            if self.draw_grid {
                self.draw_grid_lines(&painter, &to_screen);
            }
        });
    }
}
