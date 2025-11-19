pub mod list;

pub mod value;

pub mod tape;

pub mod instruction;

#[macro_export]
macro_rules! types_fuckery {
    (
        [ $($instr:tt)*    ]
        [ $($input:ident)* ]
    ) => {
        <
            <
                $crate::types_fuckery! {
                    @cons_list [ $(
                        $crate::types_fuckery! { @instr $instr }
                    ),* ]
                }
                as $crate::instruction::Instruction<
                    $crate::tape::Tape<
                        $crate::list::Nil,
                        $crate::value::V00,
                        $crate::list::Nil,
                        $crate::types_fuckery! {
                            @cons_list [ $( $crate::value::$input ),* ]
                        },
                        $crate::list::Nil
                    >
                >
            >::Output
            as $crate::tape::HasOutput
        >::Output
    };

    ( @cons_list [ $first:ty , $( $element:ty ),* ] ) => {
        $crate::list::Cons<
            $first,
            $crate::types_fuckery! { @cons_list [ $( $element ),* ] }
        >
    };

    ( @cons_list [ $first:ty ] ) => {
        $crate::list::Cons<
            $first,
            $crate::list::Nil
        >
    };

    ( @cons_list [ ] ) => { $crate::list::Nil };

    ( @instr , ) => { $crate::instruction::Input };

    ( @instr . ) => { $crate::instruction::Output };

    ( @instr + ) => { $crate::instruction::Increase };

    ( @instr - ) => { $crate::instruction::Decrease };

    ( @instr > ) => { $crate::instruction::Right };

    ( @instr < ) => { $crate::instruction::Left };

    ( @instr [ $( $body:tt )* ] ) => { $crate::instruction::Loop<
             $crate::types_fuckery! {
                @cons_list [ $(
                    $crate::types_fuckery! { @instr $body }
                ),* ]
            }
    > };
}
