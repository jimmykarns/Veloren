use super::{
    super::{
        consts::Consts,
        instances::Instances,
        model::Model,
        pipelines::{figure, fluid, sprite, terrain, ui, Globals, Light, Shadow},
    },
    Renderer,
};
use std::ops::Range;
use vek::Aabr;

pub struct Drawer<'a> {
    pub(super) encoder: Option<wgpu::CommandEncoder>,
    pub(super) renderer: &'a mut Renderer,
    pub(super) tex: wgpu::SwapChainOutput,
    pub(super) postprocess_locals: wgpu::BindGroup,
}

impl<'a> Drawer<'a> {
    pub fn first_render(&mut self) -> FirstDrawer {
        let render_pass =
            self.encoder
                .as_mut()
                .unwrap()
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.renderer.tgt_color_texture.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::TRANSPARENT,
                    }],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: &self.renderer.depth_stencil_texture.view,
                            depth_load_op: wgpu::LoadOp::Clear,
                            depth_store_op: wgpu::StoreOp::Store,
                            clear_depth: 1.0,
                            stencil_load_op: wgpu::LoadOp::Clear,
                            stencil_store_op: wgpu::StoreOp::Store,
                            clear_stencil: 0,
                        },
                    ),
                });

        FirstDrawer {
            render_pass,
            renderer: &self.renderer,
        }
    }

    pub fn second_render(&mut self) -> SecondDrawer {
        let render_pass =
            self.encoder
                .as_mut()
                .unwrap()
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.tex.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::TRANSPARENT, /*wgpu::Color {
                                                                   r: 0.1,
                                                                   g: 0.2,
                                                                   b: 0.3,
                                                                   a: 1.0,
                                                               }*/
                    }],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachmentDescriptor {
                            attachment: &self.renderer.depth_stencil_texture.view,
                            depth_load_op: wgpu::LoadOp::Load,
                            depth_store_op: wgpu::StoreOp::Store,
                            clear_depth: 1.0,
                            stencil_load_op: wgpu::LoadOp::Load,
                            stencil_store_op: wgpu::StoreOp::Store,
                            clear_stencil: 0,
                        },
                    ),
                });

        SecondDrawer {
            render_pass,
            renderer: &self.renderer,
            postprocess_locals: &self.postprocess_locals,
        }
    }
}

impl<'a> Drop for Drawer<'a> {
    fn drop(&mut self) {
        self.renderer
            .queue
            .submit(&[self.encoder.take().unwrap().finish()]);
    }
}

pub struct FirstDrawer<'a> {
    pub(super) render_pass: wgpu::RenderPass<'a>,
    pub renderer: &'a Renderer,
}

