pub trait Value {
    type Next;
    type Prev;
}

pub trait IsZero {}
pub trait IsNotZero {}

macro_rules! cell_values {
    (
        $first:ident $second:ident $( $other:ident )*
    ) => {
        cell_values!{
            @this [ @IsNotZero $second $( @IsNotZero $other )* @IsZero $first ]
            @prev [ $first $second $( $other )* ]
            @next [ $( $other )* $first $second ]
        }
    };
    (
        @this [ $( @$zero:ident $this:ident )* ]
        @prev [ $( $prev:ident )* ]
        @next [ $( $next:ident )* ]
    ) => {
        $(
            pub struct $this;
            impl Value for $this {
                type Prev = $prev;
                type Next = $next;
            }

            impl $zero for $this {}
        )*
    };
}

cell_values! {
    V00 V01 V02 V03 V04 V05 V06 V07 V08 V09 V0A V0B V0C V0D V0E V0F
    V10 V11 V12 V13 V14 V15 V16 V17 V18 V19 V1A V1B V1C V1D V1E V1F
    V20 V21 V22 V23 V24 V25 V26 V27 V28 V29 V2A V2B V2C V2D V2E V2F
    V30 V31 V32 V33 V34 V35 V36 V37 V38 V39 V3A V3B V3C V3D V3E V3F
    V40 V41 V42 V43 V44 V45 V46 V47 V48 V49 V4A V4B V4C V4D V4E V4F
    V50 V51 V52 V53 V54 V55 V56 V57 V58 V59 V5A V5B V5C V5D V5E V5F
    V60 V61 V62 V63 V64 V65 V66 V67 V68 V69 V6A V6B V6C V6D V6E V6F
    V70 V71 V72 V73 V74 V75 V76 V77 V78 V79 V7A V7B V7C V7D V7E V7F
    V80 V81 V82 V83 V84 V85 V86 V87 V88 V89 V8A V8B V8C V8D V8E V8F
    V90 V91 V92 V93 V94 V95 V96 V97 V98 V99 V9A V9B V9C V9D V9E V9F
    VA0 VA1 VA2 VA3 VA4 VA5 VA6 VA7 VA8 VA9 VAA VAB VAC VAD VAE VAF
    VB0 VB1 VB2 VB3 VB4 VB5 VB6 VB7 VB8 VB9 VBA VBB VBC VBD VBE VBF
    VC0 VC1 VC2 VC3 VC4 VC5 VC6 VC7 VC8 VC9 VCA VCB VCC VCD VCE VCF
    VD0 VD1 VD2 VD3 VD4 VD5 VD6 VD7 VD8 VD9 VDA VDB VDC VDD VDE VDF
    VE0 VE1 VE2 VE3 VE4 VE5 VE6 VE7 VE8 VE9 VEA VEB VEC VED VEE VEF
    VF0 VF1 VF2 VF3 VF4 VF5 VF6 VF7 VF8 VF9 VFA VFB VFC VFD VFE VFF
}
