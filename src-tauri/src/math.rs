use glam::{Vec3, Mat4, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct ViewMatrix {
    pub matrix: [[f32; 4]; 4],
}

impl ViewMatrix {
    pub fn new(matrix: [[f32; 4]; 4]) -> Self {
        Self { matrix }
    }
    
    pub fn world_to_screen(&self, world_pos: Vec3, screen_size: (f32, f32)) -> Option<(f32, f32)> {
        let clip = Vec4::new(
            world_pos.x * self.matrix[0][0] + world_pos.y * self.matrix[0][1] + world_pos.z * self.matrix[0][2] + self.matrix[0][3],
            world_pos.x * self.matrix[1][0] + world_pos.y * self.matrix[1][1] + world_pos.z * self.matrix[1][2] + self.matrix[1][3],
            world_pos.x * self.matrix[2][0] + world_pos.y * self.matrix[2][1] + world_pos.z * self.matrix[2][2] + self.matrix[2][3],
            world_pos.x * self.matrix[3][0] + world_pos.y * self.matrix[3][1] + world_pos.z * self.matrix[3][2] + self.matrix[3][3],
        );
        
        if clip.w < 0.1 {
            return None;
        }
        
        let ndc = Vec3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w);
        let screen_x = (screen_size.0 / 2.0) * (1.0 + ndc.x);
        let screen_y = (screen_size.1 / 2.0) * (1.0 - ndc.y);
        
        Some((screen_x, screen_y))
    }
}

pub fn calculate_angle(local_pos: Vec3, target_pos: Vec3, view_angles: Vec3) -> Vec3 {
    let delta = target_pos - local_pos;
    let magnitude = (delta.x * delta.x + delta.y * delta.y + delta.z * delta.z).sqrt();
    
    let mut pitch = (delta.z / magnitude).asin().to_degrees();
    let mut yaw = (delta.y / delta.x).atan().to_degrees();
    
    if delta.x > 0.0 {
        yaw += 180.0;
    }
    
    // Normalizar ângulos
    pitch = pitch.clamp(-89.0, 89.0);
    yaw = yaw % 360.0;
    
    let mut angle_diff = Vec3::new(pitch - view_angles.x, yaw - view_angles.y, 0.0);
    
    // Normalizar diferença
    if angle_diff.x > 180.0 { angle_diff.x -= 360.0; }
    if angle_diff.x < -180.0 { angle_diff.x += 360.0; }
    if angle_diff.y > 180.0 { angle_diff.y -= 360.0; }
    if angle_diff.y < -180.0 { angle_diff.y += 360.0; }
    
    angle_diff
}

pub fn get_fov(local_pos: Vec3, target_pos: Vec3, view_angles: Vec3) -> f32 {
    let angle = calculate_angle(local_pos, target_pos, view_angles);
    (angle.x * angle.x + angle.y * angle.y).sqrt()
}

pub fn smooth_angle(current: Vec3, target: Vec3, smooth: f32) -> Vec3 {
    Vec3::new(
        current.x + (target.x - current.x) / smooth,
        current.y + (target.y - current.y) / smooth,
        0.0,
    )
}