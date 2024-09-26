use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Display, EnumString, AsRefStr, PartialEq, Eq, EnumIter)]
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
