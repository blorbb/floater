#[derive(Debug, Default, Clone, Copy)]
pub struct Padding {
    pub outward: f64,
    pub inward: f64,
    pub cross: f64,
}

impl Padding {
    #[must_use]
    pub const fn splat(value: f64) -> Self {
        Self {
            outward: value,
            inward: value,
            cross: value,
        }
    }
}

impl From<f64> for Padding {
    fn from(value: f64) -> Self {
        Self::splat(value)
    }
}

#[macro_export]
macro_rules! impl_padding_builder {
    ($path:ident) => {
        #[must_use]
        pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
            self.$path = padding.into();
            self
        }

        #[must_use]
        pub const fn padding_inward(mut self, padding: f64) -> Self {
            self.$path.inward = padding;
            self
        }

        #[must_use]
        pub const fn padding_outward(mut self, padding: f64) -> Self {
            self.$path.outward = padding;
            self
        }

        #[must_use]
        pub const fn padding_cross(mut self, padding: f64) -> Self {
            self.$path.cross = padding;
            self
        }

        #[must_use]
        pub const fn padding_main(self, padding: f64) -> Self {
            self.padding_outward(padding).padding_inward(padding)
        }
    };
}
