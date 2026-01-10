use gpui::{
    App, AppContext, Application, Bounds, Context, Entity, IntoElement, Point, Render, SharedString, TitlebarOptions, WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowOptions, px, size};
use crate::ui::components::{window};

pub struct ZedGripApp {
    app_window: Entity<window::AppWindow>
}

impl Render for ZedGripApp {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let app_window = self.app_window.clone();
        app_window
    }
}

impl ZedGripApp {
    pub fn new(cx: &mut App) -> Self {
        Self {
            app_window: window::AppWindow::new(cx),
        }
    }
    
    pub fn start() {
        Application::new().run(move |cx: &mut App| {
            let bounds = Bounds::centered(
                None, 
                size(
                    px(600.0), 
                    px(600.0)), 
                cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_background: WindowBackgroundAppearance::Transparent,
                    window_decorations: Some(WindowDecorations::Client),
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::new("ZedGrip")),
                        appears_transparent: false,
                        traffic_light_position: Some(Point {
                            x: px(12.0),
                            y: px(11.0),
                        }),
                    }),
                    kind: gpui::WindowKind::Normal,
                    app_id: Some("org.oprimogus.zedgrip".to_string()),
                    ..Default::default()
                },
                |window, cx| {
                    window.set_window_title("ZedGrip");
                    cx.new(|cx| {
                        ZedGripApp::new(cx)
                    })
                },
            ).unwrap();
        });
    }
}

