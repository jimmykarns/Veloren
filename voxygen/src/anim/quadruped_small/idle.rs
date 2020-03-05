use super::{super::Animation, QuadrupedSmallSkeleton, SkeletonAttr};
use std::{f32::consts::PI, ops::Mul};
use vek::*;

pub struct IdleAnimation;

impl Animation for IdleAnimation {
    type Dependency = f64;
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        global_time: Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
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
        next.head.ori = Quaternion::rotation_z(head_look.x)
            * Quaternion::rotation_x(head_look.y + slow_alt * 0.03);
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
}

pub struct RhaiIdleAnimation;

#[derive(Clone)]
pub struct RhaiQuadrupedSmallSkeleton(QuadrupedSmallSkeleton);
impl RhaiQuadrupedSmallSkeleton {
    fn get_head_offset(&mut self) -> Vec3<f32> { self.0.head.offset }

    fn get_head_ori(&mut self) -> Quaternion<f32> { self.0.head.ori }

    fn get_head_scale(&mut self) -> Vec3<f32> { self.0.head.scale }

    fn set_head_offset(&mut self, offset: Vec3<f32>) { self.0.head.offset = offset; }

    fn set_head_ori(&mut self, ori: Quaternion<f32>) { self.0.head.ori = ori; }

    fn set_head_scale(&mut self, scale: Vec3<f32>) { self.0.head.scale = scale; }

    fn get_chest_offset(&mut self) -> Vec3<f32> { self.0.chest.offset }

    fn get_chest_ori(&mut self) -> Quaternion<f32> { self.0.chest.ori }

    fn get_chest_scale(&mut self) -> Vec3<f32> { self.0.chest.scale }

    fn set_chest_offset(&mut self, offset: Vec3<f32>) { self.0.chest.offset = offset; }

    fn set_chest_ori(&mut self, ori: Quaternion<f32>) { self.0.chest.ori = ori; }

    fn set_chest_scale(&mut self, scale: Vec3<f32>) { self.0.chest.scale = scale; }

    fn get_leg_lf_offset(&mut self) -> Vec3<f32> { self.0.leg_lf.offset }

    fn get_leg_lf_ori(&mut self) -> Quaternion<f32> { self.0.leg_lf.ori }

    fn get_leg_lf_scale(&mut self) -> Vec3<f32> { self.0.leg_lf.scale }

    fn set_leg_lf_offset(&mut self, offset: Vec3<f32>) { self.0.leg_lf.offset = offset; }

    fn set_leg_lf_ori(&mut self, ori: Quaternion<f32>) { self.0.leg_lf.ori = ori; }

    fn set_leg_lf_scale(&mut self, scale: Vec3<f32>) { self.0.leg_lf.scale = scale; }

    fn get_leg_rf_offset(&mut self) -> Vec3<f32> { self.0.leg_rf.offset }

    fn get_leg_rf_ori(&mut self) -> Quaternion<f32> { self.0.leg_rf.ori }

    fn get_leg_rf_scale(&mut self) -> Vec3<f32> { self.0.leg_rf.scale }

    fn set_leg_rf_offset(&mut self, offset: Vec3<f32>) { self.0.leg_rf.offset = offset; }

    fn set_leg_rf_ori(&mut self, ori: Quaternion<f32>) { self.0.leg_rf.ori = ori; }

    fn set_leg_rf_scale(&mut self, scale: Vec3<f32>) { self.0.leg_rf.scale = scale; }

    fn get_leg_lb_offset(&mut self) -> Vec3<f32> { self.0.leg_lb.offset }

    fn get_leg_lb_ori(&mut self) -> Quaternion<f32> { self.0.leg_lb.ori }

    fn get_leg_lb_scale(&mut self) -> Vec3<f32> { self.0.leg_lb.scale }

    fn set_leg_lb_offset(&mut self, offset: Vec3<f32>) { self.0.leg_lb.offset = offset; }

    fn set_leg_lb_ori(&mut self, ori: Quaternion<f32>) { self.0.leg_lb.ori = ori; }

    fn set_leg_lb_scale(&mut self, scale: Vec3<f32>) { self.0.leg_lb.scale = scale; }

    fn get_leg_rb_offset(&mut self) -> Vec3<f32> { self.0.leg_rb.offset }

    fn get_leg_rb_ori(&mut self) -> Quaternion<f32> { self.0.leg_rb.ori }

    fn get_leg_rb_scale(&mut self) -> Vec3<f32> { self.0.leg_rb.scale }

