use egui_plot::{Line, Plot, PlotPoints,LineStyle,Polygon, PlotBounds};
use egui::{Color32,remap};
use std::f64::consts::TAU;

use crate::{engine::gui_windows::{GuiWindow,GuiView}, SharedGameData, common::{vector2::Vector2,triangle::Triangle}, galaxy::sector::Sector, entities::entity::EntityType};

pub struct SectorMap {
    sector : Sector,
    selected_sector: i32,
    shared_data: SharedGameData,
}

impl Default for SectorMap {
    fn default() -> Self {
        Self { 
            sector: Sector::new("???".to_string(),-1, "???".to_string(), 5.0, Color32::LIGHT_YELLOW, Color32::LIGHT_BLUE),
            selected_sector: 0,
            shared_data: SharedGameData::new() 
        }
    }
}

impl GuiWindow for SectorMap {
    fn name(&self) -> &'static str {
        "Sector Map"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool,data: SharedGameData) -> SharedGameData{
        self.shared_data = data;
        self.sector = self.shared_data.sectors.get(self.selected_sector as usize).unwrap().clone();
        
        egui::Window::new(self.name())
            .default_width(320.0)
            .open(open)
            .show(ctx, |ui| {
                use GuiView as _;
                self.ui(ui);
            });

        return self.shared_data.clone();
    }

    fn killed(&mut self) {
        
    }
}

impl GuiView for SectorMap {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let scroll_delta = ui.input(|i| i.scroll_delta);

        ui.add_space(6.0);

        ui.horizontal(|ui| {
            egui::ComboBox::from_label("")
                .width(200.0)
                .selected_text(self.sector.name.to_string())
                .show_ui(ui, |ui| {
                    for i in self.shared_data.sectors.clone() {
                        ui.selectable_value(&mut self.selected_sector, i.id, i.name);
                    }
                }
            );
        });

        ui.add_space(6.0);

        let sector_map_plot = Plot::new("sector_map")
            .width(600.0)
            .height(600.0)
            .show_axes(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .allow_boxed_zoom(false)
            .data_aspect(1.0);

        sector_map_plot
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

                for mut e in self.shared_data.entities[self.selected_sector.clone() as usize].clone() {
                    if e.get_position().global_pos == self.selected_sector {
                        match e.get_settings().clone().e_type {
                            EntityType::Ship => {
                                plot_ui.polygon({
                                    Polygon::new(PlotPoints::new(Triangle::new(e.get_position().local_pos, 1.0, 0).get_points()))
                                        .fill_color(Color32::BLUE)
                                        .style(LineStyle::Solid)
                                        .width(0.1)
                                        .name(e.get_settings().name)
                                });
                            }
                            EntityType::Station => {
                                let points = PlotPoints::new(vec![
                                    [-1.0 + e.get_position().local_pos.x, -1.5 + e.get_position().local_pos.y],
                                    [1.0 + e.get_position().local_pos.x, -1.5 + e.get_position().local_pos.y],
                                    [1.75 + e.get_position().local_pos.x, 0.0 + e.get_position().local_pos.y],
                                    [1.0 + e.get_position().local_pos.x, 1.5 + e.get_position().local_pos.y],
                                    [-1.0 + e.get_position().local_pos.x, 1.5 + e.get_position().local_pos.y],
                                    [-1.75 + e.get_position().local_pos.x, 0.0 + e.get_position().local_pos.y],
                                    [-1.0 + e.get_position().local_pos.x, -1.5 + e.get_position().local_pos.y],
                                ]);
    
                                plot_ui.polygon(
                                    {
                                        Polygon::new(points)
                                            .fill_color(Color32::from_rgb(52, 152, 219))
                                            .style(LineStyle::Solid)
                                            .width(0.1)
                                            .name(e.get_settings().name)
                                    }
                                );
                            }
                        }
                    }
                }

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

                // -------------------
                // Entity Context Menu
                // -------------------

                if plot_ui.response().secondary_clicked(){
                    let mouse_pos = Vector2::new(plot_ui.pointer_coordinate().unwrap().x, plot_ui.pointer_coordinate().unwrap().y);

                    for mut e in self.shared_data.entities[self.selected_sector.clone() as usize].clone() {
                        if e.get_position().global_pos == self.selected_sector {
                            if e.is_mouse_hovered(mouse_pos.clone(), 3.0) {
                                println!("{}",e.get_settings().name)
                            }
                        }
                    }
                }
            }
        );
    }
}