use std;
use traits::PixLike;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PixLike)]
#[size = "16"]
#[total = "100"]
pub enum Pix {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    DownArrow,
    LeftArrow,
    Dood,
    Percent,
    UpArrow,
    RightArrow,
    Hash,
    Period,
    Comma,
    Quotes,
    Apostrophe,
    Colon,
    SemiColon,
    Emptya,
    LeftTopCorner,
    RightTopCorner,
    LeftBottomCorner,
    RightBottomCorner,
    LeftStraight,
    RightStraight,
    TopStraight,
    BottomStraight,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Empty,
}