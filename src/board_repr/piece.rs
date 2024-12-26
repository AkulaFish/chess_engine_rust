use strum_macros::{AsRefStr, Display, EnumIter, EnumString, FromRepr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, Default)]
pub enum Color {
    #[default]
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
#[derive(Debug, Display, EnumString, AsRefStr, PartialEq, Eq, EnumIter, Clone, Copy, FromRepr)]
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

    #[strum(serialize = "None", to_string = "-")]
    None,
}

impl Piece {
    pub fn get_by_index(n: u8) -> Self {
        Self::from_repr(n).unwrap_or_else(|| panic!("Piece index out of range."))
    }

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

            Self::None => panic!("Can not get color of None piece."),
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

impl Piece {
    pub fn is_pawn(&self) -> bool {
        self == &Piece::WhitePawn || self == &Piece::BlackPawn
    }

    pub fn is_king(&self) -> bool {
        self == &Piece::WhiteKing || self == &Piece::BlackKing
    }

    pub fn is_bishop(&self) -> bool {
        self == &Piece::WhiteBishop || self == &Piece::BlackBishop
    }

    pub fn is_rook(&self) -> bool {
        self == &Piece::WhiteRook || self == &Piece::BlackRook
    }

    pub fn is_knight(&self) -> bool {
        self == &Piece::WhiteKnight || self == &Piece::BlackKnight
    }

    pub fn is_queen(&self) -> bool {
        self == &Piece::WhiteQueen || self == &Piece::BlackQueen
    }

    pub fn is_none(&self) -> bool {
        self == &Piece::None
    }
}

impl Piece {
    pub fn eval_value(&self) -> i16 {
        match self {
            Self::WhitePawn => 100,
            Self::WhiteKnight => 300,
            Self::WhiteBishop => 350,
            Self::WhiteRook => 500,
            Self::WhiteQueen => 1000,
            Self::WhiteKing => 10000,

            Self::BlackPawn => -100,
            Self::BlackKnight => -300,
            Self::BlackBishop => -350,
            Self::BlackRook => -500,
            Self::BlackQueen => -1000,
            Self::BlackKing => -10000,

            _ => 0,
        }
    }
}
