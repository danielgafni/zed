use crate::{prelude::*, Button, Label, Modal, TextColor};

#[derive(RenderOnce)]
pub struct CopilotModal {
    id: ElementId,
}

impl Component for CopilotModal {
    type Rendered = gpui::Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div().id(self.id.clone()).child(
                Modal::new("some-id")
                    .title("Connect Copilot to Zed")
                    .child(Label::new("You can update your settings or sign out from the Copilot menu in the status bar.").color(TextColor::Muted))
                    .primary_action(Button::new("Connect to Github").variant(ButtonVariant::Filled)),
            )
    }
}

impl CopilotModal {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

use gpui::{Div, RenderOnce, Stateful};
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Div, Render};

    pub struct CopilotModalStory;

    impl Render for CopilotModalStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, CopilotModal>(cx))
                .child(Story::label(cx, "Default"))
                .child(CopilotModal::new("copilot-modal"))
        }
    }
}