    fn set_leg_rb_offset(&mut self, offset: Vec3<f32>) { self.0.leg_rb.offset = offset; }

    fn set_leg_rb_ori(&mut self, ori: Quaternion<f32>) { self.0.leg_rb.ori = ori; }

    fn set_leg_rb_scale(&mut self, scale: Vec3<f32>) { self.0.leg_rb.scale = scale; }
}
#[derive(Clone)]
struct RhaiSkeletonAttr(SkeletonAttr);
impl RhaiSkeletonAttr {
    fn head_0(&mut self) -> f32 { self.0.head.0 }

    fn head_1(&mut self) -> f32 { self.0.head.1 }

    fn chest_0(&mut self) -> f32 { self.0.chest.0 }

    fn chest_1(&mut self) -> f32 { self.0.chest.1 }

    fn feet_f_0(&mut self) -> f32 { self.0.feet_f.0 }

    fn feet_f_1(&mut self) -> f32 { self.0.feet_f.1 }

    fn feet_f_2(&mut self) -> f32 { self.0.feet_f.2 }

    fn feet_b_0(&mut self) -> f32 { self.0.feet_b.0 }

    fn feet_b_1(&mut self) -> f32 { self.0.feet_b.1 }

    fn feet_b_2(&mut self) -> f32 { self.0.feet_b.2 }
}

pub struct AnimationRhaiEngine {
    engine: rhai::Engine,
    ast: rhai::AST,
}

trait Vec2RhaiExt {
    fn x(&mut self) -> f32;
    fn y(&mut self) -> f32;
}
impl Vec2RhaiExt for Vec2<f32> {
    fn x(&mut self) -> f32 { self.x }

    fn y(&mut self) -> f32 { self.y }
}

const IDLE_SCRIPT: &str = "
    let wave = sin(anim_time * 14.0);
    let wave_slow = sin(anim_time * 3.5 + PI);
    let wave_slow_cos = cos(anim_time * 3.5 + PI);

    let pig_head_look = vec2(
        sin(floor((global_time + anim_time) / 8.0) * 7331.0) * 0.5,
        sin(floor((global_time + anim_time) / 8.0) * 1337.0) * 0.25
    );

    next.head_offset = vec3(0.0, skeleton_attr.head_0, skeleton_attr.head_1 + wave * 0.2) / \
                           f32(11.0);
    next.head_ori = rot_z(pig_head_look.x) * rot_x(pig_head_look.y + wave_slow_cos * 0.03);
    next.head_scale = vec3(1.0 / 10.5);

    next.chest_offset = vec3(
        wave_slow * f32(0.05),
        skeleton_attr.chest_0,
        skeleton_attr.chest_1 + wave_slow_cos * 0.2
    ) / 11.0;
    next.chest_ori = rot_y(wave_slow * f32(0.05));
    next.chest_scale = vec3(1.0 / 11.0);

    next.leg_lf_offset = vec3(
        -(skeleton_attr.feet_f_0),
        skeleton_attr.feet_f_1,
        skeleton_attr.feet_f_2
    ) / 11.0;
    next.leg_lf_ori = rot_x(wave_slow_cos * 0.08);
    next.leg_lf_scale = vec3(1.0 / 11.0);

    next.leg_rf_offset = vec3(
        skeleton_attr.feet_f_0,
        skeleton_attr.feet_f_1,
        skeleton_attr.feet_f_2
    ) / 11.0;
    next.leg_rf_ori = rot_x(wave_slow_cos * 0.08);
    next.leg_rf_scale = vec3(1.0 / 11.0);

    next.leg_lb_offset = vec3(
        -(skeleton_attr.feet_b_0),
        skeleton_attr.feet_b_1,
        skeleton_attr.feet_b_2
    ) / 11.0;
    next.leg_lb_ori = rot_x(wave_slow_cos * 0.08);
    next.leg_lb_scale = vec3(1.0 / 11.0);

    next.leg_rb_offset = vec3(
        skeleton_attr.feet_b_0,
        skeleton_attr.feet_b_1,
        skeleton_attr.feet_b_2
    ) / 11.0;
    next.leg_rb_ori = rot_x(wave_slow_cos * 0.08);
    next.leg_rb_scale = vec3(1.0 / 11.0);

    next
";