impl<'a> FirstDrawer<'a> {
    pub fn draw_skybox<'b: 'a>(
        &mut self,
        model: &'b Model,
        globals: &'b Consts<Globals>,
        verts: Range<u32>,
    ) {
        self.render_pass
            .set_pipeline(&self.renderer.skybox_pipeline.pipeline);
        self.render_pass.set_bind_group(0, &globals.bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
        self.render_pass.draw(verts, 0..1);
    }

    pub fn draw_figure<'b: 'a>(
        &mut self,
        model: &'b Model,
        locals: &'b Consts<figure::Locals>,
        bones: &'b Consts<figure::BoneData>,
        globals: &'b Consts<Globals>,
        lights: &'b Consts<Light>,
        shadows: &'b Consts<Shadow>,
        verts: Range<u32>,
    ) {
        self.render_pass
            .set_pipeline(&self.renderer.figure_pipeline.pipeline);
        self.render_pass.set_bind_group(0, &globals.bind_group, &[]);
        self.render_pass.set_bind_group(1, &lights.bind_group, &[]);
        self.render_pass.set_bind_group(2, &shadows.bind_group, &[]);
        self.render_pass.set_bind_group(3, &locals.bind_group, &[]);
        self.render_pass.set_bind_group(4, &bones.bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
        self.render_pass.draw(verts, 0..1);
    }

    pub fn draw_terrain<'b: 'a>(
        &mut self,
        model: &'b Model,
        locals: &'b Consts<terrain::Locals>,
        globals: &'b Consts<Globals>,
        lights: &'b Consts<Light>,
        shadows: &'b Consts<Shadow>,
        verts: Range<u32>,
    ) {
        self.render_pass
            .set_pipeline(&self.renderer.terrain_pipeline.pipeline);
        self.render_pass.set_bind_group(0, &globals.bind_group, &[]);
        self.render_pass.set_bind_group(1, &lights.bind_group, &[]);
        self.render_pass.set_bind_group(2, &shadows.bind_group, &[]);
        self.render_pass.set_bind_group(3, &locals.bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
        self.render_pass.draw(verts, 0..1)
    }

    pub fn draw_fluid<'b: 'a>(
        &mut self,
        model: &'b Model,
        locals: &'b Consts<terrain::Locals>,
        waves: &'b Consts<fluid::Locals>,
        globals: &'b Consts<Globals>,
        lights: &'b Consts<Light>,
        shadows: &'b Consts<Shadow>,
        verts: Range<u32>,
    ) {
        self.render_pass
            .set_pipeline(&self.renderer.fluid_pipeline.pipeline);
        self.render_pass.set_bind_group(0, &globals.bind_group, &[]);
        self.render_pass.set_bind_group(1, &lights.bind_group, &[]);
        self.render_pass.set_bind_group(2, &shadows.bind_group, &[]);
        self.render_pass.set_bind_group(3, &locals.bind_group, &[]);
        self.render_pass.set_bind_group(4, &waves.bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
        self.render_pass.draw(verts, 0..1);
    }

    pub fn draw_sprite<'b: 'a>(
        &mut self,
        model: &'b Model,
        instances: &'a Instances<sprite::Instance>,
        globals: &'b Consts<Globals>,
        lights: &'b Consts<Light>,
        shadows: &'b Consts<Shadow>,
        verts: Range<u32>,
    ) {
        self.render_pass
            .set_pipeline(&self.renderer.sprite_pipeline.pipeline);
        self.render_pass.set_bind_group(0, &globals.bind_group, &[]);
        self.render_pass.set_bind_group(1, &lights.bind_group, &[]);
        self.render_pass.set_bind_group(2, &shadows.bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
        self.render_pass.set_vertex_buffer(1, &instances.ibuf, 0, 0);
        self.render_pass.draw(verts, 0..instances.count() as u32);
    }
}

pub struct SecondDrawer<'a> {
    pub(super) render_pass: wgpu::RenderPass<'a>,
    pub renderer: &'a Renderer,
    pub postprocess_locals: &'a wgpu::BindGroup,
}

impl<'a> SecondDrawer<'a> {
    pub fn draw_post_process<'b: 'a>(
        &mut self,
        model: &'b Model,
        globals: &'b Consts<Globals>,
        verts: Range<u32>,
    ) {
        self.render_pass
            .set_pipeline(&self.renderer.postprocess_pipeline.pipeline);
        self.render_pass.set_bind_group(0, &globals.bind_group, &[]);
        self.render_pass
            .set_bind_group(1, self.postprocess_locals, &[]);
        self.render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
        self.render_pass.draw(verts, 0..1);
    }

    pub fn start_draw_ui<'b: 'a>(&mut self, globals: &'b Consts<Globals>) {
        self.render_pass
            .set_pipeline(&self.renderer.ui_pipeline.pipeline);
        self.render_pass.set_bind_group(0, &globals.bind_group, &[]);
    }

    pub fn draw_ui<'b: 'a>(
        &mut self,
        model: &'b Model,
        scissor: Aabr<u16>,
        locals: &'b Consts<ui::Locals>,
        /* globals: &'b Consts<Globals>, */
        verts: Range<u32>,
    ) {
        let Aabr { min, max } = scissor;

        self.render_pass.set_bind_group(1, &locals.bind_group, &[]);
        self.render_pass.set_vertex_buffer(0, &model.vbuf, 0, 0);
        self.render_pass.set_scissor_rect(
            min.x as u32,
            min.y as u32,
            (max.x - min.x) as u32,
            (max.y - min.y) as u32,
        );
        self.render_pass.draw(verts, 0..1);
    }
}
