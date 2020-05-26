use super::{super::Animation, SnakeSkeleton, SkeletonAttr};
use std::{f32::consts::PI, ops::Mul};
use vek::*;

pub struct IdleAnimation;

impl Animation for IdleAnimation {
    type Dependency = f64;
    type Skeleton = SnakeSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"snake_idle\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "snake_idle")]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        global_time: Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let wave_ultra_slow = (anim_time as f32 * 1.0 + PI).sin();
        let wave_ultra_slow_cos = (anim_time as f32 * 1.0 + PI).cos();
        let wave_slow = (anim_time as f32 * 3.5 + PI).sin();
        let wave_slow_cos = (anim_time as f32 * 3.5 + PI).cos();

        next.head.offset = Vec3::new(
            0.0,
            skeleton_attr.head.0,
            skeleton_attr.head.1,
        );
        next.head.ori =
            Quaternion::rotation_z(0.0) * Quaternion::rotation_x(0.0);
        next.head.scale = Vec3::one();

        next.jaw.offset = Vec3::new(
            0.0,
            skeleton_attr.jaw.0,
            skeleton_attr.jaw.1,
        );
        next.jaw.ori = Quaternion::rotation_x(0.0);
        next.jaw.scale = Vec3::one();

        next.body0.offset = Vec3::new(
            0.0,
            skeleton_attr.body0.0,
            skeleton_attr.body0.1,
        );
        next.body0.ori = Quaternion::rotation_z(0.0);
        next.body0.scale = Vec3::one();

        next.body1.offset = Vec3::new(
            0.0,
            skeleton_attr.body1.0,
            skeleton_attr.body1.1,
        );
        next.body1.ori = Quaternion::rotation_z(0.0);
        next.body1.scale = Vec3::one();

        next.body2.offset = Vec3::new(
            0.0,
            skeleton_attr.body2.0,
            skeleton_attr.body2.1,
        );
        next.body2.ori = Quaternion::rotation_z(0.0);
        next.body2.scale = Vec3::one();

        next.body3.offset = Vec3::new(
            0.0,
            skeleton_attr.body3.0,
            skeleton_attr.body3.1,
        );
        next.body3.ori = Quaternion::rotation_z(0.0);
        next.body3.scale = Vec3::one();

        next.body4.offset = Vec3::new(
            0.0,
            skeleton_attr.body4.0,
            skeleton_attr.body4.1,
        );
        next.body4.ori = Quaternion::rotation_z(0.0);
        next.body4.scale = Vec3::one();

        next.body5.offset = Vec3::new(
            0.0,
            skeleton_attr.body5.0,
            skeleton_attr.body5.1,
        );
        next.body5.ori = Quaternion::rotation_z(0.0);
        next.body5.scale = Vec3::one();

        next.body6.offset = Vec3::new(
            0.0,
            skeleton_attr.body6.0,
            skeleton_attr.body6.1,
        );
        next.body6.ori = Quaternion::rotation_z(0.0);
        next.body6.scale = Vec3::one();

        next.body7.offset = Vec3::new(
            0.0,
            skeleton_attr.body7.0,
            skeleton_attr.body7.1,
        );
        next.body7.ori = Quaternion::rotation_z(0.0);
        next.body7.scale = Vec3::one();

        next.body8.offset = Vec3::new(
            0.0,
            skeleton_attr.body8.0,
            skeleton_attr.body8.1,
        );
        next.body8.ori = Quaternion::rotation_z(0.0);
        next.body8.scale = Vec3::one();

        next
    }
}
