pub mod idle;
pub mod jump;
pub mod run;

// Reexports
pub use self::{idle::IdleAnimation, jump::JumpAnimation, run::RunAnimation};

use super::{Bone, FigureBoneData, Skeleton};
use common::comp::{self};
use vek::Vec3;

#[derive(Clone, Default)]
pub struct SnakeSkeleton {
    head: Bone,
    jaw: Bone,
    body0: Bone,
    body1: Bone,
    body2: Bone,
    body3: Bone,
    body4: Bone,
    body5: Bone,
    body6: Bone,
    body7: Bone,
    body8: Bone,
}

impl SnakeSkeleton {
    pub fn new() -> Self { Self::default() }
}

impl Skeleton for SnakeSkeleton {
    type Attr = SkeletonAttr;

    #[cfg(feature = "use-dyn-lib")]
    const COMPUTE_FN: &'static [u8] = b"snake_compute_mats\0";

    fn bone_count(&self) -> usize { 11 }

    #[cfg_attr(feature = "be-dyn-lib", export_name = "snake_compute_mats")]
    fn compute_matrices_inner(&self) -> ([FigureBoneData; 16], Vec3<f32>) {
        //let ears_mat = self.ears.compute_base_matrix();
        let head_mat = self.head.compute_base_matrix();
        //let head_lower_mat = self.head_lower.compute_base_matrix();
        //let torso_mid_mat = self.torso_mid.compute_base_matrix();
        (
            [
                FigureBoneData::new(head_mat),
                FigureBoneData::new(head_mat * self.jaw.compute_base_matrix()),
                FigureBoneData::new(self.body0.compute_base_matrix()),
                FigureBoneData::new(self.body0.compute_base_matrix() * self.body1.compute_base_matrix()),
                FigureBoneData::new(self.body1.compute_base_matrix() * self.body2.compute_base_matrix()),
                FigureBoneData::new(self.body2.compute_base_matrix() * self.body3.compute_base_matrix()),
                FigureBoneData::new(self.body3.compute_base_matrix() * self.body4.compute_base_matrix()),
                FigureBoneData::new(self.body4.compute_base_matrix() * self.body5.compute_base_matrix()),
                FigureBoneData::new(self.body5.compute_base_matrix() * self.body6.compute_base_matrix()),
                FigureBoneData::new(self.body6.compute_base_matrix() * self.body7.compute_base_matrix()),
                FigureBoneData::new(self.body7.compute_base_matrix() * self.body8.compute_base_matrix()),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
                FigureBoneData::default(),
            ],
            Vec3::default(),
        )
    }

    fn interpolate(&mut self, target: &Self, dt: f32) {
        self.head.interpolate(&target.head, dt);
        self.jaw.interpolate(&target.jaw, dt);
        self.body0.interpolate(&target.body0, dt);
        self.body1.interpolate(&target.body1, dt);
        self.body2.interpolate(&target.body2, dt);
        self.body3.interpolate(&target.body3, dt);
        self.body4.interpolate(&target.body4, dt);
        self.body5.interpolate(&target.body5, dt);
        self.body6.interpolate(&target.body6, dt);
        self.body7.interpolate(&target.body7, dt);
        self.body8.interpolate(&target.body8, dt);
    }
}

pub struct SkeletonAttr {
    head: (f32, f32),
    jaw: (f32, f32),
    body0: (f32, f32),
    body1: (f32, f32),
    body2: (f32, f32),
    body3: (f32, f32),
    body4: (f32, f32),
    body5: (f32, f32),
    body6: (f32, f32),
    body7: (f32, f32),
    body8: (f32, f32),
    height: f32,
}

impl<'a> std::convert::TryFrom<&'a comp::Body> for SkeletonAttr {
    type Error = ();

    fn try_from(body: &'a comp::Body) -> Result<Self, Self::Error> {
        match body {
            comp::Body::Snake(body) => Ok(SkeletonAttr::from(body)),
            _ => Err(()),
        }
    }
}

impl Default for SkeletonAttr {
    fn default() -> Self {
        Self {
            head: (0.0, 0.0),
            jaw: (0.0, 0.0),
            body0: (0.0, 0.0),
            body1: (0.0, 0.0),
            body2: (0.0, 0.0),
            body3: (0.0, 0.0),
            body4: (0.0, 0.0),
            body5: (0.0, 0.0),
            body6: (0.0, 0.0),
            body7: (0.0, 0.0),
            body8: (0.0, 0.0),
            height: (0.0),
        }
    }
}

impl<'a> From<&'a comp::snake::Body> for SkeletonAttr {
    fn from(body: &'a comp::snake::Body) -> Self {
        use comp::snake::Species::*;
        Self {
            head: match (body.species, body.body_type) {
                (Cobra, _) => (12.0, 16.0),
            },
            jaw: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body0: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body1: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body2: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body3: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body4: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body5: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body6: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body7: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            body8: match (body.species, body.body_type) {
                (Cobra, _) => (3.0, -5.0),
            },
            height: match (body.species, body.body_type) {
                (Cobra, _) => (1.2),
            },
        }
    }
}
