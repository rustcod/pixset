use pixset::Pix;
use std::default::Default;

impl Pix {
    fn ch_to_pix(ch: char) -> Pix {
        match ch {
            'a' => Pix::A,
            'b' => Pix::B,
            'c' => Pix::C,
            'd' => Pix::D,
            'e' => Pix::E,
            'f' => Pix::F,
            'g' => Pix::G,
            'h' => Pix::H,
            'i' => Pix::I,
            'j' => Pix::J,
            'k' => Pix::K,
            'l' => Pix::L,
            'm' => Pix::M,
            'n' => Pix::N,
            'o' => Pix::O,
            'p' => Pix::P,
            'q' => Pix::Q,
            'r' => Pix::R,
            's' => Pix::S,
            't' => Pix::T,
            'u' => Pix::U,
            'v' => Pix::V,
            'w' => Pix::W,
            'x' => Pix::X,
            'y' => Pix::Y,
            'z' => Pix::Z,
            'A' => Pix::A,
            'B' => Pix::B,
            'C' => Pix::C,
            'D' => Pix::D,
            'E' => Pix::E,
            'F' => Pix::F,
            'G' => Pix::G,
            'H' => Pix::H,
            'I' => Pix::I,
            'J' => Pix::J,
            'K' => Pix::K,
            'L' => Pix::L,
            'M' => Pix::M,
            'N' => Pix::N,
            'O' => Pix::O,
            'P' => Pix::P,
            'Q' => Pix::Q,
            'R' => Pix::R,
            'S' => Pix::S,
            'T' => Pix::T,
            'U' => Pix::U,
            'V' => Pix::V,
            'W' => Pix::W,
            'X' => Pix::X,
            'Y' => Pix::Y,
            'Z' => Pix::Z,
            ' ' => Pix::Empty,
            '#' => Pix::Hash,
            '.' => Pix::Period,
            ',' => Pix::Comma,
            '"' => Pix::Quotes,
            '\'' => Pix::Apostrophe,
            ':' => Pix::Colon,
            ';' => Pix::SemiColon,
            '0' => Pix::Zero,
            '1' => Pix::One,
            '2' => Pix::Two,
            '3' => Pix::Three,
            '4' => Pix::Four,
            '5' => Pix::Five,
            '6' => Pix::Six,
            '7' => Pix::Seven,
            '8' => Pix::Eight,
            '9' => Pix::Nine,
            _ => Pix::Empty,
        }
    }
}

pub struct Str<'a>(&'a str);

impl<'a> Str<'a> {
    pub fn iter(&self) -> FontIter {
        FontIter::new(self.0)
    }
}

pub struct FontIter<'a> {
    idx: usize,
    string: &'a str,
}

impl<'a> FontIter<'a> {
    fn new(s: &'a str) -> Self {
        FontIter {
            idx: Default::default(),
            string: s,
        }
    }
}

impl<'a> Iterator for FontIter<'a> {
    type Item = (Pix, (i32, i32));

    fn next(&mut self) -> Option<(Pix, (i32, i32))> {
        if self.idx == self.string.len() {
            None
        } else {
            // TODO can this be faster?
            let ch = self.string
                .chars()
                .nth(self.idx)
                .expect("no char at that index");
            let x = self.idx;
            self.idx += 1;
            Some((Pix::ch_to_pix(ch), (x as i32, 0)))
        }
    }
}

impl<'a> From<&'a str> for Str<'a> {
    fn from(s: &'a str) -> Self {
        Str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iteration() {
        let v = Str::from("Yo ,").iter().collect::<Vec<_>>();
        assert_eq!(v[0], (Pix::Y, (0, 0)));
        assert_eq!(v[1], (Pix::O, (1, 0)));
        assert_eq!(v[2], (Pix::Empty, (2, 0)));
        assert_eq!(v[3], (Pix::Comma, (3, 0)));
    }
}
