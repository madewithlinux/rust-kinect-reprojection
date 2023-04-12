use std::collections::VecDeque;

use bevy::prelude::*;

use bevy_inspector_egui::reflect_inspector;
use bevy_reflect::TypeRegistry;

use egui::plot::{Line, Plot, PlotPoints};
use itertools::Itertools;

use crate::{delay_buffer::query_performance_counter_ms, receiver::KinectFrameDataDelayBufferV2};

#[derive(Default, Debug, Clone)]
pub struct TimeSeriesPlot {
    pub title: String,
    pub num_points: usize,
    pub points: VecDeque<f64>,
}

impl TimeSeriesPlot {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            // num_points: 300,
            num_points: 2400,
            points: Default::default(),
        }
    }

    pub fn push(&mut self, value: impl Into<f64>) {
        self.points.push_back(value.into());
        while self.points.len() > self.num_points {
            self.points.pop_front();
        }
    }
    pub fn ui(&mut self, ui: &mut egui::Ui, type_registry: &TypeRegistry) {
        let minmax: (f64, f64) = self.points.iter().cloned().minmax().into_option().unwrap_or((0.0, 0.0));

        ui.heading(&self.title);
        egui::Grid::new(&self.title)
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("num_points");
                reflect_inspector::ui_for_value(&mut self.num_points, ui, type_registry);
                ui.end_row();

                ui.label("min");
                reflect_inspector::ui_for_value_readonly(&minmax.0, ui, type_registry);
                ui.end_row();

                ui.label("max");
                reflect_inspector::ui_for_value_readonly(&minmax.1, ui, type_registry);
                ui.end_row();
            });

        let sin: PlotPoints = (0..self.points.len()).map(|i| [-(i as f64), self.points[i]]).collect();
        let line = Line::new(sin);
        Plot::new(format!("plot{}", &self.title))
            .view_aspect(3.0)
            // .height(500.0)
            .auto_bounds_x()
            .auto_bounds_y()
            .include_y(0.0)
            .allow_scroll(false)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_boxed_zoom(false)
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}

#[derive(Debug, Clone, Resource)]
pub struct DebugTimingPlots {
    pub frame_delay_buffer_len: TimeSeriesPlot,
    pub oldest_frame_age: TimeSeriesPlot,
    pub newest_frame_age: TimeSeriesPlot,
}

impl Default for DebugTimingPlots {
    fn default() -> Self {
        Self {
            frame_delay_buffer_len: TimeSeriesPlot::new("frame_delay_buffer_len"),
            oldest_frame_age: TimeSeriesPlot::new("oldest_frame_age"),
            newest_frame_age: TimeSeriesPlot::new("newest_frame_age"),
        }
    }
}

impl DebugTimingPlots {
    pub fn ui(&mut self, ui: &mut egui::Ui, type_registry: &TypeRegistry) {
        ui.vertical(|ui| {
            self.frame_delay_buffer_len.ui(ui, type_registry);
            self.oldest_frame_age.ui(ui, type_registry);
            self.newest_frame_age.ui(ui, type_registry);
        });
    }
}

pub fn update_debug_timing_plots(
    mut timing_plots: ResMut<DebugTimingPlots>,
    frame_delay_buffer: Res<KinectFrameDataDelayBufferV2>,
) {
    timing_plots
        .frame_delay_buffer_len
        .push(frame_delay_buffer.0.len() as f64);
    let now = query_performance_counter_ms();
    timing_plots
        .oldest_frame_age
        .push((now - frame_delay_buffer.0.oldest_timestamp()) as f64);
    timing_plots
        .newest_frame_age
        .push((now - frame_delay_buffer.0.newest_timestamp()) as f64);
}
