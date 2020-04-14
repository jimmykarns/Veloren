pub mod figure;
pub mod fluid;
pub mod postprocess;
pub mod skybox;
pub mod sprite;
pub mod terrain;
pub mod ui;

use super::util::arr_to_mat;
use common::terrain::BlockKind;
use vek::*;
use zerocopy::AsBytes;

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Globals {
    view_mat: [[f32; 4]; 4],
    proj_mat: [[f32; 4]; 4],
    all_mat: [[f32; 4]; 4],
    cam_pos: [f32; 4],
    focus_pos: [f32; 4],
    // TODO: Fix whatever alignment issue requires these uniforms to be aligned.
    view_distance: [f32; 4],
    time_of_day: [f32; 4], // TODO: Make this f64.
    tick: [f32; 4],
    screen_res: [f32; 4],
    light_shadow_count: [u32; 4],
    medium: [u32; 4],
    select_pos: [i32; 4],
    gamma: [f32; 4],
}

impl Globals {
    /// Create global consts from the provided parameters.
    pub fn new(
        view_mat: Mat4<f32>,
        proj_mat: Mat4<f32>,
        cam_pos: Vec3<f32>,
        focus_pos: Vec3<f32>,
        view_distance: f32,
        time_of_day: f64,
        tick: f64,
        screen_res: Vec2<u16>,
        light_count: usize,
        shadow_count: usize,
        medium: BlockKind,
        select_pos: Option<Vec3<i32>>,
        gamma: f32,
    ) -> Self {
        Self {
            view_mat: arr_to_mat(view_mat.into_col_array()),
            proj_mat: arr_to_mat(proj_mat.into_col_array()),
            all_mat: arr_to_mat((proj_mat * view_mat).into_col_array()),
            cam_pos: Vec4::from(cam_pos).into_array(),
            focus_pos: Vec4::from(focus_pos).into_array(),
            view_distance: [view_distance; 4],
            time_of_day: [time_of_day as f32; 4],
            tick: [tick as f32; 4],
            screen_res: Vec4::from(screen_res.map(|e| e as f32)).into_array(),
            light_shadow_count: [light_count as u32, shadow_count as u32, 0, 0],
            medium: [if medium.is_fluid() { 1 } else { 0 }; 4],
            select_pos: select_pos
                .map(|sp| Vec4::from(sp) + Vec4::unit_w())
                .unwrap_or(Vec4::zero())
                .into_array(),
            gamma: [gamma; 4],
        }
    }
}

impl Default for Globals {
    fn default() -> Self {
        Self::new(
            Mat4::identity(),
            Mat4::identity(),
            Vec3::zero(),
            Vec3::zero(),
            0.0,
            0.0,
            0.0,
            Vec2::new(800, 500),
            0,
            0,
            BlockKind::Air,
            None,
            1.0,
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Light {
    pos: [f32; 4],
    col: [f32; 4],
}

impl Light {
    pub fn new(pos: Vec3<f32>, col: Rgb<f32>, strength: f32) -> Self {
        Self {
            pos: Vec4::from(pos).into_array(),
            col: Rgba::new(col.r, col.g, col.b, strength).into_array(),
        }
    }

    pub fn get_pos(&self) -> Vec3<f32> { Vec3::new(self.pos[0], self.pos[1], self.pos[2]) }
}

impl Default for Light {
    fn default() -> Self { Self::new(Vec3::zero(), Rgb::zero(), 0.0) }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, AsBytes)]
pub struct Shadow {
    pos_radius: [f32; 4],
}

impl Shadow {
    pub fn new(pos: Vec3<f32>, radius: f32) -> Self {
        Self {
            pos_radius: [pos.x, pos.y, pos.z, radius],
        }
    }

    pub fn get_pos(&self) -> Vec3<f32> {
        Vec3::new(self.pos_radius[0], self.pos_radius[1], self.pos_radius[2])
    }
}

impl Default for Shadow {
    fn default() -> Self { Self::new(Vec3::zero(), 0.0) }
}

pub struct GlobalsLayouts {
    pub globals: wgpu::BindGroupLayout,
    pub light: wgpu::BindGroupLayout,
    pub shadow: wgpu::BindGroupLayout,
}

impl GlobalsLayouts {
    pub fn new(device: &wgpu::Device) -> Self {
        let globals = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Globals layout"),
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
        });
        let light = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Light layout"),
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
        });
        let shadow = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow layout"),
            bindings: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

        Self {
            globals,
            light,
            shadow,
        }
    }
}
