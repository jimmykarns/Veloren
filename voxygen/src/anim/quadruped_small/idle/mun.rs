use super::super::{super::Animation, QuadrupedSmallSkeleton, SkeletonAttr};
use mun_runtime::{invoke_fn, Runtime, RuntimeBuilder, StructRef};
use std::{cell::RefCell, rc::Rc};
use vek::*;

pub struct MunIdleAnimation;

trait BitConversion {
    type Bits;
    fn to_bits(self) -> Self::Bits;
    fn from_bits(b: Self::Bits) -> Self;
}

impl BitConversion for Quaternion<f32> {
    type Bits = u128;

    fn to_bits(self) -> Self::Bits {
        (self.x.to_bits() as u128) << 0
            | (self.y.to_bits() as u128) << 32
            | (self.z.to_bits() as u128) << 64
            | (self.w.to_bits() as u128) << 96
    }

    fn from_bits(b: Self::Bits) -> Self {
        Quaternion::from_xyzw(
            f32::from_bits(((b >> 0) & 0xffffffff) as u32),
            f32::from_bits(((b >> 32) & 0xffffffff) as u32),
            f32::from_bits(((b >> 64) & 0xffffffff) as u32),
            f32::from_bits(((b >> 96) & 0xffffffff) as u32),
        )
    }
}

extern "C" fn sin(x: f32) -> f32 { x.sin() }
extern "C" fn floor(x: f32) -> f32 { x.floor() }
extern "C" fn rot_x(x: f32) -> u128 { Quaternion::rotation_x(x).to_bits() }
extern "C" fn rot_y(x: f32) -> u128 { Quaternion::rotation_y(x).to_bits() }
extern "C" fn rot_z(x: f32) -> u128 { Quaternion::rotation_z(x).to_bits() }
extern "C" fn quat_mul(a: u128, b: u128) -> u128 {
    (Quaternion::from_bits(a) * Quaternion::from_bits(b)).to_bits()
}

pub struct MunRuntime {
    runtime: Rc<RefCell<Runtime>>,
}

impl MunRuntime {
    pub fn new() -> Self {
        let runtime = RuntimeBuilder::new("idle.munlib")
            .insert_fn("sin", sin as extern "C" fn(f32) -> f32)
            .insert_fn("floor", floor as extern "C" fn(f32) -> f32)
            .insert_fn("rot_x", rot_x as extern "C" fn(f32) -> u128)
            .insert_fn("rot_y", rot_y as extern "C" fn(f32) -> u128)
            .insert_fn("rot_z", rot_z as extern "C" fn(f32) -> u128)
            .insert_fn("quat_mul", quat_mul as extern "C" fn(u128, u128) -> u128)
            .spawn()
            .unwrap();

        Self { runtime }
    }
}

trait Marshall {
    fn to_mun(&self, runtime: &Rc<RefCell<Runtime>>) -> StructRef;
    fn from_mun(sref: StructRef) -> Self;
}

trait MarshallFrom<T> {
    fn from_mun(self) -> T;
}

impl<T: Marshall> MarshallFrom<T> for StructRef {
    fn from_mun(self) -> T { T::from_mun(self) }
}

impl Marshall for Vec3<f32> {
    fn to_mun(&self, runtime: &Rc<RefCell<Runtime>>) -> StructRef {
        invoke_fn!(runtime, "vec3", self.x, self.y, self.z).unwrap()
    }

    fn from_mun(sref: StructRef) -> Self {
        let x = sref.get("x").unwrap();
        let y = sref.get("y").unwrap();
        let z = sref.get("z").unwrap();

        Vec3::new(x, y, z)
    }
}
impl Marshall for Quaternion<f32> {
    fn to_mun(&self, runtime: &Rc<RefCell<Runtime>>) -> StructRef {
        invoke_fn!(runtime, "quaternion", self.to_bits()).unwrap()
    }

    fn from_mun(sref: StructRef) -> Self { Quaternion::from_bits(sref.get("0").unwrap()) }
}

impl Marshall for crate::anim::Bone {
    fn to_mun(&self, runtime: &Rc<RefCell<Runtime>>) -> StructRef {
        let offset = self.offset.to_mun(runtime);
        let ori = self.ori.to_mun(runtime);
        let scale = self.scale.to_mun(runtime);
        invoke_fn!(runtime, "bone", offset, ori, scale).unwrap()
    }

    fn from_mun(sref: StructRef) -> Self {
        let offset = sref.get::<StructRef>("offset").unwrap().from_mun();
        let ori = sref.get::<StructRef>("ori").unwrap().from_mun();
        let scale = sref.get::<StructRef>("scale").unwrap().from_mun();

        Self { offset, ori, scale }
    }
}

impl Marshall for QuadrupedSmallSkeleton {
    fn to_mun(&self, runtime: &Rc<RefCell<Runtime>>) -> StructRef {
        let head = self.head.to_mun(runtime);
        let chest = self.chest.to_mun(runtime);
        let leg_lf = self.leg_lf.to_mun(runtime);
        let leg_rf = self.leg_rf.to_mun(runtime);
        let leg_lb = self.leg_lb.to_mun(runtime);
        let leg_rb = self.leg_rb.to_mun(runtime);
        let tail = self.tail.to_mun(runtime);

        invoke_fn!(
            runtime, "skeleton", head, chest, leg_lf, leg_rf, leg_lb, leg_rb, tail
        )
        .unwrap()
    }

    fn from_mun(sref: StructRef) -> Self {
        let head = sref.get::<StructRef>("head").unwrap().from_mun();
        let chest = sref.get::<StructRef>("chest").unwrap().from_mun();
        let leg_lf = sref.get::<StructRef>("leg_lf").unwrap().from_mun();
        let leg_rf = sref.get::<StructRef>("leg_rf").unwrap().from_mun();
        let leg_lb = sref.get::<StructRef>("leg_lb").unwrap().from_mun();
        let leg_rb = sref.get::<StructRef>("leg_rb").unwrap().from_mun();
        let tail = sref.get::<StructRef>("tail").unwrap().from_mun();

        QuadrupedSmallSkeleton {
            head,
            chest,
            leg_lf,
            leg_rf,
            leg_lb,
            leg_rb,
            tail,
        }
    }
}
impl Marshall for SkeletonAttr {
    fn to_mun(&self, runtime: &Rc<RefCell<Runtime>>) -> StructRef {
        invoke_fn!(
            runtime,
            "attr",
            self.head.0,
            self.head.1,
            self.chest.0,
            self.chest.1,
            self.feet_f.0,
            self.feet_f.1,
            self.feet_f.2,
            self.feet_b.0,
            self.feet_b.1,
            self.feet_b.2,
            self.tail.0,
            self.tail.1
        )
        .unwrap()
    }

    // Not needed
    fn from_mun(_sref: StructRef) -> Self { unimplemented!() }
}

impl<'a> Animation for &'a MunIdleAnimation {
    type Dependency = (f64, &'a mut MunRuntime);
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        (global_time, runtime): Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let runtime = &runtime.runtime;
        // marshall data into mun
        let skeleton = skeleton.to_mun(runtime);
        let attr = skeleton_attr.to_mun(runtime);
        // call update function
        let next: StructRef = invoke_fn!(
            runtime,
            "update_skeleton",
            skeleton,
            global_time as f32,
            anim_time as f32,
            attr
        )
        .unwrap();
        // marshall result out
        next.from_mun()
    }
}