impl AnimationRhaiEngine {
    pub fn new() -> Self {
        let mut engine = rhai::Engine::new();
        // Add functions
        use rhai::RegisterFn;
        use std::ops::Div;
        engine.register_fn("floor", f32::floor);
        engine.register_fn("sin", f32::sin);
        engine.register_fn("cos", f32::cos);
        engine.register_type::<Vec2<f32>>();
        engine.register_type::<Vec3<f32>>();
        engine.register_type::<Quaternion<f32>>();
        engine.register_fn("vec2", Vec2::<f32>::new);
        engine.register_fn("vec3", Vec3::<f32>::new);
        engine.register_fn("/", <Vec3<f32> as Div<f32>>::div);
        engine.register_fn("vec3", Vec3::<f32>::broadcast);
        engine.register_fn("rot_x", Quaternion::<f32>::rotation_x);
        engine.register_fn("rot_y", Quaternion::<f32>::rotation_y);
        engine.register_fn("rot_z", Quaternion::<f32>::rotation_z);
        engine.register_fn("*", <Quaternion<f32> as Mul<Quaternion<f32>>>::mul);
        fn mul_f64(x: f32, y: f64) -> f32 { x * y as f32 }
        engine.register_fn("*", mul_f64);
        fn div_f64(x: f32, y: f64) -> f32 { x / y as f32 }
        engine.register_fn("/", div_f64);
        fn v3_div_f64(x: Vec3<f32>, y: f64) -> Vec3<f32> { x / y as f32 }
        engine.register_fn("/", v3_div_f64);
        fn vec3_broadcast_f64(x: f64) -> Vec3<f32> { Vec3::broadcast(x as f32) }
        engine.register_fn("vec3", vec3_broadcast_f64);
        fn vec3_first_f64(x: f64, y: f32, z: f32) -> Vec3<f32> { Vec3::new(x as f32, y, z) }
        engine.register_fn("vec3", vec3_first_f64);

        engine.register_get("x", Vec2::<f32>::x);
        engine.register_get("y", Vec2::<f32>::y);

        engine.register_type::<RhaiQuadrupedSmallSkeleton>();
        engine.register_get_set(
            "head_offset",
            RhaiQuadrupedSmallSkeleton::get_head_offset,
            RhaiQuadrupedSmallSkeleton::set_head_offset,
        );
        engine.register_get_set(
            "head_ori",
            RhaiQuadrupedSmallSkeleton::get_head_ori,
            RhaiQuadrupedSmallSkeleton::set_head_ori,
        );
        engine.register_get_set(
            "head_scale",
            RhaiQuadrupedSmallSkeleton::get_head_scale,
            RhaiQuadrupedSmallSkeleton::set_head_scale,
        );
        engine.register_get_set(
            "chest_offset",
            RhaiQuadrupedSmallSkeleton::get_chest_offset,
            RhaiQuadrupedSmallSkeleton::set_chest_offset,
        );
        engine.register_get_set(
            "chest_ori",
            RhaiQuadrupedSmallSkeleton::get_chest_ori,
            RhaiQuadrupedSmallSkeleton::set_chest_ori,
        );
        engine.register_get_set(
            "chest_scale",
            RhaiQuadrupedSmallSkeleton::get_chest_scale,
            RhaiQuadrupedSmallSkeleton::set_chest_scale,
        );
        engine.register_get_set(
            "leg_lf_offset",
            RhaiQuadrupedSmallSkeleton::get_leg_lf_offset,
            RhaiQuadrupedSmallSkeleton::set_leg_lf_offset,
        );
        engine.register_get_set(
            "leg_lf_ori",
            RhaiQuadrupedSmallSkeleton::get_leg_lf_ori,
            RhaiQuadrupedSmallSkeleton::set_leg_lf_ori,
        );
        engine.register_get_set(
            "leg_lf_scale",
            RhaiQuadrupedSmallSkeleton::get_leg_lf_scale,
            RhaiQuadrupedSmallSkeleton::set_leg_lf_scale,
        );
        engine.register_get_set(
            "leg_rf_offset",
            RhaiQuadrupedSmallSkeleton::get_leg_rf_offset,
            RhaiQuadrupedSmallSkeleton::set_leg_rf_offset,
        );
        engine.register_get_set(
            "leg_rf_ori",
            RhaiQuadrupedSmallSkeleton::get_leg_rf_ori,
            RhaiQuadrupedSmallSkeleton::set_leg_rf_ori,
        );
        engine.register_get_set(
            "leg_rf_scale",
            RhaiQuadrupedSmallSkeleton::get_leg_rf_scale,
            RhaiQuadrupedSmallSkeleton::set_leg_rf_scale,
        );
        engine.register_get_set(
            "leg_lb_offset",
            RhaiQuadrupedSmallSkeleton::get_leg_lb_offset,
            RhaiQuadrupedSmallSkeleton::set_leg_lb_offset,
        );
        engine.register_get_set(
            "leg_lb_ori",
            RhaiQuadrupedSmallSkeleton::get_leg_lb_ori,
            RhaiQuadrupedSmallSkeleton::set_leg_lb_ori,
        );
        engine.register_get_set(
            "leg_lb_scale",
            RhaiQuadrupedSmallSkeleton::get_leg_lb_scale,
            RhaiQuadrupedSmallSkeleton::set_leg_lb_scale,
        );
        engine.register_get_set(
            "leg_rb_offset",
            RhaiQuadrupedSmallSkeleton::get_leg_rb_offset,
            RhaiQuadrupedSmallSkeleton::set_leg_rb_offset,
        );
        engine.register_get_set(
            "leg_rb_ori",
            RhaiQuadrupedSmallSkeleton::get_leg_rb_ori,
            RhaiQuadrupedSmallSkeleton::set_leg_rb_ori,
        );
        engine.register_get_set(
            "leg_rb_scale",
            RhaiQuadrupedSmallSkeleton::get_leg_rb_scale,
            RhaiQuadrupedSmallSkeleton::set_leg_rb_scale,
        );

        engine.register_type::<RhaiSkeletonAttr>();
        engine.register_get("head_0", RhaiSkeletonAttr::head_0);
        engine.register_get("head_1", RhaiSkeletonAttr::head_1);
        engine.register_get("chest_0", RhaiSkeletonAttr::chest_0);
        engine.register_get("chest_1", RhaiSkeletonAttr::chest_1);
        engine.register_get("feet_f_0", RhaiSkeletonAttr::feet_f_0);
        engine.register_get("feet_f_1", RhaiSkeletonAttr::feet_f_1);
        engine.register_get("feet_f_2", RhaiSkeletonAttr::feet_f_2);
        engine.register_get("feet_b_0", RhaiSkeletonAttr::feet_b_0);
        engine.register_get("feet_b_1", RhaiSkeletonAttr::feet_b_1);
        engine.register_get("feet_b_2", RhaiSkeletonAttr::feet_b_2);

        fn f32(x: f64) -> f32 { x as f32 }
        engine.register_fn("f32", f32);

        let ast = engine.compile(IDLE_SCRIPT).unwrap();

        Self { engine, ast }
    }
}

