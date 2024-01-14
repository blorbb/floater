#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Side {
    Left,
    Top,
    Right,
    #[default]
    Bottom,
}

impl Side {
    /// Returns the side opposite to the current.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
        }
    }

    /// Returns the side 90 degrees clockwise of the current.
    #[must_use]
    pub const fn clockwise(self) -> Self {
        match self {
            Self::Left => Self::Top,
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
        }
    }

    /// Returns the side 90 degrees anticlockwise of the current.
    #[must_use]
    pub const fn anticlockwise(self) -> Self {
        match self {
            Self::Left => Self::Bottom,
            Self::Top => Self::Left,
            Self::Right => Self::Top,
            Self::Bottom => Self::Right,
        }
    }

    /// Returns the two sides adjacent to the current. This will be in the order
    /// of the side anticlockwise, then clockwise.
    #[must_use]
    pub const fn adjacents(self) -> [Self; 2] { [self.anticlockwise(), self.clockwise()] }

    /// Returns the orientation of a side.
    ///
    /// Left/right => vertical, top/bottom => horizontal.
    #[must_use]
    pub const fn orientation(self) -> Orientation {
        match self {
            Self::Left | Self::Right => Orientation::Vertical,
            Self::Top | Self::Bottom => Orientation::Horizontal,
        }
    }
}

/// The orientation of a [`Side`], either horizontal or vertical.
///
/// Created by [`Side::orientation`].
pub enum Orientation {
    Horizontal,
    Vertical,
}
