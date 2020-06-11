use common::{
    terrain::TerrainGrid,
    vol::{ReadVol, Vox},
};
use std::f32::consts::PI;
use treeculler::Frustum;
use vek::*;

const NEAR_PLANE: f32 = 0.5;
const FAR_PLANE: f32 = 100000.0;

const FIRST_PERSON_INTERP_TIME: f32 = 0.1;
const THIRD_PERSON_INTERP_TIME: f32 = 0.1;
const LERP_ORI_RATE: f32 = 15.0;
pub const MIN_ZOOM: f32 = 0.1;

// Possible TODO: Add more modes
#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub enum CameraMode {
    FirstPerson = 0,
    ThirdPerson = 1,
}

impl Default for CameraMode {
    fn default() -> Self { Self::ThirdPerson }
}

#[derive(Clone)]
pub struct Dependents {
    pub view_mat: Mat4<f32>,
    pub proj_mat: Mat4<f32>,
    pub cam_pos: Vec3<f32>,
}

pub struct Camera {
    tgt_focus: Vec3<f32>,
    focus: Vec3<f32>,
    tgt_ori: Vec3<f32>,
    ori: Vec3<f32>,
    tgt_dist: f32,
    dist: f32,
    fov: f32,
    aspect: f32,
    mode: CameraMode,

    last_time: Option<f64>,

    enclosed: bool,
    enclosure_max_distance: f32,
    enclosure_check_step: u8,
    enclosure_check_total: u8,
    enclosure_check_hit: u8,
    enclosure_current_camera_distance: f32,
    target_distance_before_enclosure: f32,

    dependents: Dependents,
}

impl Camera {
    /// Create a new `Camera` with default parameters.
    pub fn new(aspect: f32, mode: CameraMode) -> Self {
        Self {
            tgt_focus: Vec3::unit_z() * 10.0,
            focus: Vec3::unit_z() * 10.0,
            tgt_ori: Vec3::zero(),
            ori: Vec3::zero(),
            tgt_dist: 10.0,
            dist: 10.0,
            fov: 1.1,
            aspect,
            mode,

            last_time: None,

            // Whether the camera was detected to be inside of a structure. Switches us to indoor
            // camera.
            enclosed: false,

            enclosure_max_distance: 0.0,
            enclosure_check_step: 0,
            enclosure_check_total: 0,
            enclosure_check_hit: 0,
            enclosure_current_camera_distance: 0.0,
            target_distance_before_enclosure: 0.0,

            dependents: Dependents {
                view_mat: Mat4::identity(),
                proj_mat: Mat4::identity(),
                cam_pos: Vec3::zero(),
            },
        }
    }

