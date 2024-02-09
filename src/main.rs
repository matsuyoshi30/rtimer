use gpui::*;

struct RTimer {
    text: SharedString,
}

impl Render for RTimer {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .full()
            .flex()
            .bg(rgb(0x2e7d32))
            .justify_center()
            .items_center()
            .shadow_lg()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

pub static WIDTH: f64 = 500.0;
pub static HEIGHT: f64 = 200.0;

fn main() {
    let mut options = WindowOptions::default();
    let bounds: Bounds<GlobalPixels> = Bounds::new(
        Point {
            x: GlobalPixels::from(250.0),
            y: GlobalPixels::from(160.0),
        },
        Size {
            width: GlobalPixels::from(WIDTH),
            height: GlobalPixels::from(HEIGHT),
        },
    );
    options.bounds = WindowBounds::Fixed(bounds);
    options.center = true;
    options.titlebar = None;
    options.is_movable = true;

    App::new().run(|cx: &mut AppContext| {
        cx.open_window(options, |cx| {
            cx.new_view(|_cx| RTimer {
                text: "World".into(),
            })
        });
    });
}
