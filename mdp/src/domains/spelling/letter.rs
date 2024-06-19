use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, EnumIter)]
pub enum Letter {
    A,
    M,
    R,
    S,
}

impl Letter {
    pub fn toggle(&self) -> Letter {
        match self {
            Letter::A => Letter::M,
            Letter::M => Letter::R,
            Letter::R => Letter::S,
            Letter::S => Letter::A,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Letter::A => 'A',
            Letter::M => 'M',
            Letter::R => 'R',
            Letter::S => 'S',
        }
    }
}
