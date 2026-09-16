use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::config::CONFIG;
use crate::game::GameData;
use crate::math::ViewMatrix;
use crate::offsets::Offsets;
use glam::Vec3;
use imgui::Ui;

static SCREEN_WIDTH: AtomicU32 = AtomicU32::new(1920);
static SCREEN_HEIGHT: AtomicU32 = AtomicU32::new(1080);

pub fn set_screen_resolution(width: u32, height: u32) {
    SCREEN_WIDTH.store(width, Ordering::Relaxed);
    SCREEN_HEIGHT.store(height, Ordering::Relaxed);
}

pub fn get_screen_resolution() -> (f32, f32) {
    (
        SCREEN_WIDTH.load(Ordering::Relaxed) as f32,
        SCREEN_HEIGHT.load(Ordering::Relaxed) as f32,
    )
}

pub struct EspRenderer;

impl EspRenderer {
    pub fn render(ui: &Ui, data: &GameData) {
        let config = CONFIG.lock().unwrap();

        if !config.esp_enabled || data.enemies.is_empty() {
            return;
        }

        let draw_list = ui.get_background_draw_list();
        let [screen_width, screen_height] = ui.io().display_size;
        set_screen_resolution(screen_width as u32, screen_height as u32);
        let screen_size = (screen_width, screen_height);
        let center = [screen_width / 2.0, screen_height / 2.0];

        if config.fov_enabled {
            Self::render_fov_circle(&draw_list, center, &config);
        }
        
        for enemy in &data.enemies {
            Self::render_enemy(&draw_list, enemy, data.view_matrix, screen_size, &config);
        }
    }

    fn render_fov_circle(
        draw_list: &imgui::DrawListMut,
        center: [f32; 2],
        config: &crate::config::CheatConfig,
    ) {
        let radius = config.fov_size;
        let color = config.fov_color;

        draw_list
            .add_circle(center, radius, color)
            .thickness(2.0)
            .num_segments(64)
            .build();
    }

    // Renderiza o glow (brilho) ao redor do box - SEMPRE ATIVO
    fn render_glow(
        draw_list: &imgui::DrawListMut,
        top_left: [f32; 2],
        bottom_right: [f32; 2],
        color: [f32; 4],
        intensity: f32,
    ) {
        let glow_steps = 4;
        let glow_factor = 0.3 * intensity;
        
        for i in 1..=glow_steps {
            let alpha = color[3] * (1.0 - (i as f32 / glow_steps as f32)) * glow_factor;
            let glow_color = [color[0], color[1], color[2], alpha];
            let offset = i as f32 * 2.0;
            
            let glow_top_left = [top_left[0] - offset, top_left[1] - offset];
            let glow_bottom_right = [bottom_right[0] + offset, bottom_right[1] + offset];
            
            draw_list
                .add_rect(glow_top_left, glow_bottom_right, glow_color)
                .thickness(1.0)
                .build();
        }
    }

    // Renderiza glow no texto - SEMPRE ATIVO
    fn render_text_glow(
        draw_list: &imgui::DrawListMut,
        text: &str,
        pos: [f32; 2],
        color: [f32; 4],
        intensity: f32,
    ) {
        let glow_steps = 3;
        let glow_factor = 0.25 * intensity;
        
        for i in (1..=glow_steps).rev() {
            let alpha = color[3] * (1.0 - (i as f32 / glow_steps as f32)) * glow_factor;
            let glow_color = [color[0], color[1], color[2], alpha];
            let offset = i as f32;
            
            for ox in -offset as i32..=offset as i32 {
                for oy in -offset as i32..=offset as i32 {
                    if ox == 0 && oy == 0 {
                        continue;
                    }
                    draw_list.add_text([pos[0] + ox as f32, pos[1] + oy as f32], glow_color, text);
                }
            }
        }
    }

