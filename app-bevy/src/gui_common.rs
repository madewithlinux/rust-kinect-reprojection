use std::fmt::Debug;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use egui::emath::Numeric;
use egui::{Button, CollapsingHeader, DragValue, Grid, Response, RichText, ScrollArea, Slider, Ui};

pub const DEFAULT_DRAG_VALUE_SPEED: f32 = 0.1;
pub const DEFAULT_MIN_DECIMALS: usize = 1;
pub const DEFAULT_MAX_DECIMALS: usize = 3;

pub trait GuiViewable {
    fn gui_view(&self, ui: &mut Ui) -> Response;
}
pub trait GuiEditable {
    fn gui_edit(&mut self, ui: &mut Ui) -> Response;
}

impl GuiEditable for bool {
    fn gui_edit(&mut self, ui: &mut Ui) -> Response {
        ui.checkbox(self, "")
    }
}

trait StdNumberMarker {}
impl StdNumberMarker for f32 {}
impl StdNumberMarker for f64 {}
impl StdNumberMarker for u8 {}
impl StdNumberMarker for u16 {}
impl StdNumberMarker for u32 {}
impl StdNumberMarker for u64 {}
impl StdNumberMarker for usize {}
impl StdNumberMarker for i8 {}
impl StdNumberMarker for i16 {}
impl StdNumberMarker for i32 {}
impl StdNumberMarker for isize {}

impl<T: egui::emath::Numeric + StdNumberMarker> GuiViewable for T {
    fn gui_view(&self, ui: &mut Ui) -> Response {
        if Self::INTEGRAL {
            ui.add(
                egui::Button::new(
                    RichText::new(egui::emath::format_with_minimum_decimals(self.to_f64(), 0)).monospace(),
                )
                .wrap(false)
                .sense(egui::Sense::hover()),
            )
        } else {
            let decimal_range = DEFAULT_MIN_DECIMALS..=DEFAULT_MAX_DECIMALS;
            ui.add(
                egui::Button::new(
                    RichText::new(egui::emath::format_with_decimals_in_range(self.to_f64(), decimal_range)).monospace(),
                )
                .wrap(false)
                .sense(egui::Sense::hover()),
            )
        }
    }
}

// impl GuiViewable for f32 {
//     fn gui_view(&self, ui: &mut Ui) -> Response {
//         let decimal_range = DEFAULT_MIN_DECIMALS..=DEFAULT_MAX_DECIMALS;
//         ui.add(
//             egui::Button::new(
//                 RichText::new(egui::emath::format_with_decimals_in_range(self.to_f64(), decimal_range)).monospace(),
//             )
//             .wrap(false)
//             .sense(egui::Sense::hover()),
//         )
//     }
// }
impl GuiEditable for f32 {
    fn gui_edit(&mut self, ui: &mut Ui) -> Response {
        ui.add(DragValue::new(self).speed(DEFAULT_DRAG_VALUE_SPEED))
    }
}

impl GuiViewable for Vec3 {
    fn gui_view(&self, ui: &mut Ui) -> Response {
        ui.columns(3, |uis| {
            let mut response = self.x.gui_view(&mut uis[0]);
            response |= self.y.gui_view(&mut uis[1]);
            response |= self.z.gui_view(&mut uis[2]);
            response
        })
    }
}
impl GuiEditable for Vec3 {
    fn gui_edit(&mut self, ui: &mut Ui) -> Response {
        ui.columns(3, |uis| {
            let mut response = self.x.gui_edit(&mut uis[0]);
            response |= self.y.gui_edit(&mut uis[1]);
            response |= self.z.gui_edit(&mut uis[2]);
            response
        })
    }
}

impl<T0: GuiViewable, T1: GuiViewable> GuiViewable for (T0, T1) {
    fn gui_view(&self, ui: &mut Ui) -> Response {
        ui.columns(3, |uis| {
            let mut response = self.0.gui_view(&mut uis[0]);
            response |= self.1.gui_view(&mut uis[1]);
            response
        })
    }
}
impl<T0: GuiViewable, T1: GuiViewable, T2: GuiViewable> GuiViewable for (T0, T1, T2) {
    fn gui_view(&self, ui: &mut Ui) -> Response {
        ui.columns(3, |uis| {
            let mut response = self.0.gui_view(&mut uis[0]);
            response |= self.1.gui_view(&mut uis[1]);
            response |= self.2.gui_view(&mut uis[2]);
            response
        })
    }
}

// // region: blanket impls
// pub trait GuiViewableDebug: Debug {}

// impl<T: GuiViewableDebug> GuiViewable for T {
//     fn gui_view(&self, ui: &mut Ui) -> egui::Response {
//         ui.label(format!("{:?}", self))
//     }
// }

// // endregion
