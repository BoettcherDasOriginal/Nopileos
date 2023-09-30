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
        Self { first_zoom: false, sector: Sector::new("Test".to_string(), "Test".to_string(), 5.0, Color32::LIGHT_YELLOW, Color32::LIGHT_BLUE, vec![])}
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
        let markers_plot = Plot::new("sector_map")
            .width(600.0)
            .height(600.0)
            .show_axes(false)
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

                // making sure the player stays in the given bounds, so the RAM doesn't start exploding
                if self.first_zoom {
                    let bounds = plot_ui.plot_bounds();

                    let bounds_space = Vector2::new(bounds.max().to_vec()[0], bounds.max().to_vec()[1]) - Vector2::new(bounds.min().to_vec()[0], bounds.min().to_vec()[1]); //max - min
                    if bounds_space.x > 600.0 || bounds_space.y > 600.0 {
                        plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                            Vector2::new(bounds.min().to_vec()[0] + 10.0, bounds.min().to_vec()[1] + 10.0).as_slice(), 
                            Vector2::new(bounds.max().to_vec()[0] - 10.0, bounds.max().to_vec()[1] - 10.0).as_slice()
                        ));
                    }
                    if bounds_space.x < 20.0 || bounds_space.y < 20.0 {
                        plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                            Vector2::new(bounds.min().to_vec()[0] - 0.5, bounds.min().to_vec()[1] - 0.5).as_slice(), 
                            Vector2::new(bounds.max().to_vec()[0] + 0.5, bounds.max().to_vec()[1] + 0.5).as_slice()
                        ));
                    }
                }
                else {
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                        Vector2::new(-280.0, -280.0).as_slice(), 
                        Vector2::new(280.0, 280.0).as_slice()
                    ));
                    self.first_zoom = true;
                }
            }
        );
    }
}