    fn render_skeleton(
        draw_list: &imgui::DrawListMut,
        enemy: &crate::game::EnemyData,
        view_matrix: &ViewMatrix,
        screen_size: (f32, f32),
        config: &crate::config::CheatConfig,
        skeleton_color: [f32; 4],
    ) {
        if enemy.bone_positions.is_empty() {
            return;
        }

        let distance_meters = enemy.distance / 10.0;
        let adjusted_skeleton_thickness = if distance_meters < 20.0 {
            config.skeleton_thickness
        } else if distance_meters < 50.0 {
            config.skeleton_thickness * 0.6
        } else {
            config.skeleton_thickness * 0.3
        }
        .max(0.5);

        let connections = [
            (Offsets::BONE_HEAD, Offsets::BONE_NECK_END),
            (Offsets::BONE_NECK_END, Offsets::BONE_PELVIS),
            (Offsets::BONE_NECK_END, Offsets::BONE_LEFT_SHOULDER),
            (Offsets::BONE_NECK_END, Offsets::BONE_RIGHT_SHOULDER),
            (Offsets::BONE_RIGHT_SHOULDER, Offsets::BONE_RIGHT_ELBOW),
            (Offsets::BONE_LEFT_SHOULDER, Offsets::BONE_LEFT_ELBOW),
            (Offsets::BONE_LEFT_ELBOW, Offsets::HAND_L_BONE),
            (Offsets::BONE_RIGHT_ELBOW, Offsets::HAND_R_BONE),
            (Offsets::BONE_PELVIS, Offsets::BONE_LEFT_HIP),
            (Offsets::BONE_LEFT_HIP, Offsets::BONE_LEFT_KNEE),
            (Offsets::BONE_LEFT_KNEE, Offsets::BONE_LEFT_FOOT),
            (Offsets::BONE_PELVIS, Offsets::BONE_RIGHT_HIP),
            (Offsets::BONE_RIGHT_HIP, Offsets::BONE_RIGHT_KNEE),
            (Offsets::BONE_RIGHT_KNEE, Offsets::BONE_RIGHT_FOOT),
        ];

        let mut screen_positions: HashMap<usize, [f32; 2]> = HashMap::new();

        for (&bone_id, &bone_pos) in &enemy.bone_positions {
            if let Some(screen_pos) = view_matrix.world_to_screen(bone_pos, screen_size) {
                if screen_pos.0 > 0.0
                    && screen_pos.0 < screen_size.0
                    && screen_pos.1 > 0.0
                    && screen_pos.1 < screen_size.1
                {
                    screen_positions.insert(bone_id, [screen_pos.0, screen_pos.1]);
                }
            }
        }

        let head_radius = (300.0 / distance_meters).clamp(3.0, 12.0);

        if let Some(&head_pos) = screen_positions.get(&Offsets::BONE_HEAD) {
            draw_list
                .add_circle(head_pos, head_radius, skeleton_color)
                .thickness(adjusted_skeleton_thickness)
                .build();
        }

        for (bone1, bone2) in connections {
            if let (Some(&pos1), Some(&pos2)) =
                (screen_positions.get(&bone1), screen_positions.get(&bone2))
            {
                let dx = pos1[0] - pos2[0];
                let dy = pos1[1] - pos2[1];
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < 500.0 {
                    draw_list
                        .add_line(pos1, pos2, skeleton_color)
                        .thickness(adjusted_skeleton_thickness)
                        .build();
                }
            }
        }
    }

    fn render_enemy(
        draw_list: &imgui::DrawListMut,
        enemy: &crate::game::EnemyData,
        view_matrix_opt: Option<[[f32; 4]; 4]>,
        screen_size: (f32, f32),
        config: &crate::config::CheatConfig,
    ) {
        let view_matrix = match view_matrix_opt {
            Some(vm) => ViewMatrix::new(vm),
            None => return,
        };

        let (head_screen, foot_screen) =
            match Self::get_screen_positions(&view_matrix, enemy, screen_size) {
                Some(positions) => positions,
                None => return,
            };

        let (head_x, head_y) = head_screen;
        let (foot_x, foot_y) = foot_screen;

        let box_height = foot_y - head_y;
        let box_width = box_height * 0.5;

        let top_left = [head_x - box_width / 2.0, head_y];
        let bottom_right = [head_x + box_width / 2.0, foot_y];

        let box_color = if enemy.team == 2 {
            config.box_color_ct
        } else {
            config.box_color_t
        };

        let glow_intensity = 1.0;

        let distance_meters = enemy.distance / 10.0;
        let adjusted_thickness = if distance_meters < 20.0 {
            config.box_thickness
        } else if distance_meters < 50.0 {
            config.box_thickness * 0.7
        } else {
            config.box_thickness * 0.4
        }
        .max(0.5);

        // 🔥 GLOW DO BOX - SEMPRE ATIVO (sem verificação de configuração)
        if config.esp_box_enabled {
            Self::render_glow(draw_list, top_left, bottom_right, box_color, glow_intensity);
        }

        if config.esp_box_enabled {
            draw_list
                .add_rect(top_left, bottom_right, box_color)
                .thickness(adjusted_thickness)
                .build();
        }

        if config.esp_health_bar_enabled {
            Self::render_health_bar(
                draw_list,
                head_x,
                head_y,
                box_width,
                box_height,
                enemy.health,
                config,
                enemy.distance,
            );
        }

        if config.esp_name_enabled || config.esp_distance_enabled {
            let name_color = if enemy.team == 2 {
                config.name_color_ct
            } else {
                config.name_color_t
            };
            Self::render_texts(draw_list, head_x, head_y, foot_y, enemy, name_color, config, glow_intensity);
        }

        if config.esp_filled_box_enabled {
            let mut fill_color = box_color;
            fill_color[3] *= 0.6;

            Self::render_box_fill(draw_list, top_left, bottom_right, fill_color);
        }

        if config.esp_skeleton_enabled {
            let skeleton_color = if enemy.team == 2 {
                config.skeleton_color_ct
            } else {
                config.skeleton_color_t
            };
            Self::render_skeleton(
                draw_list,
                enemy,
                &view_matrix,
                screen_size,
                config,
                skeleton_color,
            );
        }
    }

