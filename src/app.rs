use std::sync::mpsc::Sender;
use eframe::egui::{CtxRef, Vec2};
use eframe::epi::Frame;
use {eframe, eframe::epi, eframe::egui};

pub enum TemplateAppEvent
{
    Sleep { time: u64 },
    Pause { state: bool },
    Restart,
}

pub struct TemplateApp
{
    pub sleep_time: u64,
    pub pause: bool,
    evt_bus: Sender<TemplateAppEvent>
}

impl TemplateApp {
    pub fn new(tx: Sender<TemplateAppEvent>) -> TemplateApp
    {
        TemplateApp
        {
            sleep_time: 100,
            pause: true,
            evt_bus: tx
        }
    }
}

impl epi::App for TemplateApp
{
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("KiraKun's Game of Life");

            ui.horizontal(|ui| {
                ui.label("Sleep Time: ");
                if ui.add(egui::Slider::new(&mut self.sleep_time, 0..=5000)).drag_released()
                {
                    self.evt_bus.send(TemplateAppEvent::Sleep { time: self.sleep_time });
                }

                if ui.button("-").clicked()
                {
                    self.sleep_time -= 100;
                    self.evt_bus.send(TemplateAppEvent::Sleep { time: self.sleep_time });
                }

                if ui.button("+").clicked()
                {
                    self.sleep_time += 100;
                    self.evt_bus.send(TemplateAppEvent::Sleep { time: self.sleep_time });
                }
            });

            if ui.button("RESTART").clicked()
            {
                self.evt_bus.send(TemplateAppEvent::Restart);
            }

            if ui.checkbox(&mut self.pause, "Pause").changed()
            {
                self.evt_bus.send(TemplateAppEvent::Pause { state: self.pause });
            }

            egui::warn_if_debug_build(ui);
        });

        _frame.set_window_size(Vec2::new(280., 110.));
    }

    fn name(&self) -> &str { "KiraKun's Game of Life" }
}