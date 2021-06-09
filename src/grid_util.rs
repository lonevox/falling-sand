use derive_more::{Add, Sub, Mul, Div};

/* ---------- Position ---------- */

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Add, Sub, Mul, Div)]
#[mul(forward)]
#[div(forward)]
pub(crate) struct Position { pub x: i32, pub y: i32, }

impl Position {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    pub(crate) const ZERO: Self = Self {x: 0, y: 0};

    /// Returns a new Position at this Position's x and y values plus the given dx and dy.
    pub(crate) fn move_by(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl From<Size> for Position {
    fn from(size: Size) -> Self {
        Self::new(size.width as i32, size.height as i32)
    }
}

/* ---------- Size ---------- */

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Add, Sub, Mul, Div)]
#[mul(forward)]
#[div(forward)]
pub(crate) struct Size { pub width: u32, pub height: u32 }

impl Size {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {width, height}
    }

    pub(crate) const ZERO: Self = Self {width: 0, height: 0};
}

/* ---------- Rectangle ---------- */

pub(crate) struct Rectangle {
    pub top_left: Position,
    pub top_right: Position,
    pub bottom_left: Position,
    pub bottom_right: Position,
    pub size: Size,
}

impl Rectangle {
    pub(crate) fn new(position: Position, size: Size) -> Self {
        Self {
            top_left: position,
            top_right: position.move_by(size.width as i32, 0),
            bottom_left: position.move_by(0, size.height as i32),
            bottom_right: position.move_by(size.width as i32, size.height as i32),
            size,
        }
    }

    pub(crate) fn new_from_parts(x: i32, y: i32, width: u32, height: u32) -> Self {
        let position: Position = Position::new(x, y);
        let size: Size = Size::new(width, height);
        Self::new(position, size)
    }

    pub(crate) fn contains_position(&self, position: Position) -> bool {
        position > self.top_left && position < self.bottom_right
    }

    pub(crate) fn contains_rectangle(&self, rectangle: Rectangle) -> bool {
        rectangle.top_left > self.top_left && rectangle.bottom_right < self.bottom_right
    }
}