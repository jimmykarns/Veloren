pub mod idle;
pub mod jump;
pub mod run;

// Reexports
pub use self::{idle::IdleAnimation, jump::JumpAnimation, run::RunAnimation};

use super::{make_bone, vek::*, Bone, FigureBoneData, Skeleton};
use common::comp::{self};
use core::convert::TryFrom;

skeleton_impls!(struct SlimeSkeleton {
    + body_1,
    + body_2,
    + body_3,
    + body_4,
    + body_5,
    + tail_upper,
    + tail_lower,
});

pub struct SlimeAttr {
    body_1: (f32, f32),
    body_2: (f32, f32),
    body_3: (f32, f32),
    body_4: (f32, f32),
    body_5: (f32, f32),
    tail_upper: (f32, f32),
    tail_lower: (f32, f32),
}

impl Skeleton for SlimeSkeleton {
    type Attr = SlimeAttr;

    const BONE_COUNT: usize = 7;
    #[cfg(feature = "use-dyn-lib")]
    const COMPUTE_FN: &'static [u8] = b"slime_compute_mats\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "slime_compute_mats")]

    fn compute_matrices_inner(
        &self,
        base_mat: Mat4<f32>,
        buf: &mut [FigureBoneData; super::MAX_BONE_COUNT],
    ) -> Vec3<f32> {
        let body_1_mat = base_mat * Mat4::<f32>::from(self.body_1);
        let body_2_mat = base_mat * Mat4::<f32>::from(self.body_2);
        let body_3_mat = base_mat * Mat4::<f32>::from(self.body_3);
        let body_4_mat = base_mat * Mat4::<f32>::from(self.body_4);
        let body_5_mat = base_mat * Mat4::<f32>::from(self.body_5);
        let tail_upper_mat = base_mat * Mat4::<f32>::from(self.tail_upper);
        let tail_lower_mat = base_mat * Mat4::<f32>::from(self.tail_lower);

        *(<&mut [_; Self::BONE_COUNT]>::try_from(&mut buf[0..Self::BONE_COUNT]).unwrap()) = [
            make_bone(body_1_mat),
            make_bone(body_1_mat * body_2_mat),
            make_bone(body_1_mat * body_2_mat * body_3_mat),
            make_bone(body_1_mat * body_2_mat * body_3_mat * body_4_mat),
            make_bone(body_1_mat * body_2_mat * body_3_mat * body_4_mat * body_5_mat),
            make_bone(body_1_mat * tail_upper_mat),
            make_bone(body_1_mat * tail_upper_mat * tail_lower_mat),
        ];
        Vec3::default()
    }
}

impl<'a> std::convert::TryFrom<&'a comp::Body> for SlimeAttr {
    type Error = ();

    fn try_from(body: &'a comp::Body) -> Result<Self, Self::Error> {
        match body {
            comp::Body::Slime(body) => Ok(SlimeAttr::from(body)),
            _ => Err(()),
        }
    }
}

impl SlimeAttr {
    #[allow(clippy::match_single_binding)] // TODO: Pending review in #587
    pub fn calculate_scale(body: &comp::slime::Body) -> f32 {
        match (body.species, body.body_type) {
            (_, _) => 0.0,
        }
    }
}

impl Default for SlimeAttr {
    fn default() -> Self {
        Self {
            body_1: (0.0, 0.0),
            body_2: (0.0, 0.0),
            body_3: (0.0, 0.0),
            body_4: (0.0, 0.0),
            body_5: (0.0, 0.0),
            tail_upper: (0.0, 0.0),
            tail_lower: (0.0, 0.0),
        }
    }
}

impl<'a> From<&'a comp::slime::Body> for SlimeAttr {
    fn from(body: &'a comp::slime::Body) -> Self {
        use comp::slime::Species::*;
        Self {
            body_1: match (body.species, body.body_type) {
                (GreenSlime, _) => (0.0, 0.0),
            },
            body_2: match (body.species, body.body_type) {
                (GreenSlime, _) => (0.0, 0.0),
            },
            body_3: match (body.species, body.body_type) {
                (GreenSlime, _) => (0.0, 0.0),
            },
            body_4: match (body.species, body.body_type) {
                (GreenSlime, _) => (0.0, 0.0),
            },
            body_5: match (body.species, body.body_type) {
                (GreenSlime, _) => (0.0, 0.0),
            },
            tail_upper: match (body.species, body.body_type) {
                (GreenSlime, _) => (0.0, 0.0),
            },
            tail_lower: match (body.species, body.body_type) {
                (GreenSlime, _) => (0.0, 0.0),
            },
        }
    }
}
