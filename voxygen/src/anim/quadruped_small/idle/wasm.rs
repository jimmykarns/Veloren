use super::super::{super::Animation, QuadrupedSmallSkeleton, SkeletonAttr};
use crate::anim::Bone;
use std::{
    cell::Cell,
    fs::File,
    io::{BufReader, Read},
};
use vek::*;
use wasmer_runtime::{func, imports, instantiate, Array, Ctx, Func, Memory, WasmPtr};

pub struct WasmIdleAnimation;

pub struct WasmRuntime {
    ptr: WasmPtr<f32, Array>,
    memory: &'static Memory,
    update: Func<'static, (f64, f64, f32)>,
}

impl WasmRuntime {
    pub fn new() -> Self {
        let mut file = BufReader::new(
            File::open("./target/wasm32-unknown-unknown/release/idle.wasm").unwrap(),
        );

        let mut wasm = Vec::new();
        file.read_to_end(&mut wasm).unwrap();

        fn panic(_: &mut Ctx, a: i32, b: i32, c: i32) {
            println!("Wasm panic bounds check {} {} {}", a, b, c);
            panic!("Wasm panic bounds check {} {} {}", a, b, c);
        }

        // Why do we need this?????
        let imports = imports! {
            "env" => {
                "_ZN4core9panicking18panic_bounds_check17hb3a2a75a3abbaabeE" => func!(panic),
            }
        };

        let instance = instantiate(&wasm, &imports).unwrap();
        let instance = Box::leak::<'static>(Box::new(instance));

        let pointer_fn = instance
            .exports
            .get::<Func<(), WasmPtr<f32, Array>>>("buffer_pointer")
            .unwrap();
        let ptr = pointer_fn.call().unwrap();

        let update = instance.exports.get("update_skeleton_c").unwrap();

        let memory = instance.context().memory(0);

        Self {
            ptr,
            memory,
            update,
        }
    }
}

impl<'a> Animation for &'a WasmIdleAnimation {
    type Dependency = (f64, &'a WasmRuntime);
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        (global_time, runtime): Self::Dependency,
        anim_time: f64,
        rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        // Get buffer to fill with input
        let buffer = runtime.ptr.deref(&runtime.memory, 0, 82).unwrap();
        skeleton.write(&buffer[0..70]);
        skeleton_attr.write(&buffer[70..82]);
        runtime.update.call(global_time, anim_time, *rate).unwrap();

        // Read result
        (&buffer[0..70]).into()
    }
}

/*trait WriteToSlice {
    fn write(self, slice: &mut [f32]);
}

impl WriteToSlice for Bone {
    fn write(self, slice: &mut [f32]) {
        slice[0..3].copy_from_slice(self.offset.as_slice());
        slice[3..7].copy_from_slice(self.ori.into_vec4().as_slice());
        slice[7..10].copy_from_slice(self.scale.as_slice());
    }
}

impl WriteToSlice for QuadrupedSmallSkeleton {
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

impl WriteToSlice for SkeletonAttr {
    fn write(self, slice: &mut [f32]) {
        slice[0] = self.head.0;
        slice[1] = self.head.1;
        slice[2] = self.chest.0;
        slice[3] = self.chest.1;
        slice[4] = self.feet_f.0;
        slice[5] = self.feet_f.1;
        slice[6] = self.feet_f.2;
        slice[7] = self.feet_b.0;
        slice[8] = self.feet_b.1;
        slice[9] = self.feet_b.2;
        slice[10] = self.tail.0;
        slice[11] = self.tail.1;
    }
}*/
trait WriteToCellSlice {
    fn write(self, slice: &[Cell<f32>]);
}

impl WriteToCellSlice for &Bone {
    fn write(self, slice: &[Cell<f32>]) {
        self.offset
            .iter()
            .enumerate()
            .for_each(|(i, v)| slice[i + 0].set(*v));
        self.ori
            .into_vec4()
            .iter()
            .enumerate()
            .for_each(|(i, v)| slice[i + 3].set(*v));
        self.scale
            .iter()
            .enumerate()
            .for_each(|(i, v)| slice[i + 7].set(*v));
    }
}

impl WriteToCellSlice for &QuadrupedSmallSkeleton {
    fn write(self, slice: &[Cell<f32>]) {
        self.head.write(&slice[0..10]);
        self.chest.write(&slice[10..20]);
        self.leg_lf.write(&slice[20..30]);
        self.leg_rf.write(&slice[30..40]);
        self.leg_lb.write(&slice[40..50]);
        self.leg_rb.write(&slice[50..60]);
        self.tail.write(&slice[60..70]);
    }
}

impl WriteToCellSlice for &SkeletonAttr {
    fn write(self, slice: &[Cell<f32>]) {
        slice[0].set(self.head.0);
        slice[1].set(self.head.1);
        slice[2].set(self.chest.0);
        slice[3].set(self.chest.1);
        slice[4].set(self.feet_f.0);
        slice[5].set(self.feet_f.1);
        slice[6].set(self.feet_f.2);
        slice[7].set(self.feet_b.0);
        slice[8].set(self.feet_b.1);
        slice[9].set(self.feet_b.2);
        slice[10].set(self.tail.0);
        slice[11].set(self.tail.1);
    }
}

impl From<&[Cell<f32>]> for Bone {
    fn from(s: &[Cell<f32>]) -> Self {
        Self {
            offset: Vec3::new(s[0].get(), s[1].get(), s[2].get()),
            ori: Vec4::new(s[3].get(), s[4].get(), s[5].get(), s[6].get()).into(),
            scale: Vec3::new(s[7].get(), s[8].get(), s[9].get()),
        }
    }
}

impl From<&[Cell<f32>]> for QuadrupedSmallSkeleton {
    fn from(slice: &[Cell<f32>]) -> Self {
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
