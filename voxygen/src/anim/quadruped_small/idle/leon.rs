use super::super::{super::Animation, QuadrupedSmallSkeleton, SkeletonAttr};
use std::f32::consts::PI;
use vek::*;

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
                -skeleton_attr.feet_f_0,
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
                -skeleton_attr.feet_b_0,
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

use leon::{object::InvalidOperation, Value};
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
        &mut self,
        field: &str,
        f: Box<dyn FnOnce(&mut Value<'a>) -> Result<(), leon::ExecError> + 'b>,
    ) -> Result<(), leon::ExecError> {
        let mutate_vec3 =
            |f: Box<dyn FnOnce(&mut Value<'a>) -> Result<(), leon::ExecError> + 'b>,
             vec3: &mut Vec3<f32>| {
                let mut value = Value::Custom(Box::new(Vec3Wrap(*vec3)));
                let res = f(&mut value);
                if res.is_err() {
                    return res;
                }
                value
                    .extract::<Vec3Wrap>()
                    .map(|res| *vec3 = res.0)
                    .ok_or_else(|| {
                        leon::ExecError::InvalidObjOperation("Mutated to invalid data type".into())
                    })
            };
        let mutate_quat =
            |f: Box<dyn FnOnce(&mut Value<'a>) -> Result<(), leon::ExecError> + 'b>,
             quat: &mut Quaternion<f32>| {
                let mut value = Value::Custom(Box::new(QuatWrap(*quat)));
                let res = f(&mut value);
                if res.is_err() {
                    return res;
                }
                value
                    .extract::<QuatWrap>()
                    .map(|res| *quat = res.0)
                    .ok_or_else(|| {
                        leon::ExecError::InvalidObjOperation("Mutated to invalid data type".into())
                    })
            };

        match field {
            "head_offset" => mutate_vec3(f, &mut self.head.offset),
            "head_ori" => mutate_quat(f, &mut self.head.ori),
            "head_scale" => mutate_vec3(f, &mut self.head.scale),
            "chest_offset" => mutate_vec3(f, &mut self.chest.offset),
            "chest_ori" => mutate_quat(f, &mut self.chest.ori),
            "chest_scale" => mutate_vec3(f, &mut self.chest.scale),
            "leg_lf_offset" => mutate_vec3(f, &mut self.leg_lf.offset),
            "leg_lf_ori" => mutate_quat(f, &mut self.leg_lf.ori),
            "leg_lf_scale" => mutate_vec3(f, &mut self.leg_lf.scale),
            "leg_rf_offset" => mutate_vec3(f, &mut self.leg_rf.offset),
            "leg_rf_ori" => mutate_quat(f, &mut self.leg_rf.ori),
            "leg_rf_scale" => mutate_vec3(f, &mut self.leg_rf.scale),
            "leg_lb_offset" => mutate_vec3(f, &mut self.leg_lb.offset),
            "leg_lb_ori" => mutate_quat(f, &mut self.leg_lb.ori),
            "leg_lb_scale" => mutate_vec3(f, &mut self.leg_lb.scale),
            "leg_rb_offset" => mutate_vec3(f, &mut self.leg_rb.offset),
            "leg_rb_ori" => mutate_quat(f, &mut self.leg_rb.ori),
            "leg_rb_scale" => mutate_vec3(f, &mut self.leg_rb.scale),
            field => Err(leon::ExecError::InvalidObjOperation(format!(
                "This field doesn't exist: {}",
                field
            ))),
        }
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
            field => Err(InvalidOperation(format!(
                "This field doesn't exist: {}",
                field
            ))),
        }
    }
}

#[derive(Clone, Debug)]
struct Vec2Wrap(Vec2<f32>);
#[derive(Clone, Debug)]
struct Vec3Wrap(Vec3<f32>);
#[derive(Clone, Debug)]
struct QuatWrap(Quaternion<f32>);

impl leon::Object for Vec2Wrap {
    // Get x and y fields
    fn field<'a>(&self, field: &str) -> Result<Value<'a>, InvalidOperation> {
        match field {
            "x" => Ok(Value::Number(self.0.x as f64)),
            "y" => Ok(Value::Number(self.0.y as f64)),
            field => Err(InvalidOperation(format!(
                "This field doesn't exist: {}",
                field
            ))),
        }
    }
}