    pub fn build_up_enclosure_check(&mut self, terrain: &TerrainGrid) {
        let ray_distance = 30.0;
        let raycast_resolution = 75;
        let horizontal_resolution = 12;

        let horizontal_angle =
            (self.enclosure_check_step as f32 * PI) / (horizontal_resolution as f32 / 2.0);
        let (start, end) = (
            self.focus,
            self.focus
                + (Vec3::new(
                    -f32::sin(horizontal_angle as f32),
                    -f32::cos(horizontal_angle as f32),
                    0.0,
                ) * ray_distance),
        );
        self.enclosure_check_total += 1;
        if let (d, Ok(Some(_))) = terrain
            .ray(start, end)
            .ignore_error()
            .max_iter(raycast_resolution)
            .until(|b| b.is_solid())
            .cast()
        {
            self.enclosure_check_hit += 1;
            if d > self.enclosure_max_distance {
                self.enclosure_max_distance = d;
            }
        }

        for vertical in 0..3 {
            let vertical_angle = (vertical as f32 * PI) / 6.0;
            let (vertical_start, vertical_end) = (
                self.focus,
                self.focus
                    + (Vec3::new(
                        -f32::sin(horizontal_angle) * f32::cos(vertical_angle),
                        -f32::cos(horizontal_angle) * f32::cos(vertical_angle),
                        f32::sin(vertical_angle),
                    ) * ray_distance),
            );

            self.enclosure_check_total += 1;
            if let (d, Ok(Some(_))) = terrain
                .ray(vertical_start, vertical_end)
                .ignore_error()
                .max_iter(raycast_resolution)
                .until(|b| b.is_solid())
                .cast()
            {
                self.enclosure_check_hit += 1;
                if d > self.enclosure_max_distance {
                    self.enclosure_max_distance = d;
                }
            }
        }

        self.enclosure_check_step += 1;
        if self.enclosure_check_step >= horizontal_resolution {
            let enclosure_percentage =
                self.enclosure_check_hit as f32 / self.enclosure_check_total as f32;
            if enclosure_percentage >= 0.9 {
                if !self.enclosed {
                    self.target_distance_before_enclosure = self.tgt_dist;
                }
                self.enclosed = true;
            } else {
                if self.enclosed {
                    self.dist = self.enclosure_current_camera_distance;
                    self.tgt_dist = self.target_distance_before_enclosure;
                }
                self.enclosed = false;
                self.enclosure_max_distance = 0.0;
                self.target_distance_before_enclosure = 0.0;
            }
            self.enclosure_check_step = 0;
            self.enclosure_check_total = 0;
            self.enclosure_check_hit = 0;
        }
    }

    fn compute_dependents_given_distance(&mut self, distance: f32) {
        self.dependents.view_mat = Mat4::<f32>::identity()
            * Mat4::translation_3d(-Vec3::unit_z() * distance)
            * Mat4::rotation_z(self.ori.z)
            * Mat4::rotation_x(self.ori.y)
            * Mat4::rotation_y(self.ori.x)
            * Mat4::rotation_3d(PI / 2.0, -Vec4::unit_x())
            * Mat4::translation_3d(-self.focus);

        self.dependents.proj_mat =
            Mat4::perspective_rh_no(self.fov, self.aspect, NEAR_PLANE, FAR_PLANE);

        // TODO: Make this more efficient.
        self.dependents.cam_pos = Vec3::from(self.dependents.view_mat.inverted() * Vec4::unit_w());
    }

    /// Compute_dependents adjusted for when there is no terrain data (character
    /// selection)
    pub fn compute_dependents_no_terrain(&mut self) {
        self.compute_dependents_given_distance(self.dist);
    }

    /// Compute the transformation matrices (view matrix and projection matrix)
    /// and position of the camera.
    pub fn compute_dependents(&mut self, terrain: &TerrainGrid) {
        let dist = {
            let (start, end) = (
                self.focus
                    + (Vec3::new(
                        -f32::sin(self.ori.x) * f32::cos(self.ori.y),
                        -f32::cos(self.ori.x) * f32::cos(self.ori.y),
                        f32::sin(self.ori.y),
                    ) * self.dist),
                self.focus,
            );

            if self.enclosed {
                match terrain
                    .ray(start, end)
                    .ignore_error()
                    .max_iter(500)
                    .until(|vox| !vox.is_solid())
                    .last_edge_cast()
                {
                    Ok(d) => {
                        if d >= self.dist {
                            self.dist
                        } else {
                            f32::min(self.dist - d - 0.1, self.dist)
                        }
                    },
                    Err(_) => self.dist,
                }
            } else {
                match terrain
                    .ray(start, end)
                    .ignore_error()
                    .max_iter(500)
                    .until(|b| b.is_empty())
                    .cast()
                {
                    (d, Ok(Some(_))) => f32::min(self.dist - d - 0.03, self.dist),
                    (_, Ok(None)) => self.dist,
                    (_, Err(_)) => self.dist,
                }
                .max(0.0)
            }
        };
        if self.enclosed {
            self.enclosure_current_camera_distance = dist;
            if self.tgt_dist > self.enclosure_max_distance && self.tgt_dist > self.dist {
                self.tgt_dist = dist;
            }
        }
        self.compute_dependents_given_distance(dist);
    }

