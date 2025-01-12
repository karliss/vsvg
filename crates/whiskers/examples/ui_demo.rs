//! This example demonstrates all UI building capabilities of the [`derive@Sketch`] and
//! [`derive@Widget`] derived traits.

use whiskers::prelude::*;

#[derive(Sketch, Default)]
struct UiDemoSketch {
    // all basic numerical types are supported
    int_64: i64,

    // numerical types can be configured with the `#[param(...)]` attribute
    #[param(min = 0, max = 100)]
    int_0_to_100: i8,

    // other fields may be used
    #[param(min = 0, max = self.int_0_to_100)]
    int_variable_bound: i8,

    // a slider can be used instead of a DragValue
    #[param(slider, min = 0.0, max = 100.0)]
    float_0_to_100: f32,

    // a logarithmic slider can be used also
    #[param(slider, logarithmic, min = 0.01, max = 10.)]
    float_log: f64,

    // custom types
    custom_struct: CustomStruct,
    custom_struct_unnamed: CustomStructUnnamed,

    // these types are supported but have no configuration options
    boolean: bool,
    string: String,
    color: Color,
    point: Point,
}

// Custom types may be used as sketch parameter if a corresponding [`whiskers::widgets::Widget`]
// type exists. This can be done using the [`whiskers_derive::Widget`] derive macro. Alternatively,
// the [`whiskers::widgets::WidgetMapper`] trait can be implemented manually, see the `custom_ui`
// example.
// Note: all types must implement [`Default`].
#[derive(Widget, Default)]
struct CustomStruct {
    #[param(min = 0.0)]
    some_float: f64,

    #[param(min = 0.0, max = self.some_float)]
    another_float: f64,

    // nested struct are supported
    custom_struct_unnamed: CustomStructUnnamed,
}

// Tuple structs are supported too
#[derive(Widget, Default)]
struct CustomStructUnnamed(bool, String);

impl App for UiDemoSketch {
    fn update(&mut self, _sketch: &mut Sketch, _ctx: &mut Context) -> anyhow::Result<()> {
        Ok(())
    }
}

fn main() -> Result {
    Runner::new(UiDemoSketch::default())
        .with_page_size_options(PageSize::A5H)
        .with_layout_options(LayoutOptions::Center)
        .run()
}
