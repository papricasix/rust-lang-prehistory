//! The presence of an unreachable field in the source type (e.g., a public
//! field with a private type does not affect transmutability. (This rule is
//! distinct from type privacy, which still may forbid naming such types.)

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code)]

mod assert {
    use std::mem::BikeshedIntrinsicFrom;

    pub fn is_transmutable<Src, Dst, Context>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, false, false, false, false>
        // visibility is NOT assumed ---------------------------------^^^^^
    {}
}

mod src {
    #[repr(C)] pub(self) struct Zst; // <- unreachable type

    #[repr(C)] pub(in super) struct Src {
        pub(in super) field: Zst, //~ ERROR private type
    }
}

mod dst {
    #[repr(C)] pub(in super) struct Zst;

    #[repr(C)] pub(in super) struct Dst {
        pub(in super) field: Zst,
    }
}

fn test() {
    struct Context;
    assert::is_transmutable::<src::Src, dst::Dst, Context>();
}
