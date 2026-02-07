use bevy::prelude::*;

/// The seven elements in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Element {
    #[default]
    Pyro,
    Hydro,
    Electro,
    Cryo,
    Anemo,
    Geo,
    Dendro,
}

impl Element {
    /// Display color for each element.
    pub fn color(&self) -> Color {
        match self {
            Element::Pyro => Color::srgb(1.0, 0.3, 0.1),
            Element::Hydro => Color::srgb(0.2, 0.5, 1.0),
            Element::Electro => Color::srgb(0.7, 0.3, 1.0),
            Element::Cryo => Color::srgb(0.5, 0.9, 1.0),
            Element::Anemo => Color::srgb(0.4, 0.9, 0.6),
            Element::Geo => Color::srgb(1.0, 0.8, 0.2),
            Element::Dendro => Color::srgb(0.3, 0.8, 0.2),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Element::Pyro => "Pyro",
            Element::Hydro => "Hydro",
            Element::Electro => "Electro",
            Element::Cryo => "Cryo",
            Element::Anemo => "Anemo",
            Element::Geo => "Geo",
            Element::Dendro => "Dendro",
        }
    }
}