    fn get_screen_positions(
        view_matrix: &ViewMatrix,
        enemy: &crate::game::EnemyData,
        screen_size: (f32, f32),
    ) -> Option<((f32, f32), (f32, f32))> {
        let head = view_matrix.world_to_screen(enemy.head_position, screen_size)?;
        let foot = view_matrix.world_to_screen(enemy.position, screen_size)?;
        Some((head, foot))
    }

    fn render_health_bar(
        draw_list: &imgui::DrawListMut,
        head_x: f32,
        head_y: f32,
        box_width: f32,
        box_height: f32,
        health: i32,
        config: &crate::config::CheatConfig,
        distance: f32,
    ) {
        let distance_meters = distance / 10.0;
        let bar_width = if distance_meters < 20.0 {
            config.health_bar_width
        } else if distance_meters < 50.0 {
            config.health_bar_width * 0.7
        } else {
            config.health_bar_width * 0.4
        }
        .max(2.0);

        let bar_x = head_x - box_width / 2.0 - bar_width - 2.0;
        let bar_y = head_y;

        draw_list
            .add_rect(
                [bar_x, bar_y],
                [bar_x + bar_width, bar_y + box_height],
                [0.2, 0.2, 0.2, 0.8],
            )
            .filled(true)
            .build();

        let health_percent = (health as f32 / 100.0).clamp(0.0, 1.0);
        let health_height = box_height * health_percent;
        let health_y = bar_y + (box_height - health_height);

        let health_color = if health_percent > 0.6 {
            config.health_high_color
        } else if health_percent > 0.3 {
            config.health_medium_color
        } else {
            config.health_low_color
        };

        draw_list
            .add_rect(
                [bar_x, health_y],
                [bar_x + bar_width, bar_y + box_height],
                health_color,
            )
            .filled(true)
            .build();
    }

    fn render_box_fill(
        draw_list: &imgui::DrawListMut,
        top_left: [f32; 2],
        bottom_right: [f32; 2],
        base_color: [f32; 4],
    ) {
        let top_color = [base_color[0], base_color[1], base_color[2], 0.0];
        let bottom_color = base_color;

        draw_list.add_rect_filled_multicolor(
            top_left,
            bottom_right,
            top_color,
            top_color,
            bottom_color,
            bottom_color,
        );
    }

    fn render_texts(
        draw_list: &imgui::DrawListMut,
        head_x: f32,
        head_y: f32,
        foot_y: f32,
        enemy: &crate::game::EnemyData,
        name_color: [f32; 4],
        config: &crate::config::CheatConfig,
        glow_intensity: f32,
    ) {
        let outline_color = [0.0, 0.0, 0.0, 1.0];

        // =========================
        // Nome com Glow - SEMPRE ATIVO
        // =========================
        if config.esp_name_enabled {
            let text = enemy.name.clone();
            let text_width = text.len() as f32 * 7.0;
            let text_height = 14.0;

            let x = head_x - text_width / 2.0;
            let y = head_y - 22.0;

            // Fundo
            draw_list
                .add_rect(
                    [x - 4.0, y - 2.0],
                    [x + text_width + 4.0, y + text_height + 2.0],
                    [0.0, 0.0, 0.0, 0.85],
                )
                .filled(true)
                .rounding(3.0)
                .build();

            // 🔥 GLOW do texto - SEMPRE ATIVO
            Self::render_text_glow(draw_list, &text, [x, y], name_color, glow_intensity);

            // Outline
            for ox in -1..=1 {
                for oy in -1..=1 {
                    if ox == 0 && oy == 0 {
                        continue;
                    }
                    draw_list.add_text([x + ox as f32, y + oy as f32], outline_color, &text);
                }
            }

            // Texto principal
            draw_list.add_text([x, y], name_color, &text);
        }

        // =========================
        // Distância com Glow - SEMPRE ATIVO
        // =========================
        if config.esp_distance_enabled {
            let text = format!("{:.0}m", enemy.distance / 10.0);
            let text_width = text.len() as f32 * 7.0;
            let text_height = 14.0;

            let x = head_x - text_width / 2.0;
            let y = foot_y + 5.0;

            // Fundo
            draw_list
                .add_rect(
                    [x - 4.0, y - 2.0],
                    [x + text_width + 4.0, y + text_height + 2.0],
                    [0.0, 0.0, 0.0, 0.85],
                )
                .filled(true)
                .rounding(3.0)
                .build();

            // 🔥 GLOW do texto de distância - SEMPRE ATIVO
            Self::render_text_glow(draw_list, &text, [x, y], name_color, glow_intensity);

            // Outline
            for ox in -1..=1 {
                for oy in -1..=1 {
                    if ox == 0 && oy == 0 {
                        continue;
                    }
                    draw_list.add_text([x + ox as f32, y + oy as f32], outline_color, &text);
                }
            }

            // Texto principal
            draw_list.add_text([x, y], name_color, &text);
        }
    }
}