use std::time::Duration;

use gpui::*;

struct RTimer {
    hours: SharedString,
    minutes: SharedString,
    secs: SharedString,
    duration: usize,
}

impl RTimer {
    pub fn build(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            cx.spawn(|view, mut cx| async move {
                loop {
                    let _ = cx.update(|cx| {
                        view.update(cx, |this: &mut RTimer, cx| {
                            this.duration += 1;
                            let (h, m, s) = this.duration_to_hhmmss();
                            this.hours = format!("{:02}", h).into();
                            this.minutes = format!("{:02}", m).into();
                            this.secs = format!("{:02}", s).into();
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
                hours: "00".into(),
                minutes: "00".into(),
                secs: "00".into(),
                duration: 0,
            }
        })
    }

    fn duration_to_hhmmss(&self) -> (usize, usize, usize) {
        let hours = self.duration / 3600;
        let minutes = (self.duration % 3600) / 60;
        let seconds = self.duration % 60;

        (hours, minutes, seconds)
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
            .child(format!("{}:{}:{}", &self.hours, &self.minutes, &self.secs))
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