impl leon::Object for Vec3Wrap {
    // Divide by scalar
    fn div<'a>(&self, rhs: &Value<'a>) -> Result<Value<'a>, InvalidOperation> {
        match rhs {
            Value::Number(d) => Ok(Value::Custom(Box::new(Vec3Wrap(self.0 / *d as f32)))),
            val => Err(InvalidOperation(format!("Not dividable by: {:?}", val))),
        }
    }
}

impl leon::Object for QuatWrap {
    // Multiply by Self
    fn mul<'a>(&self, rhs: &Value<'a>) -> Result<Value<'a>, InvalidOperation> {
        match rhs {
            Value::Custom(o) => o
                .as_any()
                .downcast_ref::<Self>()
                .cloned()
                .map(|v| Value::Custom(Box::new(QuatWrap(self.0 * v.0))))
                .ok_or_else(|| InvalidOperation("Cannot add with provided type".into())),
            _ => Err(InvalidOperation(format!("Cannot add with {:?}", rhs))),
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
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Number(n.sin())),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}
impl leon::Object for Cos {
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Number(n.cos())),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}
impl leon::Object for Floor {
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Number(n.floor())),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}
impl leon::Object for RotX {
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Custom(Box::new(QuatWrap(Quaternion::rotation_x(
                *n as f32,
            ))))),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}
impl leon::Object for RotY {
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Custom(Box::new(QuatWrap(Quaternion::rotation_y(
                *n as f32,
            ))))),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}
impl leon::Object for RotZ {
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(n)] => Ok(Value::Custom(Box::new(QuatWrap(Quaternion::rotation_z(
                *n as f32,
            ))))),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}
impl leon::Object for NewVec2 {
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(x), Value::Number(y)] => Ok(Value::Custom(Box::new(Vec2Wrap(
                Vec2::new(*x as f32, *y as f32),
            )))),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}
impl leon::Object for NewVec3 {
    fn call<'a>(&self, args: &[Value<'a>]) -> Result<Value<'a>, InvalidOperation> {
        match args {
            [Value::Number(b)] => Ok(Value::Custom(Box::new(Vec3Wrap(Vec3::broadcast(
                *b as f32,
            ))))),
            [Value::Number(x), Value::Number(y), Value::Number(z)] => Ok(Value::Custom(Box::new(
                Vec3Wrap(Vec3::new(*x as f32, *y as f32, *z as f32)),
            ))),
            a => Err(InvalidOperation(format!("Invalid args: {:?}", a))),
        }
    }
}

pub struct LeonAst(leon::Ast);
impl LeonAst {
    pub fn new() -> Self { Self(leon::Engine::parse(LEON_IDLE_SCRIPT).unwrap()) }
}

impl<'a> Animation for &'a LeonIdleAnimation {
    type Dependency = (f64, &'a LeonAst);
    type Skeleton = QuadrupedSmallSkeleton;

    fn update_skeleton(
        skeleton: &Self::Skeleton,
        (global_time, ast): Self::Dependency,
        anim_time: f64,
        _rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        let globals = vec![
            ("next".into(), Value::Custom(Box::new(skeleton.clone()))),
            ("anim_time".into(), Value::Number(anim_time)),
            ("global_time".into(), Value::Number(global_time)),
            (
                "skeleton_attr".into(),
                Value::Custom(Box::new(skeleton_attr.clone())),
            ),
            ("sin".into(), Value::Custom(Box::new(Sin))),
            ("cos".into(), Value::Custom(Box::new(Cos))),
            ("floor".into(), Value::Custom(Box::new(Floor))),
            ("rot_x".into(), Value::Custom(Box::new(RotX))),
            ("rot_y".into(), Value::Custom(Box::new(RotY))),
            ("rot_z".into(), Value::Custom(Box::new(RotZ))),
            ("vec2".into(), Value::Custom(Box::new(NewVec2))),
            ("vec3".into(), Value::Custom(Box::new(NewVec3))),
            ("PI".into(), Value::Number(PI as f64)),
        ];

        leon::Engine
            .exec_parsed(&ast.0, globals, |v| {
                v.extract::<QuadrupedSmallSkeleton>().unwrap()
            })
            .unwrap()
    }
}
