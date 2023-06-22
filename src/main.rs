use vizia::{
    prelude::{EmitContext, EnvironmentEvent, ThemeMode},
    window::WindowModifiers,
    Application,
};

use gui::Gui;

mod gui;

fn main() {
    Application::new(|cx| {
        cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));

        Gui::new(cx);
    })
    .title("Password generator")
    .min_inner_size(Some((400, 600)))
    .inner_size((400, 300))
    .run()
}
