
use std::fmt;
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}
pub struct Image {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<Color>>
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n")?;
        write!(f, "{} {}\n", self.width, self.height)?;
        write!(f, "255\n")?;
        
        for row in &self.data {
            for color in row {
                write!(f, "{}", color)?;
            }
        }

        Ok(())
    }
}