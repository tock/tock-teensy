#[macro_export]
macro_rules! bitmasks {
    {
        $valtype:ty, [
            $( $field:ident $a:tt ),+
        ]
    } => {
        $( bitmasks!($valtype, $field, $a, []); )*
    };

    {
        $valtype:ty, [
            $( $field:ident $a:tt $b:tt ),+
        ]
    } => {
        $( bitmasks!($valtype, $field, $a, $b); )*
    };

    {
        $valtype:ty, $field:ident, 
                    ($mask:expr, $shift:expr), 
                    [$( $valname:ident = $value:expr ),*]
    } => {
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const $field: Field<$valtype> = Field::<$valtype>::new($mask, $shift);

        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod $field {
            #[allow(unused_imports)]
            use $crate::regs::FieldValue;

            $(
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const $valname: FieldValue<$valtype> = FieldValue::<$valtype>::new($mask, $shift, $value);
            )*

            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {
                $(
                    $valname = $value,
                )*
            }
        }
    };

    {
        $valtype:ty, $field:ident, $bit:expr, 
        [$( $valname:ident = $value:expr),* ]
    } => {
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const $field: Field<$valtype> = Field::<$valtype>::new(1, $bit);

        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod $field {
            #[allow(unused_imports)]
            use $crate::regs::FieldValue;

            $(
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const $valname: FieldValue<$valtype> = FieldValue::<$valtype>::new(1, $bit, $value);
            )*

            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<$valtype> = FieldValue::<$valtype>::new(1, $bit, 1);

            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<$valtype> = FieldValue::<$valtype>::new(1, $bit, 0);

            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {
                $(
                    $valname = $value,
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! bitfields {
    {
        $valtype:ty, $( $reg:ident $fields:tt ),*
    } => {
        $(
            #[allow(non_snake_case)]
            pub mod $reg {
                use $crate::regs::Field;
                bitmasks!( $valtype, $fields );
            }
        )*
    }
}
