use colour::ansi::AnsiColour;
use terminal::Style;

#[derive(Clone, Copy, Debug)]
pub enum SimpleTile {
    SolidColour(AnsiColour),
    Foreground(char, AnsiColour, Style),
    Full {
        ch: char,
        fg: AnsiColour,
        bg: AnsiColour,
        style: Style,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum ComplexTile {
    Simple(SimpleTile),
    Wall { front: SimpleTile, back: SimpleTile },
}

pub fn solid_colour(colour: AnsiColour) -> ComplexTile {
    ComplexTile::Simple(SimpleTile::SolidColour(colour))
}

pub fn foreground(ch: char, colour: AnsiColour, style: Style) -> ComplexTile {
    ComplexTile::Simple(SimpleTile::Foreground(ch, colour, style))
}

pub fn full(ch: char, fg: AnsiColour, bg: AnsiColour, style: Style) -> ComplexTile {
    ComplexTile::Simple(SimpleTile::Full {
        ch: ch,
        fg: fg,
        bg: bg,
        style: style,
    })
}

impl SimpleTile {
    pub fn opaque_bg(self) -> bool {
        match self {
            SimpleTile::SolidColour(_) => true,
            SimpleTile::Foreground(_, _, _) => false,
            SimpleTile::Full { ch: _, fg: _, bg: _, style: _ } => true,
        }
    }

    pub fn background_colour(self) -> Option<AnsiColour> {
        match self {
            SimpleTile::SolidColour(c) => Some(c),
            SimpleTile::Foreground(_, _, _) => None,
            SimpleTile::Full { ch: _, fg: _, bg, style: _ } => Some(bg),
        }
    }

    pub fn foreground_colour(self) -> Option<AnsiColour> {
        match self {
            SimpleTile::SolidColour(_) => None,
            SimpleTile::Foreground(_, c, _) => Some(c),
            SimpleTile::Full { ch: _, fg, bg: _, style: _ } => Some(fg),
        }
    }

    pub fn character(self) -> Option<char> {
        match self {
            SimpleTile::SolidColour(_) => None,
            SimpleTile::Foreground(ch, _, _) => Some(ch),
            SimpleTile::Full { ch, fg: _, bg: _, style: _ } => Some(ch),
        }
    }

    pub fn style(self) -> Option<Style> {
        match self {
            SimpleTile::SolidColour(_) => None,
            SimpleTile::Foreground(_, _, style) => Some(style),
            SimpleTile::Full { ch: _, fg: _, bg: _, style } => Some(style),
        }
    }
}

impl ComplexTile {
    pub fn opaque_bg(self) -> bool {
        match self {
            ComplexTile::Simple(s) => s.opaque_bg(),
            ComplexTile::Wall { front, back } => front.opaque_bg() || back.opaque_bg(),
        }
    }
}
