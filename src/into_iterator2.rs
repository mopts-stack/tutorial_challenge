struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct ColorIntoIter {
    color: Color,
    position: u8,
}

impl Iterator for ColorIntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };

        self.position += 1;
        next
    }
}

struct ColorIter<'a> {
    color: &'a Color,
    position: u8,
}

impl<'a> Iterator for ColorIter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };

        self.position += 1;
        next
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = ColorIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            color: self,
            position: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Color {
    type Item = u8;
    type IntoIter = ColorIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            color: self,
            position: 0,
        }
    }
}

pub fn start() {
    let color = Color {
        r: 10,
        g: 20,
        b: 30,
    };

    for c in &color {
        println!("{}", c);
    }

    for c in color {
        println!("{}", c);
    }
}
