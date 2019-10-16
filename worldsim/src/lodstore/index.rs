use std::{u16, u32};

pub trait ToOptionUsize: Copy {
    fn none() -> Self;
    fn is_some(self) -> bool;
    fn into_usize(self) -> usize;
}

impl ToOptionUsize for u32 {
    fn none() -> Self {
        u32::MAX
    }
    fn is_some(self) -> bool {
        self != u32::MAX
    }
    fn into_usize(self) -> usize {
        self as usize
    }
}

impl ToOptionUsize for u16 {
    fn none() -> Self {
        u16::MAX
    }
    fn is_some(self) -> bool {
        self != u16::MAX
    }
    fn into_usize(self) -> usize {
        self as usize
    }
}
