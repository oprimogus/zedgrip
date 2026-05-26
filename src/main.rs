mod theme;
mod ui;

use gpui::{
    prelude::*, px, size, App, Application, Bounds, Point, SharedString, TitlebarOptions,
    WindowBackgroundAppearance, WindowBounds, WindowDecorations, WindowOptions,
};
use ui::app::ZedGripApp; // Import ZedGripApp from ui::app

fn main() {
    Application::new().run(|cx: &mut App| {
        // Start the ZedGripApp which now manages MainView
        ZedGripApp::start(cx);
    });
}
