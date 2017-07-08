use std;
use std::fmt::Debug;

pub trait PixLike: Default + Sized + Copy + Clone + Debug {
    fn pix_order() -> Vec<Self>;
    fn tile_size(&self) -> (u32, u32);
    fn get(&self) -> (f32, f32, f32, f32);
}

#[macro_export]
macro_rules! pix {
    (tileset => $ts:expr;
     width => $w:expr;
     height => $h:expr;
     total => $t:expr;
     $($e:ident),+;
     $($ch:expr => $e2:ident),+;) => {
        use std::str;

        pub static TILESET: &'static [u8] = include_bytes!($ts);

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PixLike)]
        #[size(width = $w, height = $h)]
        #[total = $t]
        pub enum Pix {
            $($e),+,
        }

        impl Pix {
            fn ch_to_pix(ch: char) -> Pix {
                match ch {
                    $($ch => Pix::$e2),+,
                    _ => Pix::Empty,
                }
            }
        }

        pub struct PixStr<'a>(&'a str);

        impl<'a> PixStr<'a> {
            pub fn iter(&self) -> FontIter {
                FontIter::new(self.0)
            }
        }

        pub struct FontIter<'a> {
            offset: usize,
            chars: str::Chars<'a>,
        }

        impl<'a> FontIter<'a> {
            fn new(s: &'a str) -> Self {
                FontIter {
                    offset: 0,
                    chars: s.chars(),
                }
            }
        }

        impl<'a> Iterator for FontIter<'a> {
            type Item = (Pix, (i32, i32));

            fn next(&mut self) -> Option<(Pix, (i32, i32))> {
                match self.chars.next() {
                    None => None,
                    Some(ch) => {
                        let offset = self.offset;
                        self.offset += 1;
                        Some((Pix::ch_to_pix(ch), (offset as i32, 0)))
                    }
                }
            }
        }

        impl<'a> From<&'a str> for PixStr<'a> {
            fn from(s: &'a str) -> Self {
                PixStr(s)
            }
        }
    }
}

pix! {
    tileset => "../assets/tileset.png";
    width => "16";
    height => "16";
    total => "100";
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
    Empty;
    'a' => A,
    'b' => B,
    'c' => C,
    'd' => D,
    'e' => E,
    'f' => F,
    'g' => G,
    'h' => H,
    'i' => I,
    'j' => J,
    'k' => K,
    'l' => L,
    'm' => M,
    'n' => N,
    'o' => O,
    'p' => P,
    'q' => Q,
    'r' => R,
    's' => S,
    't' => T,
    'u' => U,
    'v' => V,
    'w' => W,
    'x' => X,
    'y' => Y,
    'z' => Z,
    'A' => A,
    'B' => B,
    'C' => C,
    'D' => D,
    'E' => E,
    'F' => F,
    'G' => G,
    'H' => H,
    'I' => I,
    'J' => J,
    'K' => K,
    'L' => L,
    'M' => M,
    'N' => N,
    'O' => O,
    'P' => P,
    'Q' => Q,
    'R' => R,
    'S' => S,
    'T' => T,
    'U' => U,
    'V' => V,
    'W' => W,
    'X' => X,
    'Y' => Y,
    'Z' => Z,
    ' ' => Empty,
    '#' => Hash,
    '.' => Period,
    ',' => Comma,
    '"' => Quotes,
    ':' => Colon,
    ';' => SemiColon,
    '0' => Zero,
    '1' => One,
    '2' => Two,
    '3' => Three,
    '4' => Four,
    '5' => Five,
    '6' => Six,
    '7' => Seven,
    '8' => Eight,
    '9' => Nine,
    '\'' => Apostrophe;
}
