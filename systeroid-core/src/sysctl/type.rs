/// Macro for generating enum type with a Default variant and common implementations.
macro_rules! gen_type_property {
    ($name: ident,
     $($variant: ident,)+
    ) => {
        /// Enum containing variants.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            /// Default variant.
            Default,
            $(
                /// Variant.
                $variant
            ),+
        }

        impl<'a> From<&'a str> for $name {
            fn from(value: &'a str) -> Self {
                for section in Self::variants() {
                    if value.to_lowercase() == section.to_string() {
                        return *section;
                    }
                }
                Self::Default
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::Default
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", format!("{:?}", self).to_lowercase())
            }
        }

        impl $name {
            /// Returns the variants.
            pub fn variants() -> &'static [Self] {
                &[Self::Default, $(Self::$variant),+]
            }
        }
    };
}

gen_type_property!(DisplayType, Name, Value, Binary,);
gen_type_property!(OutputType, Tree, Json,);

#[cfg(test)]
mod tests {
    #[test]
    fn test_gen_type() {
        gen_type_property!(TestType, One, Two, Three,);
        assert_eq!(TestType::Two, TestType::from("two"));
        assert_eq!(TestType::Two, TestType::from("TwO"));
        assert_eq!(TestType::Default, TestType::from("tw0"));
        assert_eq!(TestType::Default, TestType::default());
        assert_eq!("three", &TestType::Three.to_string());
        assert_eq!("one", &TestType::One.to_string());
        assert_eq!(
            &[
                TestType::Default,
                TestType::One,
                TestType::Two,
                TestType::Three
            ],
            TestType::variants()
        );
    }
}
