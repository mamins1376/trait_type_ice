#![feature(extern_types)]

extern {
    type Extern;
}

trait Trait {
    type Type;
}

#[inline]
fn f<'a>(_: <&'a Extern as Trait>::Type) where &'a Extern: Trait {}