impl<'a> Animation for &'a RhaiIdleAnimation {
    type Dependency = (f64, &'a mut AnimationRhaiEngine);
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        (global_time, rhai_engine): Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        use rhai::Dynamic;
        // TODO: could reuse scope using set_value
        let mut scope = rhai::Scope::new();
        scope.push_dynamic(
            "next",
            Dynamic::from(RhaiQuadrupedSmallSkeleton(skeleton.clone())),
        );
        scope.push_dynamic("anim_time", Dynamic::from(anim_time as f32));
        scope.push_dynamic("global_time", Dynamic::from(global_time as f32));
        scope.push_dynamic(
            "skeleton_attr",
            Dynamic::from(RhaiSkeletonAttr(skeleton_attr.clone())),
        );
        scope.push_dynamic("PI", Dynamic::from(PI));

        rhai_engine
            .engine
            .eval_ast_with_scope::<RhaiQuadrupedSmallSkeleton>(&mut scope, &rhai_engine.ast)
            //.eval_with_scope::<RhaiQuadrupedSmallSkeleton>(&mut scope, IDLE_SCRIPT)
            .unwrap()
            .0
     }
}

const LEON_IDLE_SCRIPT: &str = "
            var wave = sin(anim_time * 14.0);
            var wave_slow = sin(anim_time * 3.5 + PI); 
            var wave_slow_cos = cos(anim_time * 3.5 + PI);

            var pig_head_look = vec2(
                sin(floor((global_time + anim_time) / 8.0) * 7331.0) * 0.5,
                sin(floor((global_time + anim_time) / 8.0) * 1337.0) * 0.25
            );

            next.head_offset = vec3(0.0, skeleton_attr.head_0, skeleton_attr.head_1 + wave * 0.2) \
                           / 11.0;
            next.head_ori = rot_z(pig_head_look.x) * rot_x(pig_head_look.y + wave_slow_cos * 0.03);
            next.head_scale = vec3(1.0 / 10.5);
            
            next.chest_offset = vec3(
                wave_slow * 0.05,
                skeleton_attr.chest_0,
                skeleton_attr.chest_1 + wave_slow_cos * 0.2
            ) / 11.0;
            next.chest_ori = rot_y(wave_slow * 0.05);
            next.chest_scale = vec3(1.0 / 11.0);

            next.leg_lf_offset = vec3(
                -(skeleton_attr.feet_f_0),
                skeleton_attr.feet_f_1,
                skeleton_attr.feet_f_2
            ) / 11.0;
            next.leg_lf_ori = rot_x(wave_slow_cos * 0.08);
            next.leg_lf_scale = vec3(1.0 / 11.0);
            
            next.leg_rf_offset = vec3(
                skeleton_attr.feet_f_0,
                skeleton_attr.feet_f_1,
                skeleton_attr.feet_f_2
            ) / 11.0;
            next.leg_rf_ori = rot_x(wave_slow_cos * 0.08);
            next.leg_rf_scale = vec3(1.0 / 11.0);

            next.leg_lb_offset = vec3(
                -(skeleton_attr.feet_b_0),
                skeleton_attr.feet_b_1,
                skeleton_attr.feet_b_2
            ) / 11.0;
            next.leg_lb_ori = rot_x(wave_slow_cos * 0.08);
            next.leg_lb_scale = vec3(1.0 / 11.0);

            next.leg_rb_offset = vec3(
                skeleton_attr.feet_b_0,
                skeleton_attr.feet_b_1,
                skeleton_attr.feet_b_2
            ) / 11.0;
            next.leg_rb_ori = rot_x(wave_slow_cos * 0.08);
            next.leg_rb_scale = vec3(1.0 / 11.0);

            next
        ";

