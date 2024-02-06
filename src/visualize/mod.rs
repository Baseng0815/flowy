use chrono::{DateTime, TimeZone, Local};
use eframe::egui;
use egui::{Painter, Sense, Slider, Grid};
use epaint::{Color32, Rounding, pos2, Vec2, emath::RectTransform, Rect, Stroke, vec2};

use crate::simulator::{math::vector2, simulator::Simulator, grid::StaggeredMACGrid};

struct Snapshot {
    from_when: DateTime<Local>,
    grid: StaggeredMACGrid
}

impl Snapshot {
    fn new(from_when: DateTime<Local>, grid: StaggeredMACGrid) -> Self {
        Self {
            from_when, grid
        }
    }
}

pub struct FlowyApp {
    simulator: Simulator,

    // visualization parameters
    line_width: f32,
    visualization_scaling_factor: f32,

    draw_grid: bool,
    draw_velocity_edge_vectors: bool,
    draw_velocity_center_vectors: bool,
    draw_velocity_greyscale: bool,

    // simulation parameters
    dt: f64,
    simulation_running: bool,

    snapshots: Vec<Snapshot>,
}

impl FlowyApp {
    pub fn new(simulator: Simulator) -> Self {
        Self {
            simulator,
            line_width: 0.5,
            visualization_scaling_factor: 1.0,

            draw_grid: true,
            draw_velocity_edge_vectors: false,
            draw_velocity_center_vectors: true,
            draw_velocity_greyscale: true,

            dt: 1.0,
            simulation_running: false,

            snapshots: Vec::new()
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
                let vx = (self.simulator.grid.get_vel_x(x, y) + self.simulator.grid.get_vel_x(x + 1, y)) / 2.0;
                let vy = (self.simulator.grid.get_vel_y(x, y) + self.simulator.grid.get_vel_y(x, y + 1)) / 2.0;
                let len = (vector2(vx, vy).len_squared() / 2.0f64.sqrt()) as f32 * self.visualization_scaling_factor;
                let color = Color32::from_gray((len * 255 as f32) as u8);

                let rect = Rect::from_min_size(pos2(x as f32 / cc as f32, y as f32 / cc as f32), rect_size);
                let rect_screencoords = to_screen.transform_rect(rect);
                painter.rect_filled(rect_screencoords, Rounding::ZERO, color);
            }
        }
    }

    fn draw_grid_velocities_edge_vectors(&self, painter: &Painter, to_screen: &RectTransform) {
        let cc = self.simulator.grid.cell_count;
        let half_grid = 1.0 / (2.0 * cc as f32);

        for row in 0..cc {
            for col in 0..=cc {
                let vx = self.simulator.grid.get_vel_x(col, row);
                let vy = self.simulator.grid.get_vel_y(row, col);
                let lenx = vx as f32 * self.visualization_scaling_factor / ((cc + 1) as f32);
                let leny = vy as f32 * self.visualization_scaling_factor / ((cc + 1) as f32);

                let minx = pos2(col as f32 / cc as f32, row as f32 / cc as f32 + half_grid);
                let miny = pos2(row as f32 / cc as f32 + half_grid, col as f32 / cc as f32);

                let minxt = to_screen.transform_pos(minx);
                let minyt = to_screen.transform_pos(miny);
                let maxxt = to_screen.transform_pos(minx + vec2(lenx, 0.0));
                let maxyt = to_screen.transform_pos(miny + vec2(0.0, leny));

                painter.line_segment([minxt, maxxt], Stroke::new(self.line_width, Color32::GREEN));
                painter.line_segment([minyt, maxyt], Stroke::new(self.line_width, Color32::GREEN));
            }
        }
    }

    fn draw_grid_velocities_center_vectors(&self, painter: &Painter, to_screen: &RectTransform) {
        let cc = self.simulator.grid.cell_count;
        let half_grid = 1.0 / (2.0 * cc as f32);

        for x in 0..cc {
            for y in 0..cc {
                let vel = self.simulator.grid.vel(vector2(x as f64, y as f64));
                let vel_scaled = (1.0 / (cc + 1) as f64) * self.visualization_scaling_factor as f64 * vel;

                let minx = pos2(x as f32 / cc as f32 + half_grid, y as f32 / cc as f32 + half_grid);

                let minxt = to_screen.transform_pos(minx);
                let maxxt = to_screen.transform_pos(minx + vel_scaled.into());

                painter.line_segment([minxt, maxxt], Stroke::new(self.line_width, Color32::GREEN));
            }
        }
    }
}

impl eframe::App for FlowyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("settings_panel").show(ctx, |ui| {
            ui.heading("Settings");

            ui.label("Visualization parameters");
            ui.add(Slider::new(&mut self.line_width, 0.01..=5.0).text("Line width"));
            ui.add(Slider::new(&mut self.visualization_scaling_factor, 0.01..=10.0).text("Visualization scaling factor"));

            ui.toggle_value(&mut self.draw_grid, "Draw grid");
            // ui.checkbox(&mut self.draw_grid, "Draw grid");
            ui.checkbox(&mut self.draw_velocity_greyscale, "Draw velocity (greyscale)");
            ui.checkbox(&mut self.draw_velocity_edge_vectors, "Draw velocity (edge vectors)");
            ui.checkbox(&mut self.draw_velocity_center_vectors, "Draw velocity (center vectors)");

            ui.label("Simulation parameters");
            ui.add(Slider::new(&mut self.dt, 1.0..=1000.0).text("Time step (ms)"));
            ui.checkbox(&mut self.simulation_running, "Run simulation at full speed");
            if ui.button("Step simulation").clicked() {
                self.simulator.advect(self.dt);
            }

            if ui.button("Take snapshot").clicked() {
                self.snapshots.push(Snapshot::new(Local::now(), self.simulator.grid.clone());
            }

            egui::ComboBox::from_label("Restore snapshot")
                .selected_text("lmao")
                .show_ui(ui, |ui| {
                    for snapshot
                })
            ui.menu_button("Restore snapshot", )
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

            if self.draw_velocity_edge_vectors {
                self.draw_grid_velocities_edge_vectors(&painter, &to_screen);
            }

            if self.draw_velocity_center_vectors {
                self.draw_grid_velocities_center_vectors(&painter, &to_screen);
            }

            if self.draw_grid {
                self.draw_grid_lines(&painter, &to_screen);
            }
        });
    }
}
