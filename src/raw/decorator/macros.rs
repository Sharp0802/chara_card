macro_rules! decorators {
    (@ty number) => { i64 };
    (@ty boolean) => { bool };
    (@ty string) => { std::ops::Range<usize> };
    (@ty $ty:tt) => { $ty };

    // Parsing helpers
    (@parse string $base:ident $arg:ident) => {
        {
            let start = $arg.as_ptr() as usize - $base.as_ptr() as usize;
            start..(start + $arg.len())
        }
    };
    (@parse $ty:tt $base:ident $arg:ident) => { $arg.parse()? };

    // Formatting helpers
    (@fmt string $base:ident $val:ident) => { &$base[$val.clone()] };
    (@fmt $ty:tt $base:ident $val:ident) => { $val };

    (@munch
        [$base_id:ident, $args_id:ident]
        [$($variants:tt)*]
        [$($arms:tt)*]
        [$($to_string_arms:tt)*] // <-- Added accumulator for to_string
        $name:ident { $($arg:ident : $ty:tt),+ } ,
        $($rest:tt)*
    ) => {
        decorators! { @munch
            [$base_id, $args_id]
            [$($variants)* $name { $($arg : decorators!{ @ty $ty }),+ } ,]
            [$($arms)*
                paste::paste!{ stringify!([< $name:snake >]) } => {
                    $(
                        let $arg = $args_id.next().ok_or(Error::InsufficientArgument)?;
                        let $arg : decorators!{ @ty $ty } = decorators!{ @parse $ty $base_id $arg };
                    )+
                    if $args_id.next().is_some() {
                        return Err(Error::ExtraneousArgument);
                    }
                    Ok(Self::$name { $($arg),+ })
                }
            ]
            [$($to_string_arms)*
                Self::$name { $($arg),+ } => {
                    let name_str = paste::paste!{ stringify!([< $name:snake >]) };
                    let args_str = vec![
                        $( format!("{}", decorators!(@fmt $ty $base_id $arg)) ),+
                    ].join(",");
                    format!("{} {}", name_str, args_str)
                }
            ]
            $($rest)*
        }
    };

    (@munch
        [$base_id:ident, $args_id:ident]
        [$($variants:tt)*]
        [$($arms:tt)*]
        [$($to_string_arms:tt)*]
        $name:ident ( $ty:tt ) ,
        $($rest:tt)*
    ) => {
        decorators! { @munch
            [$base_id, $args_id]
            [$($variants)* $name ( decorators!{ @ty $ty } ) ,]
            [$($arms)*
                paste::paste!{ stringify!([< $name:snake >]) } => {
                    let value = $args_id.next().ok_or(Error::InsufficientArgument)?;
                    let value : decorators!{ @ty $ty } = decorators!{ @parse $ty $base_id value };
                    if $args_id.next().is_some() {
                        return Err(Error::ExtraneousArgument);
                    }
                    Ok(Self::$name(value))
                }
            ]
            [$($to_string_arms)*
                Self::$name(value) => {
                    let name_str = paste::paste!{ stringify!([< $name:snake >]) };
                    let arg_str = format!("{}", decorators!(@fmt $ty $base_id value));
                    format!("{} {}", name_str, arg_str)
                }
            ]
            $($rest)*
        }
    };

    (@munch
        [$base_id:ident, $args_id:ident]
        [$($variants:tt)*]
        [$($arms:tt)*]
        [$($to_string_arms:tt)*]
        $name:ident ,
        $($rest:tt)*
    ) => {
        decorators! { @munch
            [$base_id, $args_id]
            [$($variants)* $name ,]
            [$($arms)*
                paste::paste!{ stringify!([< $name:snake >]) } => {
                    if $args_id.next().is_some() {
                        return Err(Error::ExtraneousArgument);
                    }
                    Ok(Self::$name)
                }
            ]
            [$($to_string_arms)*
                Self::$name => {
                    paste::paste!{ stringify!([< $name:snake >]) }.to_string()
                }
            ]
            $($rest)*
        }
    };

    // Missing comma fallbacks
    (@munch [$base_id:ident, $args_id:ident] [$($v:tt)*] [$($a:tt)*] [$($t:tt)*] $name:ident { $($arg:ident : $ty:tt),+ }) => {
        decorators! { @munch [$base_id, $args_id] [$($v)*] [$($a)*] [$($t)*] $name { $($arg : $ty),+ } , }
    };
    (@munch [$base_id:ident, $args_id:ident] [$($v:tt)*] [$($a:tt)*] [$($t:tt)*] $name:ident ( $ty:tt )) => {
        decorators! { @munch [$base_id, $args_id] [$($v)*] [$($a)*] [$($t)*] $name ( $ty ) , }
    };
    (@munch [$base_id:ident, $args_id:ident] [$($v:tt)*] [$($a:tt)*] [$($t:tt)*] $name:ident) => {
        decorators! { @munch [$base_id, $args_id] [$($v)*] [$($a)*] [$($t)*] $name , }
    };

    // The Finalizer
    (@munch
        [$base_id:ident, $args_id:ident]
        [$($variants:tt)*]
        [$($arms:tt)*]
        [$($to_string_arms:tt)*]
    ) => {
        #[derive(Debug, Clone)]
        pub enum DecoratorKind {
            $($variants)*

            Misc { name: std::ops::Range<usize>, args: Vec<std::ops::Range<usize>> },
        }

        impl DecoratorKind {
            pub fn parse_line($base_id: &str, line: &str) -> Result<Self, Error> {
                if let Some(at) = line.find(' ') {
                    let (name, args) = line.split_at(at);
                    let args = args.split(',').map(|s| s.trim());

                    Self::parse($base_id, name, args)
                } else {
                    Self::parse($base_id, line.trim(), std::iter::empty::<&str>())
                }
            }

            fn parse<'a>($base_id: &'a str, name: &'a str, mut $args_id: impl Iterator<Item = &'a str>) -> Result<Self, Error> {
                match name {
                    $($arms)*

                    _ => {
                        let name_start = name.as_ptr() as usize - $base_id.as_ptr() as usize;

                        Ok(Self::Misc {
                            name: name_start..(name_start + name.len()),
                            args: $args_id.map(|arg| {
                                let start = arg.as_ptr() as usize - $base_id.as_ptr() as usize;
                                start..(start + arg.len())
                            }).collect()
                        })
                    }
                }
            }

            // The generated method!
            pub fn to_string(&self, $base_id: &str) -> String {
                match self {
                    $($to_string_arms)*

                    Self::Misc { name, args } => {
                        let name_str = &$base_id[name.clone()];
                        if args.is_empty() {
                            name_str.to_string()
                        } else {
                            let args_str = args.iter()
                                .map(|arg| &$base_id[arg.clone()])
                                .collect::<Vec<_>>()
                                .join(",");

                            format!("{} {}", name_str, args_str)
                        }
                    }
                }
            }
        }
    };

    ($($rest:tt)*) => {
        decorators! { @munch
            [base, args]
            []
            []
            [] // Start the to_string accumulator empty
            $($rest)*
        }
    };
}

pub(crate) use decorators;
