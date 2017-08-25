#[macro_export]
macro_rules! bitmasks {
    {
        $valtype:ty, [
            $(
                $field:ident ($mask:expr, $shift:expr) [
                    $( $valname:ident = $value:expr ),*
                ]
            ),*
        ]
    } => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $field: FieldMask<$valtype> = FieldMask::new($mask, $shift);

            #[allow(non_snake_case)]
            #[allow(unused)]
            pub mod $field {
                #[allow(unused_imports)]
                use $crate::regs::FieldValue;

                $(
                #[allow(non_upper_case_globals)]
                pub const $valname: FieldValue<$valtype> = FieldValue::<$valtype>::new($mask, $shift, $value);
                )*

                #[allow(non_upper_case_globals)]
                pub const True: FieldValue<$valtype> = FieldValue::<$valtype>::new($mask, $shift, 1);

                #[allow(non_upper_case_globals)]
                pub const False: FieldValue<$valtype> = FieldValue::<$valtype>::new($mask, $shift, 0);

                #[allow(dead_code)]
                #[allow(non_camel_case_types)]
                pub enum Value {
                    $(
                        $valname = $value,
                    )*
                }
            }
        )*
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
                use $crate::regs::FieldMask;
                bitmasks!( $valtype, $fields );
            }
        )*
    }
}
