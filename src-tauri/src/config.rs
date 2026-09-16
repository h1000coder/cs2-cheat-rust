use std::sync::LazyLock;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CheatConfig {
    // ========== AIMBOT OPTIONS ==========
    pub aimbot_enabled: bool,
    pub fov_enabled: bool,
    pub fov_size: f32,
    pub aimbot_smooth: f32,
    pub aimbot_key: i32,

    //Triggerbot
    pub trigger_bot_enabled: bool,
    pub trigger_delay_ms: u64,
    pub trigger_key: i32,  
    pub trigger_hold_mode: bool, 
    pub trigger_team_check: bool,
    pub trigger_scan_delay_ms: u64,
    
    // ========== ESP OPTIONS ==========
    pub esp_enabled: bool,
    pub esp_box_enabled: bool,
    pub esp_filled_box_enabled: bool,
    pub esp_health_bar_enabled: bool,
    pub esp_name_enabled: bool,
    pub esp_distance_enabled: bool,
    pub esp_skeleton_enabled: bool,
    
    // ========== CORES ==========
    pub box_color_ct: [f32; 4],
    pub fill_box_color_ct: [f32; 4],
    pub box_color_t: [f32; 4],
    pub fill_box_color_t: [f32; 4],
    pub name_color_ct: [f32; 4],
    pub name_color_t: [f32; 4],
    pub skeleton_color_ct: [f32; 4],
    pub skeleton_color_t: [f32; 4],
    pub fov_color: [f32; 4],
    
    // ========== CORES DA VIDA ==========
    pub health_high_color: [f32; 4],
    pub health_medium_color: [f32; 4],
    pub health_low_color: [f32; 4],
    
    // ========== AJUSTES ==========
    pub health_bar_width: f32,
    pub box_thickness: f32,
    pub skeleton_thickness: f32,
    pub head_offset_y: f32,

    //TRIGGERBOT
    
}

impl Default for CheatConfig {
    fn default() -> Self {
        Self {
            // AIMBOT
            aimbot_enabled: false,
            fov_enabled: true,
            fov_size: 100.0,
            aimbot_smooth: 5.0,
            aimbot_key: 0x01,

            // TRIGGERBOT
            trigger_bot_enabled: false,
            trigger_delay_ms: 0,
            trigger_key: 0x02,
            trigger_hold_mode: true,
            trigger_team_check: true,
            trigger_scan_delay_ms: 1,
            
            // ESP
            esp_enabled: true,
            esp_box_enabled: true,
            esp_filled_box_enabled: false,
            esp_health_bar_enabled: true,
            esp_name_enabled: true,
            esp_distance_enabled: true,
            esp_skeleton_enabled: false,
            
            // CORES
            box_color_ct: [0.0, 0.5, 1.0, 1.0],
            fill_box_color_ct: [0.0, 0.5, 1.0, 0.3],
            box_color_t: [1.0, 0.2, 0.0, 1.0],
            fill_box_color_t: [1.0, 0.2, 0.0, 0.3],
            name_color_ct: [0.5, 0.7, 1.0, 1.0],
            name_color_t: [1.0, 0.5, 0.3, 1.0],
            skeleton_color_ct: [0.0, 0.5, 1.0, 0.8],
            skeleton_color_t: [1.0, 0.2, 0.0, 0.8],
            fov_color: [1.0, 1.0, 1.0, 0.8],
            
            // HEALTH COLORS
            health_high_color: [0.0, 1.0, 0.0, 0.9],
            health_medium_color: [1.0, 1.0, 0.0, 0.9],
            health_low_color: [1.0, 0.0, 0.0, 0.9],
            
            // AJUSTES
            health_bar_width: 6.0,
            box_thickness: 2.0,
            skeleton_thickness: 1.5,
            head_offset_y: -12.0,
        }
    }
}

// Configuração global
pub static CONFIG: LazyLock<Mutex<CheatConfig>> = LazyLock::new(|| Mutex::new(CheatConfig::default()));