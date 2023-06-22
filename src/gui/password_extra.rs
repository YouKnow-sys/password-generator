use vizia::prelude::*;

use super::{GuiData, GuiEvent};

pub struct PasswordExtra;

impl View for PasswordExtra {
    fn element(&self) -> Option<&'static str> {
        Some("password-extra")
    }
}

impl PasswordExtra {
    pub fn new(cx: &mut Context, lens: impl Lens<Target = bool>) -> Handle<Self> {
        Self.build(cx, |cx| {
            Binding::new(cx, lens, |cx, lens| {
                if lens.get(cx) {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Password Characters").bottom(Pixels(5.0));
                        Textbox::new_multiline(cx, GuiData::password_chars, true)
                            .height(Pixels(80.0))
                            .width(Pixels(360.0))
                            .on_submit(|cx, text, _| cx.emit(GuiEvent::ChangePassChars(text)));
                    })
                    .child_left(Stretch(1.0))
                    .child_right(Stretch(1.0));
                }
            });
        })
    }
}