    pub fn frustum(&self) -> Frustum<f32> {
        Frustum::from_modelview_projection(
            (self.dependents.proj_mat * self.dependents.view_mat).into_col_arrays(),
        )
    }

    pub fn dependents(&self) -> Dependents { self.dependents.clone() }

    /// Rotate the camera about its focus by the given delta, limiting the input
    /// accordingly.
    pub fn rotate_by(&mut self, delta: Vec3<f32>) {
        // Wrap camera yaw
        self.tgt_ori.x = (self.tgt_ori.x + delta.x).rem_euclid(2.0 * PI);
        // Clamp camera pitch to the vertical limits
        self.tgt_ori.y = (self.tgt_ori.y + delta.y)
            .min(PI / 2.0 - 0.0001)
            .max(-PI / 2.0 + 0.0001);
        // Wrap camera roll
        self.tgt_ori.z = (self.tgt_ori.z + delta.z).rem_euclid(2.0 * PI);
    }

    /// Set the orientation of the camera about its focus.
    pub fn set_orientation(&mut self, ori: Vec3<f32>) {
        // Wrap camera yaw
        self.tgt_ori.x = ori.x.rem_euclid(2.0 * PI);
        // Clamp camera pitch to the vertical limits
        self.tgt_ori.y = ori.y.min(PI / 2.0 - 0.0001).max(-PI / 2.0 + 0.0001);
        // Wrap camera roll
        self.tgt_ori.z = ori.z.rem_euclid(2.0 * PI);
    }

    /// Set the orientation of the camera about its focus without lerping.
    pub fn set_ori_instant(&mut self, ori: Vec3<f32>) {
        // Wrap camera yaw
        self.ori.x = ori.x.rem_euclid(2.0 * PI);
        // Clamp camera pitch to the vertical limits
        self.ori.y = ori.y.min(PI / 2.0 - 0.0001).max(-PI / 2.0 + 0.0001);
        // Wrap camera roll
        self.ori.z = ori.z.rem_euclid(2.0 * PI);
    }

    /// Zoom the camera by the given delta, limiting the input accordingly.
    pub fn zoom_by(&mut self, delta: f32) {
        match self.mode {
            CameraMode::ThirdPerson => {
                // Clamp camera dist to the 2 <= x <= infinity range
                self.tgt_dist = (self.tgt_dist + delta).max(2.0);
            },
            CameraMode::FirstPerson => {},
        };
    }

    /// Zoom with the ability to switch between first and third-person mode.
    pub fn zoom_switch(&mut self, delta: f32) {
        if delta > 0_f32 || self.mode != CameraMode::FirstPerson {
            let t = self.tgt_dist + delta;
            const MIN_THIRD_PERSON: f32 = 2.35;
            match self.mode {
                CameraMode::ThirdPerson => {
                    if t < MIN_THIRD_PERSON {
                        self.set_mode(CameraMode::FirstPerson);
                    } else {
                        self.tgt_dist = t;
                        if self.enclosed && self.tgt_dist > self.enclosure_max_distance {
                            self.tgt_dist = self.enclosure_max_distance;
                        }
                    }
                },
                CameraMode::FirstPerson => {
                    self.set_mode(CameraMode::ThirdPerson);
                    self.tgt_dist = MIN_THIRD_PERSON;
                },
            }
        }
    }

    /// Get the distance of the camera from the focus
    pub fn get_distance(&self) -> f32 { self.dist }

    /// Set the distance of the camera from the focus (i.e., zoom).
    pub fn set_distance(&mut self, dist: f32) { self.tgt_dist = dist; }

