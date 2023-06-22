use clipboard::{ClipboardProvider, ClipboardContext};
use password_generator::PasswordGenerator;
use vizia::{prelude::*, icons::{ICON_COPY, ICON_SETTINGS_AUTOMATION}};

use self::password_extra::PasswordExtra;

mod password_extra;

#[derive(Lens)]
struct GuiData {
    password_length: usize,
    password_extra: bool,
    password_generator: PasswordGenerator,
    password_chars: String,
    generated_password: String,
    status: String,
    status_color: Color,
    theme: char,
}

impl Default for GuiData {
    fn default() -> Self {
        let password_generator = PasswordGenerator::default();
        let password_chars = password_generator.get_chars();
        Self {
            password_length: 8,
            password_extra: false,
            password_generator,
            password_chars,
            generated_password: String::new(),
            status: "Program started".to_owned(),
            status_color: Color::white(),
            theme: '\u{eaf8}',
        }
    }
}

impl Model for GuiData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _| match event {
            GuiEvent::IncreasePassLength => self.password_length = self.password_length.saturating_add(1),
            GuiEvent::DecreasePassLength => self.password_length = self.password_length.saturating_sub(1),
            GuiEvent::SetPassLength(len) => self.password_length = *len,
            GuiEvent::TogglePassExtra => self.password_extra ^= true,
            GuiEvent::RegeneratePass => {
                self.password_generator.regenerate_characters();
                self.password_chars = self.password_generator.get_chars();
                self.status = "regenerated password characters.".to_owned();
                self.status_color = Color::green();
            },
            GuiEvent::ChangePassChars(chars) => {
                self.password_chars = chars.to_owned();
                self.password_generator.change_characters(chars.chars().collect());
            },
            GuiEvent::GeneratePass => {
                self.generated_password = self.password_generator.generate(self.password_length);
                self.status = "password generated.".to_owned();
                self.status_color = Color::green();
            },
            GuiEvent::CopyPass => {
                match ClipboardContext::new().and_then(|mut cx| cx.set_contents(self.generated_password.clone())) {
                    Ok(_) => {
                        self.status = "copied to clipboard".to_owned();
                        self.status_color = Color::green();
                    },
                    Err(_) => {
                        self.status = "error in copying to clipboard".to_owned();
                        self.status_color = Color::red();
                    },
                }
            },
            GuiEvent::ChangeTheme => match self.theme {
                '\u{eb30}' /* sun */ => {
                    cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::DarkMode));
                    self.theme = '\u{eaf8}';
                    self.status_color = Color::white();
                },
                '\u{eaf8}' /* moon */ => {
                    cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::LightMode));
                    self.theme = '\u{eb30}';
                    self.status_color = Color::black();
                },
                _ => unreachable!(),
            },
        })
    }
}

enum GuiEvent {
    IncreasePassLength,
    DecreasePassLength,
    SetPassLength(usize),
    TogglePassExtra,
    RegeneratePass,
    ChangePassChars(String),
    GeneratePass,
    CopyPass,
    ChangeTheme,
}

pub struct Gui;

impl View for Gui {
    fn element(&self) -> Option<&'static str> {
        Some("gui")
    }
}

impl Gui {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self.build(cx, |cx| {
            GuiData::default().build(cx);

            cx.add_stylesheet(include_style!("src/gui/styles/style.css"))
                .expect("Failed to add stylesheet");

            Label::new(cx, "Password Generator").class("title");

            HStack::new(cx, |cx| {
                Label::new(cx, "Length:");
                Spinbox::custom(
                    cx,
                    |cx| {
                        Textbox::new(cx, GuiData::password_length)
                            .validate(|v| v.parse::<usize>().is_ok())
                            .on_submit(|cx, text, _| {
                                cx.emit(GuiEvent::SetPassLength(text.parse().unwrap()))
                            })
                    },
                    SpinboxKind::Horizontal,
                    SpinboxIcons::PlusMinus,
                )
                .on_increment(|cx| cx.emit(GuiEvent::IncreasePassLength))
                .on_decrement(|cx| cx.emit(GuiEvent::DecreasePassLength));
            })
            .class("password-input");

            HStack::new(cx, |cx| {
                Checkbox::new(cx, GuiData::password_extra)
                    .on_toggle(|cx| cx.emit(GuiEvent::TogglePassExtra));

                Label::new(cx, "More");
                
                Button::new(
                    cx,
                    |cx| cx.emit(GuiEvent::RegeneratePass),
                    |cx| Label::new(cx, "Regenerate password characters")
                );
            })
            .class("password-extra");

            Label::new(cx, "Password").class("password-title");

            HStack::new(cx, |cx| {
                Textbox::new(cx, GuiData::generated_password);
                Button::new(
                    cx,
                    |cx| cx.emit(GuiEvent::CopyPass),
                    |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Copy");
                            Icon::new(cx, ICON_COPY);
                        })
                    }
                );
            })
            .class("password-output");

            Button::new(
                cx,
                |cx| cx.emit(GuiEvent::GeneratePass),
                |cx| {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Generate");
                        Icon::new(cx, ICON_SETTINGS_AUTOMATION);
                    })
                }
            )
            .width(Stretch(1.0));

            PasswordExtra::new(cx, GuiData::password_extra).size(Stretch(1.0));

            HStack::new(cx, |cx| {
                Label::new(cx, GuiData::status).color(GuiData::status_color);
                Button::new(
                    cx,
                    |cx| cx.emit(GuiEvent::ChangeTheme),
                    |cx| Icon::new(cx, GuiData::theme),
                )
                .left(Stretch(1.0));
            })
            .class("footer")
            .child_top(Stretch(1.0));
        })
        .layout_type(LayoutType::Column)
    }
}
