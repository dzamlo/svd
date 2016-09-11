macro_rules! join_expected_strs {
    ($s1:expr, $s2:expr, ) => {
        concat!($s1, " or ", $s2)
    };
    ($s:expr, $( $str:expr, )*) => {
        concat!($s, ", ", join_expected_strs!($($str,)*))
    };
}

macro_rules! str_enum {
    ( $name:ident,  $( $str:expr => $variant:ident, )* ) => {
       str_enum!{$name, $( $str => $variant ),* }
    };
    ( $name:ident,  $( $str:expr => $variant:ident ),* ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $($variant),*
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<$name> {
                match s {
                    $($str => Ok($name::$variant), )*
                    _ => Err(
                        ErrorKind::UnexpectedValue(
                            concat!("one of ", join_expected_strs!($($str,)*)),
                            s.to_string()
                        ).into()
                    )
                }

            }
        }
    };
}
