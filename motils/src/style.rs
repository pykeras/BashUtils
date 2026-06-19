use std::fmt::{self, Display};

#[allow(unused)]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum FgColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

#[allow(unused)]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum BgColor {
    Black = 40,
    Red = 41,
    Green = 42,
    Yellow = 43,
    Blue = 44,
    Magenta = 45,
    Cyan = 46,
    White = 47,
    BrightBlack = 100,
    BrightRed = 101,
    BrightGreen = 102,
    BrightYellow = 103,
    BrightBlue = 104,
    BrightMagenta = 105,
    BrightCyan = 106,
    BrightWhite = 107,
}

#[derive(Debug)]
pub struct Styled<T> {
    inner: T,
    fg: Option<FgColor>,
    bg: Option<BgColor>,
    bold: bool,
}

impl<T: Display> Styled<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            fg: None,
            bg: None,
            bold: false,
        }
    }

    pub fn fg(mut self, color: FgColor) -> Self {
        self.fg = Some(color);
        self
    }
    pub fn bg(mut self, color: BgColor) -> Self {
        self.bg = Some(color);
        self
    }
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    // Convenience methods
    pub fn bold_white_on_blue(self) -> Self {
        self.fg(FgColor::White).bg(BgColor::Blue).bold()
    }
    pub fn cyan(self) -> Self {
        self.fg(FgColor::Cyan)
    }
    pub fn green(self) -> Self {
        self.fg(FgColor::Green)
    }
    pub fn yellow(self) -> Self {
        self.fg(FgColor::Yellow)
    }
    pub fn red(self) -> Self {
        self.fg(FgColor::Red)
    }
    pub fn magenta(self) -> Self {
        self.fg(FgColor::Magenta)
    }
}

pub trait Colorize: Display + Sized {
    fn styled(self) -> Styled<Self> {
        Styled::new(self)
    }
}

impl<T: Display> Colorize for T {}

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let bold: u8 = if self.bold { 1 } else { 0 };
        match (self.fg, self.bg) {
            (Some(fg), Some(bg)) => {
                write!(
                    f,
                    "\x1b[{};{};{}m{}\x1b[0m",
                    bold, fg as u8, bg as u8, self.inner
                )
            }
            (Some(fg), None) => write!(f, "\x1b[{};{}m{}\x1b[0m", bold, fg as u8, self.inner,),
            (None, Some(bg)) => write!(f, "\x1b[{};{}m{}\x1b[0m", bold, bg as u8, self.inner),
            (None, None) => write!(f, "\x1b[{}m{}\x1b[0m", bold, self.inner),
        }
    }
}
