use std::{f32::consts::PI, ops::Mul};
use vek::*;

#[derive(Copy, Clone, Debug)]
pub struct Bone {
    pub offset: Vec3<f32>,
    pub ori: Quaternion<f32>,
    pub scale: Vec3<f32>,
}

impl Default for Bone {
    fn default() -> Self {
        Self {
            offset: Vec3::zero(),
            ori: Quaternion::identity(),
            scale: Vec3::broadcast(1.0 / 11.0),
        }
    }
}
#[derive(Clone, Default, Debug)]
pub struct QuadrupedSmallSkeleton {
    head: Bone,
    chest: Bone,
    leg_lf: Bone,
    leg_rf: Bone,
    leg_lb: Bone,
    leg_rb: Bone,
    tail: Bone,
}

impl QuadrupedSmallSkeleton {
    pub fn new() -> Self { Self::default() }
}

#[derive(Clone, Debug)]
pub struct SkeletonAttr {
    head: (f32, f32),
    chest: (f32, f32),
    feet_f: (f32, f32, f32),
    feet_b: (f32, f32, f32),
    tail: (f32, f32),
}

type Skeleton = QuadrupedSmallSkeleton;

#[no_mangle]
pub fn update_skeleton(
    skeleton: &Skeleton,
    global_time: f64,
    anim_time: f64,
    _rate: &mut f32,
    skeleton_attr: &SkeletonAttr,
) -> Skeleton {
    let mut next = (*skeleton).clone();

    let slow = (anim_time as f32 * 3.5).sin();
    let slowa = (anim_time as f32 * 3.5 + PI / 2.0).sin();

    let slow_alt = (anim_time as f32 * 3.5 + PI).sin();

    let head_look = Vec2::new(
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

    next.head.offset =
        Vec3::new(0.0, skeleton_attr.head.0, skeleton_attr.head.1 + slow * 0.2) / 11.0;
    next.head.ori =
        Quaternion::rotation_z(head_look.x) * Quaternion::rotation_x(head_look.y + slow_alt * 0.03);
    next.head.scale = Vec3::one() / 10.5;

    next.chest.offset = Vec3::new(
        slow * 0.05,
        skeleton_attr.chest.0,
        skeleton_attr.chest.1 + slowa * 0.2,
    ) / 11.0;
    next.chest.ori = Quaternion::rotation_y(slow * 0.05);
    next.chest.scale = Vec3::one() / 11.0;

    next.leg_lf.offset = Vec3::new(
        -skeleton_attr.feet_f.0,
        skeleton_attr.feet_f.1,
        skeleton_attr.feet_f.2,
    ) / 11.0;
    next.leg_lf.ori = Quaternion::rotation_x(slow * 0.08);
    next.leg_lf.scale = Vec3::one() / 11.0;

    next.leg_rf.offset = Vec3::new(
        skeleton_attr.feet_f.0,
        skeleton_attr.feet_f.1,
        skeleton_attr.feet_f.2,
    ) / 11.0;
    next.leg_rf.ori = Quaternion::rotation_x(slow_alt * 0.08);
    next.leg_rf.scale = Vec3::one() / 11.0;

    next.leg_lb.offset = Vec3::new(
        -skeleton_attr.feet_b.0,
        skeleton_attr.feet_b.1,
        skeleton_attr.feet_b.2,
    ) / 11.0;
    next.leg_lb.ori = Quaternion::rotation_x(slow_alt * 0.08);
    next.leg_lb.scale = Vec3::one() / 11.0;

    next.leg_rb.offset = Vec3::new(
        skeleton_attr.feet_b.0,
        skeleton_attr.feet_b.1,
        skeleton_attr.feet_b.2,
    ) / 11.0;
    next.leg_rb.ori = Quaternion::rotation_x(slow * 0.08);
    next.leg_rb.scale = Vec3::one() / 11.0;

    next.tail.offset = Vec3::new(0.0, skeleton_attr.tail.0, skeleton_attr.tail.1);
    next.tail.ori = Quaternion::rotation_z(slow * 0.4);
    next.tail.scale = Vec3::one();

    next
}

// For Wasm
impl From<&[f32]> for Bone {
    fn from(slice: &[f32]) -> Self {
        Self {
            offset: Vec3::from_slice(&slice[0..3]),
            ori: Vec4::from_slice(&slice[3..7]).into(),
            scale: Vec3::from_slice(&slice[7..10]),
        }
    }
}

impl From<&[f32]> for Skeleton {
    fn from(slice: &[f32]) -> Self {
        Self {
            head: slice[0..10].into(),
            chest: slice[10..20].into(),
            leg_lf: slice[20..30].into(),
            leg_rf: slice[30..40].into(),
            leg_lb: slice[40..50].into(),
            leg_rb: slice[50..60].into(),
            tail: slice[60..70].into(),
        }
    }
}

impl From<&[f32]> for SkeletonAttr {
    fn from(slice: &[f32]) -> Self {
        Self {
            head: (slice[0], slice[1]),
            chest: (slice[2], slice[3]),
            feet_f: (slice[4], slice[5], slice[6]),
            feet_b: (slice[7], slice[8], slice[9]),
            tail: (slice[10], slice[11]),
        }
    }
}

trait WriteToSlice {
    fn write(self, slice: &mut [f32]);
}

impl WriteToSlice for Bone {
    fn write(self, slice: &mut [f32]) {
        slice[0..3].copy_from_slice(self.offset.as_slice());
        slice[3..7].copy_from_slice(self.ori.into_vec4().as_slice());
        slice[7..10].copy_from_slice(self.scale.as_slice());
    }
}

impl WriteToSlice for Skeleton {
    fn write(self, slice: &mut [f32]) {
        self.head.write(&mut slice[0..10]);
        self.chest.write(&mut slice[10..20]);
        self.leg_lf.write(&mut slice[20..30]);
        self.leg_rf.write(&mut slice[30..40]);
        self.leg_lb.write(&mut slice[40..50]);
        self.leg_rb.write(&mut slice[50..60]);
        self.tail.write(&mut slice[60..70]);
    }
}

// Hold input skeleton (70 f32) and skeleton attr (12 f32)
// Also holds output skeleton the update fn returns
static mut BUFFER: [f32; 82] = [0.0; 82];

#[no_mangle]
pub extern "C" fn buffer_pointer() -> *const f32 { unsafe { BUFFER.as_ptr() } }

// TODO: return error code if something bad happens
#[no_mangle]
pub extern "C" fn update_skeleton_c(global_time: f64, anim_time: f64, rate: f32) {
    let buffer = unsafe { &mut BUFFER };

    let skeleton = buffer[0..70].into();
    let attr = buffer[70..82].into();
    let mut rate = rate;

    let skeleton = update_skeleton(&skeleton, global_time, anim_time, &mut rate, &attr);

    skeleton.write(&mut buffer[0..70]);
}