pub struct LeonIdleAnimation;

use leon::{Value, object::InvalidOperation};
/*pub struct QuadrupedSmallSkeleton {
    head: Bone,
    chest: Bone,
    leg_lf: Bone,
    leg_rf: Bone,
    leg_lb: Bone,
    leg_rb: Bone,
}*/
impl leon::Object for QuadrupedSmallSkeleton {
   // Assign to fields 
    fn field_mutate<'a, 'b>(
        &self,
        field: &str,
        f: Box<dyn FnOnce(&mut Value<'a>) -> Result<(), ExecError> +'b>
    ) -> Result<(), ExecError> { 
        let mutate = |mut value| {
            f(&value);
            value
        };

        match field {
            "head_offset" => mutate(Value::Custom(Box::new(Vec3Wrap(self.head_offset)))).extract::<Vec3Wrap>().map(|res| self.head_offset = res.0).ok_or_else(|| Err(InvalidOperation("Mutated to invalid data type".into()))),
            "head_ori" => mutate(Value::Custom(Box::new(QuatWrap(self.head_ori)))).extract::<Vec3Wrap>().map(|res| self.head_ori = res.0).ok_or_else(|| Err(InvalidOperation("Mutated to invalid data type".into()))),
            field => Err(InvalidOperation(fmt!("This field doesn't exist: {}", field)))
        }
        Err(InvalidOperation("Not field mutable!".into()).into())
    }
}

impl leon::Object for SkeletonAttr {
    // Access fields
    fn field<'a>(&self, field: &str) -> Result<Value<'a>, InvalidOperation> { 
        match field {
            "head_0" => Ok(Value::Number(self.head.0 as f64)),
            "head_1" => Ok(Value::Number(self.head.1 as f64)),
            "chest_0" => Ok(Value::Number(self.chest.0 as f64)),
            "chest_1" => Ok(Value::Number(self.chest.1 as f64)),
            "feet_f_0" => Ok(Value::Number(self.feet_f.0 as f64)),
            "feet_f_1" => Ok(Value::Number(self.feet_f.1 as f64)),
            "feet_f_2" => Ok(Value::Number(self.feet_f.2 as f64)),
            "feet_b_0" => Ok(Value::Number(self.feet_b.0 as f64)),
            "feet_b_1" => Ok(Value::Number(self.feet_b.1 as f64)),
            "feet_b_2" => Ok(Value::Number(self.feet_b.2 as f64)),
            field => Err(InvalidOperation(fmt!("This field doesn't exist: {}", field)))
        }
    }
}

#[derive(Clone, Debug)]
struct Vec2Wrap(Vec2<f32>);
#[derive(Clone, Debug)]
struct Vec3Wrap(Vec3<f32>);
#[derive(Clone, Debug)]
struct QuatWrap(Quaterion<f32>);

impl leon::Object for Vec2Wrap {
    // Get x and y fields
    fn field<'a>(&self, field: &str) -> Result<Value<'a>, InvalidOperation> { 
        match field {
            "x" => Ok(Value::Number(self.0.x as f64)),
            "y" => Ok(Value::Number(self.0.y as f64)),
            field => Err(InvalidOperation(fmt!("This field doesn't exist: {}", field))),
        }
    }
}

impl leon::Object for Vec3Wrap {
    // Divide by scalar 
    fn div<'a>(&self, rhs: &Value<'a>) -> Result<Value<'a>, InvalidOperation> { 
        match rhs {
            Value::Number(d) => Ok(Value::Custom(Box::new(Vec3Wrap(self.0 / *d as f32)))),
            val => Err(InvalidOperation(fmt!"Not dividable by: {:?}", val)),
        }
    }
}

