pub mod context;
pub mod prelude;
pub mod sketch;
mod sketch_runner;
pub mod widgets;

pub type Result = anyhow::Result<()>;

pub use sketch::Sketch;

/// This is the trait that your sketch app must implement.
pub trait App {
    fn update(&mut self, sketch: &mut Sketch, ctx: &mut context::Context) -> anyhow::Result<()>;

    //TODO:
    // - extra ui?
    // - extra CLI?
}

pub trait SketchUI {
    /// Draw the UI for the sketch, return whether the sketch should be updated.
    ///
    /// This function is generated by the [`Sketch`] derive macro.
    fn ui(&mut self, ui: &mut egui::Ui) -> bool;
}

pub trait SketchApp: App + SketchUI {}

pub fn run_default<APP: SketchApp + Default + 'static>() -> anyhow::Result<()> {
    vsvg_viewer::show_with_viewer_app(Box::new(sketch_runner::SketchRunner::new(
        Box::<APP>::default(),
    )))
}

pub fn run<APP: SketchApp + 'static>(app: APP) -> anyhow::Result<()> {
    vsvg_viewer::show_with_viewer_app(Box::new(sketch_runner::SketchRunner::new(Box::new(app))))
}