    pub fn update(
        &mut self,
        time: f64,
        dt: f32,
        smoothing_enabled: bool,
        terrain: Option<&TerrainGrid>,
    ) {
        if let Some(terrain) = terrain {
            self.build_up_enclosure_check(&*terrain);
        }

        // This is horribly frame time dependent, but so is most of the game
        let delta = self.last_time.replace(time).map_or(0.0, |t| time - t);
        if (self.dist - self.tgt_dist).abs() > 0.01 {
            self.dist = f32::lerp(
                self.dist,
                self.tgt_dist,
                0.65 * (delta as f32) / self.interp_time(),
            );
        }

        if (self.focus - self.tgt_focus).magnitude_squared() > 0.001 {
            let lerped_focus = Lerp::lerp(
                self.focus,
                self.tgt_focus,
                (delta as f32) / self.interp_time()
                    * if matches!(self.mode, CameraMode::FirstPerson) {
                        2.0
                    } else {
                        1.0
                    },
            );

            self.focus.x = lerped_focus.x;
            self.focus.y = lerped_focus.y;

            // Always lerp in z
            self.focus.z = lerped_focus.z;
        }

        let lerp_angle = |a: f32, b: f32, rate: f32| {
            let offs = [-2.0 * PI, 0.0, 2.0 * PI]
                .iter()
                .min_by_key(|offs: &&f32| ((a - (b + *offs)).abs() * 1000.0) as i32)
                .unwrap();
            Lerp::lerp(a, b + *offs, rate)
        };

        if smoothing_enabled {
            self.set_ori_instant(Vec3::new(
                lerp_angle(self.ori.x, self.tgt_ori.x, LERP_ORI_RATE * dt),
                Lerp::lerp(self.ori.y, self.tgt_ori.y, LERP_ORI_RATE * dt),
                lerp_angle(self.ori.z, self.tgt_ori.z, LERP_ORI_RATE * dt),
            ));
        } else {
            self.set_ori_instant(self.tgt_ori)
        };
    }

    pub fn interp_time(&self) -> f32 {
        match self.mode {
            CameraMode::FirstPerson => FIRST_PERSON_INTERP_TIME,
            CameraMode::ThirdPerson => THIRD_PERSON_INTERP_TIME,
        }
    }

    /// Get the focus position of the camera.
    pub fn get_focus_pos(&self) -> Vec3<f32> { self.focus }

    /// Set the focus position of the camera.
    pub fn set_focus_pos(&mut self, focus: Vec3<f32>) { self.tgt_focus = focus; }

    /// Get the aspect ratio of the camera.
    pub fn get_aspect_ratio(&self) -> f32 { self.aspect }

    /// Set the aspect ratio of the camera.
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = if aspect.is_normal() { aspect } else { 1.0 };
    }

    /// Get the orientation of the camera.
    pub fn get_orientation(&self) -> Vec3<f32> { self.ori }

    /// Get the field of view of the camera in radians.
    pub fn get_fov(&self) -> f32 { self.fov }

    /// Set the field of view of the camera in radians.
    pub fn set_fov(&mut self, fov: f32) { self.fov = fov; }

    /// Set the FOV in degrees
    pub fn set_fov_deg(&mut self, fov: u16) {
        //Magic value comes from pi/180; no use recalculating.
        self.set_fov((fov as f32) * 0.01745329)
    }

    /// Set the mode of the camera.
    pub fn set_mode(&mut self, mode: CameraMode) {
        if self.mode != mode {
            self.mode = mode;
            match self.mode {
                CameraMode::ThirdPerson => {
                    self.zoom_by(5.0);
                },
                CameraMode::FirstPerson => {
                    self.set_distance(MIN_ZOOM);
                },
            }
        }
    }

    /// Get the mode of the camera
    pub fn get_mode(&self) -> CameraMode {
        // Perfom a bit of a trick... don't report first-person until the camera has
        // lerped close enough to the player.
        match self.mode {
            CameraMode::FirstPerson if self.dist < 0.5 => CameraMode::FirstPerson,
            CameraMode::FirstPerson => CameraMode::ThirdPerson,
            mode => mode,
        }
    }
}
