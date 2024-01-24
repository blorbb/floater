/// One of the four sides of a rectangle.
///
/// Also used as a direction.
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
    pub fn adjacents(self) -> std::array::IntoIter<Self, 2> {
        [self.anticlockwise(), self.clockwise()].into_iter()
    }

    /// Returns the axis a side points toward.
    ///
    /// Left/right => horizontal, top/bottom => vertical.
    #[must_use]
    pub const fn axis(self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::Horizontal,
            Self::Top | Self::Bottom => Axis::Vertical,
        }
    }

    /// Returns the CSS inset property of the given side, i.e. the side name in
    /// all lowercase.
    #[cfg(feature = "web-utils")]
    #[must_use]
    pub const fn as_css_prop(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
        }
    }
}

/// The direction a [`Side`] points toward, either horizontal or vertical.
///
/// Created by [`Side::axis`].
pub enum Axis {
    Vertical,
    Horizontal,
}
