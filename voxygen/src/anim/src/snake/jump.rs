use super::{super::Animation, SnakeSkeleton, SkeletonAttr};
use std::{f32::consts::PI, ops::Mul};
use vek::*;

pub struct JumpAnimation;

impl Animation for JumpAnimation {
    type Dependency = (f32, f64);
    type Skeleton = SnakeSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"snake_jump\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "snake_jump")]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        (_velocity, global_time): Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = (*skeleton).clone();

        let wave_ultra_slow = (anim_time as f32 * 1.0 + PI).sin();
        let wave_ultra_slow_cos = (anim_time as f32 * 1.0 + PI).cos();
        let wave_slow = (anim_time as f32 * 3.5 + PI).sin();
        let wave_slow_cos = (anim_time as f32 * 3.5 + PI).cos();

        let look = Vec2::new(
            ((global_time + anim_time) as f32 / 8.0)
                .floor()
                .mul(7331.0)
                .sin()
                * 0.5,
            ((global_time + anim_time) as f32 / 8.0)
                .floor()
                .mul(1337.0)
                .sin()
                * 0.25,
        );
        let tailmove = Vec2::new(
            ((global_time + anim_time) as f32 / 2.0)
                .floor()
                .mul(7331.0)
                .sin()
                * 0.25,
            ((global_time + anim_time) as f32 / 2.0)
                .floor()
                .mul(1337.0)
                .sin()
                * 0.125,
        );

        next.head.offset = Vec3::new(
            0.0,
            skeleton_attr.head.0,
            skeleton_attr.head.1 + wave_ultra_slow * 0.4,
        ) / 11.0;
        next.head.ori =
            Quaternion::rotation_z(0.8 * look.x) * Quaternion::rotation_x(0.8 * look.y);
        next.head.scale = Vec3::one() / 10.98;

        next.jaw.offset = Vec3::new(
            0.0,
            skeleton_attr.jaw.0 - wave_ultra_slow_cos * 0.12,
            skeleton_attr.jaw.1 + wave_slow * 0.2,
        );
        next.jaw.ori = Quaternion::rotation_x(wave_slow * 0.05);
        next.jaw.scale = Vec3::one() / 11.0;

        next.body0.offset = Vec3::new(
            0.0,
            skeleton_attr.body0.0,
            skeleton_attr.body0.1 + wave_ultra_slow * 0.6,
        );
        next.body0.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body0.scale = Vec3::one() / 11.0;

        next.body1.offset = Vec3::new(
            0.0,
            skeleton_attr.body1.0,
            skeleton_attr.body1.1 + wave_ultra_slow * 0.6,
        );
        next.body1.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body1.scale = Vec3::one();

        next.body2.offset = Vec3::new(
            0.0,
            skeleton_attr.body2.0,
            skeleton_attr.body2.1 + wave_ultra_slow * 0.6,
        );
        next.body2.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body2.scale = Vec3::one();

        next.body3.offset = Vec3::new(
            0.0,
            skeleton_attr.body3.0,
            skeleton_attr.body3.1 + wave_ultra_slow * 0.6,
        );
        next.body3.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body3.scale = Vec3::one();

        next.body4.offset = Vec3::new(
            0.0,
            skeleton_attr.body4.0,
            skeleton_attr.body4.1 + wave_ultra_slow * 0.6,
        );
        next.body4.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body4.scale = Vec3::one();

        next.body5.offset = Vec3::new(
            0.0,
            skeleton_attr.body5.0,
            skeleton_attr.body5.1 + wave_ultra_slow * 0.6,
        );
        next.body5.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body5.scale = Vec3::one();

        next.body6.offset = Vec3::new(
            0.0,
            skeleton_attr.body6.0,
            skeleton_attr.body6.1 + wave_ultra_slow * 0.6,
        );
        next.body6.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body6.scale = Vec3::one();

        next.body7.offset = Vec3::new(
            0.0,
            skeleton_attr.body7.0,
            skeleton_attr.body7.1 + wave_ultra_slow * 0.6,
        );
        next.body7.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body7.scale = Vec3::one();

        next.body8.offset = Vec3::new(
            0.0,
            skeleton_attr.body8.0,
            skeleton_attr.body8.1 + wave_ultra_slow * 0.6,
        );
        next.body8.ori = Quaternion::rotation_z(0.0 + wave_slow * 0.2 + tailmove.x)
            * Quaternion::rotation_x(tailmove.y);
        next.body8.scale = Vec3::one();

        next
    }
}
