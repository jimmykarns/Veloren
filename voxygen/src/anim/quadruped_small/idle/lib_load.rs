use super::super::{super::Animation, QuadrupedSmallSkeleton, SkeletonAttr};
use libloading::{Library, Symbol};

pub struct LibIdleAnimation;

type UpdateFnPointer =
    fn(&QuadrupedSmallSkeleton, f64, f64, &mut f32, &SkeletonAttr) -> QuadrupedSmallSkeleton;

pub struct IdleLib {
    update: Symbol<'static, UpdateFnPointer>,
}

impl IdleLib {
    #![allow(unsafe_code)]
    pub fn new() -> Self {
        let static_lib = Box::leak::<'static>(Box::new(
            Library::new("./target/release/libidle.so").unwrap(),
        ));

        // Safety: lib we are loading was compiled with the same rust compiler and the
        // same vek version and has the same exact function signature
        let update = unsafe { static_lib.get(b"update_skeleton\0").unwrap() };

        Self { update }
    }
}

impl<'a> Animation for &'a LibIdleAnimation {
    type Dependency = (f64, &'a IdleLib);
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        (global_time, idle_lib): Self::Dependency,
        anim_time: f64,
        rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        (idle_lib.update)(skeleton, global_time, anim_time, rate, skeleton_attr)
    }
}
