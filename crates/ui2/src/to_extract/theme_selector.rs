use crate::prelude::*;
use crate::{OrderMethod, Palette, PaletteItem};

#[derive(RenderOnce)]
pub struct ThemeSelector {
    id: ElementId,
}

impl Component for ThemeSelector {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div().child(
            Palette::new(self.id.clone())
                .items(vec![
                    PaletteItem::new("One Dark"),
                    PaletteItem::new("Rosé Pine"),
                    PaletteItem::new("Rosé Pine Moon"),
                    PaletteItem::new("Sandcastle"),
                    PaletteItem::new("Solarized Dark"),
                    PaletteItem::new("Summercamp"),
                    PaletteItem::new("Atelier Cave Light"),
                    PaletteItem::new("Atelier Dune Light"),
                    PaletteItem::new("Atelier Estuary Light"),
                    PaletteItem::new("Atelier Forest Light"),
                    PaletteItem::new("Atelier Heath Light"),
                ])
                .placeholder("Select Theme...")
                .empty_string("No matches")
                .default_order(OrderMethod::Ascending),
        )
    }
}

impl ThemeSelector {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }
}

use gpui::{Div, RenderOnce};
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use gpui::{Div, Render};

    use crate::Story;

    use super::*;

    pub struct ThemeSelectorStory;

    impl Render for ThemeSelectorStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, ThemeSelector>(cx))
                .child(Story::label(cx, "Default"))
                .child(ThemeSelector::new("theme-selector"))
        }
    }
}
