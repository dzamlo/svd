macro_rules! str_enum {
    ( $name:ident,  $( $str:expr => $variant:ident, )* ) => {
       str_enum!{$name, $( $str => $variant ),* }
    };
    ( $name:ident,  $( $str:expr => $variant:ident ),* ) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $($variant),*
        }

        impl FromStr for $name {
            type Err = FromElementError;

            fn from_str(s: &str) -> Result<$name, FromElementError> {
                match s {
                    $($str => Ok($name::$variant), )*
                    _ => Err(FromElementError::InvalidFormat),
                }

            }
        }
    };
}
