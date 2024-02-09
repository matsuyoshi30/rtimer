use std::time::Duration;

use gpui::*;

struct RTimer {
    sec: SharedString,
}

impl RTimer {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            cx.spawn(|view, mut cx| async move {
                loop {
                    let _ = cx.update(|cx| {
                        view.update(cx, |this: &mut RTimer, cx| {
                            let next = this.sec.parse::<usize>().unwrap() + 1;
                            this.sec = next.to_string().into();
                            cx.notify();
                        })
                    });
                    cx.background_executor()
                        .timer(Duration::from_millis(1000))
                        .await;
                }
            })
                .detach();

            RTimer {
                sec: 0.to_string().into(),
            }
        })
    }
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
            .child(format!("{}", &self.sec))
    }
}

pub static WIDTH: f64 = 400.0;
pub static HEIGHT: f64 = 100.0;

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
            RTimer::build(cx)
        });
    });
}
