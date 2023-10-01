use egui_plot::{Line, Plot, PlotPoints,LineStyle,Polygon, PlotBounds};
use egui::{Color32,remap};
use std::f64::consts::TAU;

use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData, common::vector2::Vector2, galaxy::sector::Sector};

pub struct SectorMap {
    first_zoom: bool,
    sector: Sector,
}

impl Default for SectorMap {
    fn default() -> Self {
        Self { first_zoom: false, sector: Sector::new("Test".to_string(), "Test (HXC-102)".to_string(), 5.0, Color32::LIGHT_YELLOW, Color32::LIGHT_BLUE, vec![])}
    }
}

impl GuiWindow for SectorMap {
    fn name(&self) -> &'static str {
        "Sector Map"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData{
        egui::Window::new(self.name())
            .default_width(320.0)
            .open(open)
            .show(ctx, |ui| {
                use GuiView as _;
                self.ui(ui);
            });

        return data;
    }

    fn killed(&mut self) {
        
    }
}

impl GuiView for SectorMap {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let scroll_delta = ui.input(|i| i.scroll_delta);

        let markers_plot = Plot::new("sector_map")
            .width(600.0)
            .height(600.0)
            .show_axes(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .data_aspect(1.0);

        markers_plot
            .show(ui, |plot_ui| {
                //Draw Star
                plot_ui.polygon(
                    {
                        let n = 512;
                        let circle_points: PlotPoints = (0..=n)
                            .map(|i| {
                                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                                let r = self.sector.star_radius;
                                [
                                    r * t.cos() + 0 as f64,
                                    r * t.sin() + 0 as f64,
                                ]
                            })
                            .collect();
                        Polygon::new(circle_points)
                            .fill_color(self.sector.star_color)
                            .style(LineStyle::Solid)
                            .width(0.1)
                            .name(self.sector.star_name.to_string())
                    }
                );
                //Draw Sector Border
                plot_ui.line(
                    {
                        let n = 512;
                        let circle_points: PlotPoints = (0..=n)
                            .map(|i| {
                                let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
                                let r = 256.0;
                                [
                                    r * t.cos() + 0 as f64,
                                    r * t.sin() + 0 as f64,
                                ]
                            })
                            .collect();
                        Line::new(circle_points)
                            .color(self.sector.border_color)
                            .style(LineStyle::Dashed { length: 10.0 })
                            .name("Sector Security Zone")
                    }
                );

                //Draw Entities
                let points = PlotPoints::new(vec![
                    [0.0, 0.0],
                    [2.0, 0.0],
                    [1.0, 2.0],
                    [0.0, 0.0],
                ]);
                let planned_line = Polygon::new(points)
                    .fill_color(Color32::from_rgb(100, 200, 100))
                    .style(LineStyle::Solid)
                    .width(0.1)
                    .name("GOX-101");
                plot_ui.polygon(planned_line);

                // ------------------
                // Custom Zoom
                // ------------------

                if scroll_delta != egui::Vec2::ZERO {
                    let frame_ratio = plot_ui.transform().bounds().height() as f64 / plot_ui.transform().frame().height() as f64; // zoomed out 1.0 zommed in 0.1

                    //Zoom
                    let mut new_bounds = plot_ui.plot_bounds();
                    new_bounds = PlotBounds::from_min_max(
                        [
                            new_bounds.min()[0] - ((-30.0 * scroll_delta.y.clamp(-1.0, 1.0) as f64) * frame_ratio),
                            new_bounds.min()[1] - ((-30.0 * scroll_delta.y.clamp(-1.0, 1.0) as f64) * frame_ratio)
                        ], 
                        [
                            new_bounds.max()[0] - ((30.0 * scroll_delta.y.clamp(-1.0, 1.0) as f64) * frame_ratio),
                            new_bounds.max()[1] - ((30.0 * scroll_delta.y.clamp(-1.0, 1.0) as f64) * frame_ratio)
                        ]
                    );

                    //Caping the zoom
                    let bounds_space = Vector2::new(new_bounds.max().to_vec()[0], new_bounds.max().to_vec()[1]) - Vector2::new(new_bounds.min().to_vec()[0], new_bounds.min().to_vec()[1]);
                    if new_bounds.is_valid() && !((bounds_space.x > 600.0 || bounds_space.y > 600.0) || (bounds_space.x < 20.0 || bounds_space.y < 20.0)) {
                        plot_ui.set_plot_bounds(new_bounds)
                    }
                }
            }
        );
    }
}