impl leon::Object for QuatWrap {
    // Multiply by Self
    fn mul<'a>(&self, rhs: &Value<'a>) -> Result<Value<'a>, InvalidOperation> {
        match rhs {
            Value::Custom(o) => o.as_any().downcast_ref::<Self>().cloned()
                .map(|v| Value::Custom(Box::new(*self * v)))
                .ok_or_else(|| InvalidOperation("Cannot add with provided type".into())),
            _ => Err(InvalidOperation(fmt!("Cannot add with {:?}", rhs))),
        }
    }

}


#[derive(Clone, Debug)]
struct Sin;
#[derive(Clone, Debug)]
struct Cos;
#[derive(Clone, Debug)]
struct Floor;
#[derive(Clone, Debug)]
struct RotX;
#[derive(Clone, Debug)]
struct RotY;
#[derive(Clone, Debug)]
struct RotZ;
#[derive(Clone, Debug)]
struct NewVec2;
#[derive(Clone, Debug)]
struct NewVec3;

impl leon::Object for Sin {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Number(n.sin())),
            a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}
impl leon::Object for Cos {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Number(n.cos())),
            a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}
impl leon::Object for Floor {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Number(n.floor())),
            a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}
impl leon::Object for RotX {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Custom(Box::new(QuatWrap(Quaternion::rotation_x(*n as f32))))),
            a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}
impl leon::Object for RotY {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Custom(Box::new(QuatWrap(Quaternion::rotation_y(*n as f32))))),
            a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}
impl leon::Object for RotZ {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Custom(Box::new(QuatWrap(Quaternion::rotation_z(*n as f32))))),
            a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}
impl leon::Object for NewVec2 {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
                [Value::Number(x), Value::Number(y)] => Ok(Value::Custom(Box::new(Vec2Wrap(Vec2::new(*x as f32, *y as f32))))),
                a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}
impl leon::Object for NewVec3 {
    fn call(&self, args: &[Value]) -> Result<Value, InvalidOperation> {
        match args {
            [Value::Number(b)] => Ok(Value::Custom(Box::new(Vec3Wrap(Vec3::broadcast(*n as f32))))),
            [Value::Number(x), Value::Number(y), Value::Number(z)] => Ok(Value::Custom(Box::new(Vec3Wrap(Vec3::new(*x as f32, *y as f32, *z as f32))))),
            a => Err(InvalidOperation(fmt!("Invalid args: {:?}", a))),
        }
    } 
}

impl Animation for LeonIdleAnimation {
    type Dependency = f64;
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        global_time: Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        use leon::{Engine, Value};
        let globals = vec![
            (
                "next".into(),
                Value::Custom(Box::new(skeleton.clone())),
            ),
            ("anim_time".into(), Value::Number(anim_time)),
            ("global_time".into(), Value::Number(global_time)),
            (
                "skeleton_attr".into(),
                Value::Custom(Box::new(skeleton_attr.clone())),
            ),
            ("PI".into(), Value::Number(PI as f64)),
        ];

        Engine
            .exec(
                LEON_IDLE_SCRIPT,
                globals,
                |v| v.extract::<QuadrupedSmallSkeleton>().unwrap(),
            )
            .unwrap()
      }
}
/*
pub struct KeyframeIdleAnimation;
impl Animation for KeyframeIdleAnimation {
    type Dependency = f64;
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        global_time: Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let mut next = skeleton.clone();
        next
    }
}
*/
