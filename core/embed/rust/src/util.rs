use crate::{
    error::Error,
    micropython::{map::Map, obj::Obj},
};

pub fn try_or_none(f: impl FnOnce() -> Result<Obj, Error>) -> Obj {
    f().unwrap_or(Obj::const_none())
}

pub fn try_with_kw(kw: *const Map, func: impl FnOnce(&Map) -> Result<Obj, Error>) -> Obj {
    unsafe { kw.as_ref() }
        .and_then(|kw| func(kw).ok())
        .unwrap_or(Obj::const_none())
}
