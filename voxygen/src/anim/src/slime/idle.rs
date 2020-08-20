use super::{super::Animation, SlimeAttr, SlimeSkeleton};
//use std::{f32::consts::PI, ops::Mul};
use super::super::vek::*;
use std::{f32::consts::PI, ops::Mul};

pub struct IdleAnimation;

impl Animation for IdleAnimation {
    type Dependency = f64;
    type Skeleton = SlimeSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"slime_idle\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "slime_idle")]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        global_time: Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SlimeAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let lab = 0.55; //.65

        let short1 = (anim_time as f32 * lab as f32 * 8.0).sin();
        let short2 = (anim_time as f32 * lab as f32 * 8.0 + PI * 0.2).sin();
        let short3 = (anim_time as f32 * lab as f32 * 8.0 + PI * 0.4).sin();
        let short4 = (anim_time as f32 * lab as f32 * 8.0 + PI * 0.6).sin();
        let short5 = (anim_time as f32 * lab as f32 * 8.0 + PI * 0.8).sin();

        next.body_1.position = Vec3::new(0.0, skeleton_attr.body_1.0, skeleton_attr.body_1.1);
        next.body_1.orientation = Quaternion::rotation_z(0.0);
        next.body_1.scale = Vec3::one() / 8.0 + short1 / 12.0;

        next.body_2.position = Vec3::new(0.0, skeleton_attr.body_2.0, skeleton_attr.body_2.1 + 4.0);
        next.body_2.orientation = Quaternion::rotation_z(0.0);
        next.body_2.scale = Vec3::one() + short2 / 4.0;
        
        next.body_3.position = Vec3::new(0.0, skeleton_attr.body_3.0, skeleton_attr.body_3.1 + 4.0);
        next.body_3.orientation = Quaternion::rotation_z(0.0);
        next.body_3.scale = Vec3::one() + short3 / 3.0;

        next.body_4.position = Vec3::new(0.0, skeleton_attr.body_4.0, skeleton_attr.body_4.1 + 4.0);
        next.body_4.orientation = Quaternion::rotation_z(0.0);
        next.body_4.scale = Vec3::one() + short4 / 2.0;

        next.body_5.position = Vec3::new(0.0, skeleton_attr.body_5.0, skeleton_attr.body_5.1 + 4.0);
        next.body_5.orientation = Quaternion::rotation_z(0.0);
        next.body_5.scale = Vec3::one() + short5 / 1.5;

        next.tail_upper.position = Vec3::new(0.0, skeleton_attr.tail_upper.0 + 12.0, skeleton_attr.tail_upper.1 + 4.0);
        next.tail_upper.orientation = Quaternion::rotation_z(0.0);
        next.tail_upper.scale = Vec3::one() + short2 / 8.0;

        next.tail_lower.position = Vec3::new(0.0, skeleton_attr.tail_lower.0 + 8.0, skeleton_attr.tail_lower.1 - 4.0);
        next.tail_lower.orientation = Quaternion::rotation_z(0.0);
        next.tail_lower.scale = Vec3::one() + short3 / 8.0;

        next
    }
}
