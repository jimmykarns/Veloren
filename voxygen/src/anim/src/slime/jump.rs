use super::{super::Animation, SlimeAttr, SlimeSkeleton};
//use std::f32::consts::PI;
use super::super::vek::*;

pub struct JumpAnimation;

impl Animation for JumpAnimation {
    type Dependency = (f32, f64);
    type Skeleton = SlimeSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"slime_jump\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "slime_jump")]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        _global_time: Self::Dependency,
        _anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SlimeAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        next.body_1.position = Vec3::new(0.0, skeleton_attr.body_1.0, skeleton_attr.body_1.1);
        next.body_1.orientation = Quaternion::rotation_z(0.0);
        next.body_1.scale = Vec3::one();

        next.body_2.position = Vec3::new(0.0, skeleton_attr.body_2.0, skeleton_attr.body_2.1);
        next.body_2.orientation = Quaternion::rotation_z(0.0);
        next.body_2.scale = Vec3::one();
        
        next.body_3.position = Vec3::new(0.0, skeleton_attr.body_3.0, skeleton_attr.body_3.1);
        next.body_3.orientation = Quaternion::rotation_z(0.0);
        next.body_3.scale = Vec3::one();

        next.body_4.position = Vec3::new(0.0, skeleton_attr.body_4.0, skeleton_attr.body_4.1);
        next.body_4.orientation = Quaternion::rotation_z(0.0);
        next.body_4.scale = Vec3::one();

        next.body_5.position = Vec3::new(0.0, skeleton_attr.body_5.0, skeleton_attr.body_5.1);
        next.body_5.orientation = Quaternion::rotation_z(0.0);
        next.body_5.scale = Vec3::one();

        next.tail_upper.position = Vec3::new(0.0, skeleton_attr.tail_upper.0, skeleton_attr.tail_upper.1);
        next.tail_upper.orientation = Quaternion::rotation_z(0.0);
        next.tail_upper.scale = Vec3::one();

        next.tail_lower.position = Vec3::new(0.0, skeleton_attr.tail_lower.0, skeleton_attr.tail_lower.1);
        next.tail_lower.orientation = Quaternion::rotation_z(0.0);
        next.tail_lower.scale = Vec3::one();

        next
    }
}
