use std::convert::TryFrom;

macro_rules! generate_option {
    ($name: ident,
     $($variant: ident => $str_repr: expr,)+
    ) => {
        /// Available options.
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum $name {
            $(
                /// Option.
                $variant
            ),+
        }

        impl<'a> TryFrom<&'a str> for $name {
            type Error = ();
            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                Self::variants()
                    .iter()
                    .find(|v| value == v.as_str())
                    .copied()
                    .ok_or(())
            }
        }

        impl $name {
            /// Returns the variants.
            pub fn variants() -> &'static [Self] {
                &[$(Self::$variant,)+]
            }

            /// Returns the string representation.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $str_repr,)+
                }
            }
        }
    };
}

generate_option!(
    CopyOption,
    Name => "Parameter name",
    Value => "Parameter value",
    Documentation => "Documentation",
);

generate_option!(
    Direction,
    Up => "up",
    Right => "right",
    Down => "down",
    Left => "left",
    Top => "top",
    Bottom => "bottom",
);

generate_option!(
    ScrollArea,
    List => "list",
    Documentation => "docs",
    Section => "section",
);
