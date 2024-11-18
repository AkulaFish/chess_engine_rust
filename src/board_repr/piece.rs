use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display)]
pub enum Color {
    White,
    Black,
    Both,
}

impl Color {
    pub fn opposite(&self) -> Self {
        if self == &Self::White {
            return Self::Black;
        }

        Self::White
    }
}

#[repr(u8)]
#[derive(Debug, Display, EnumString, AsRefStr, PartialEq, Eq, EnumIter, Clone, Copy)]
pub enum Piece {
    #[strum(serialize = "P", to_string = "♙")]
    WhitePawn,
    #[strum(serialize = "Q", to_string = "♕")]
    WhiteQueen,
    #[strum(serialize = "K", to_string = "♔")]
    WhiteKing,
    #[strum(serialize = "B", to_string = "♗")]
    WhiteBishop,
    #[strum(serialize = "N", to_string = "♘")]
    WhiteKnight,
    #[strum(serialize = "R", to_string = "♖")]
    WhiteRook,

    #[strum(serialize = "p", to_string = "♟")]
    BlackPawn,
    #[strum(serialize = "q", to_string = "♛")]
    BlackQueen,
    #[strum(serialize = "k", to_string = "♚")]
    BlackKing,
    #[strum(serialize = "b", to_string = "♝")]
    BlackBishop,
    #[strum(serialize = "n", to_string = "♞")]
    BlackKnight,
    #[strum(serialize = "r", to_string = "♜")]
    BlackRook,
}

impl Piece {
    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn color(&self) -> Color {
        if self.value() > 5 {
            return Color::Black;
        }

        Color::White
    }

    pub fn opposite_color(&self) -> Self {
        match self {
            Self::WhitePawn => Self::BlackPawn,
            Self::WhiteQueen => Self::BlackQueen,
            Self::WhiteKing => Self::BlackKing,
            Self::WhiteBishop => Self::BlackBishop,
            Self::WhiteKnight => Self::BlackKnight,
            Self::WhiteRook => Self::BlackRook,

            Self::BlackPawn => Self::WhitePawn,
            Self::BlackQueen => Self::WhiteQueen,
            Self::BlackKing => Self::WhiteKing,
            Self::BlackBishop => Self::WhiteBishop,
            Self::BlackKnight => Self::WhiteKnight,
            Self::BlackRook => Self::WhiteRook,
        }
    }

    pub fn to_color(&self, color: Color) -> Self {
        match (self, color) {
            (piece, color) if piece.color() == color => *self,

            (_, Color::White) => match self {
                Self::BlackPawn => Self::WhitePawn,
                Self::BlackQueen => Self::WhiteQueen,
                Self::BlackKing => Self::WhiteKing,
                Self::BlackBishop => Self::WhiteBishop,
                Self::BlackKnight => Self::WhiteKnight,
                Self::BlackRook => Self::WhiteRook,
                _ => *self,
            },

            (_, Color::Black) => match self {
                Self::WhitePawn => Self::BlackPawn,
                Self::WhiteQueen => Self::BlackQueen,
                Self::WhiteKing => Self::BlackKing,
                Self::WhiteBishop => Self::BlackBishop,
                Self::WhiteKnight => Self::BlackKnight,
                Self::WhiteRook => Self::BlackRook,
                _ => *self,
            },

            (_, Color::Both) => *self,
        }
    }
}
