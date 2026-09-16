mod aim;
mod config;
mod entity;
mod esp;
mod game;
mod math;
mod memory;
mod offsets;

use esp::EspRenderer;
use game::GameData;
use glam::Vec3;
use memory::GameProcess;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::aim::{Aimbot, TriggerBot};
use crate::config::{CheatConfig, CONFIG};
use crate::entity::Entity;
use crate::esp::get_screen_resolution;
use crate::game::EnemyData;
use crate::offsets::Offsets;


// ========== COMANDOS TAURI ==========

#[tauri::command]
fn get_cheat_config() -> CheatConfig {
    CONFIG.lock().unwrap().clone()
}

#[tauri::command]
fn update_cheat_config(config: CheatConfig) {
    let mut current = CONFIG.lock().unwrap();
    *current = config;
    println!("Nova config aplicada: {:?}", current.skeleton_thickness);
}

#[tauri::command]
fn reset_cheat_config() {
    let mut current = CONFIG.lock().unwrap();
    *current = CheatConfig::default();
}

#[tauri::command]
fn get_game_data(state: tauri::State<'_, Arc<Mutex<GameData>>>) -> (i32, i32, Vec<(String, f32)>) {
    let data = state.lock().unwrap();
    let enemies: Vec<(String, f32)> = data
        .enemies
        .iter()
        .map(|e| (e.name.clone(), e.distance))
        .collect();
    (data.health, data.team, enemies)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let game_data = Arc::new(Mutex::new(GameData::default()));
    let game_data_clone = game_data.clone();
    let game_data_for_state = game_data.clone();

    // Thread de Leitura de Memória (Background)
    thread::spawn(move || loop {
        if let Some(mut game) = GameProcess::find() {
            if let Some(client_base) = game.get_module_base("client.dll") {
                run_game_loop(&mut game, client_base, &game_data_clone);
            }
        }
        thread::sleep(Duration::from_secs(2));
    });

    // Thread de Overlay ESP
    let data_for_overlay = game_data.clone();
    thread::spawn(move || {
        let mut app =
            imgui_rs_overlay::window::Windows::new(&imgui_rs_overlay::window::WindowsOptions {
                title: "CS2 ESP Overlay".to_string(),
                overlay_target: imgui_rs_overlay::OverlayTarget::WindowTitle(
                    "Counter-Strike 2".to_string(),
                ),
                ..Default::default()
            })
            .expect("Falha ao criar overlay");

        app.run(move |ui, _style| {
            let data = data_for_overlay.lock().unwrap();
            EspRenderer::render(ui, &data);
            true
        })
        .unwrap();
    });

    // Inicializa Tauri com Global Shortcut
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(game_data_for_state)
        .invoke_handler(tauri::generate_handler![
            get_cheat_config,
            update_cheat_config,
            reset_cheat_config,
            get_game_data,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();

            let shortcut = Shortcut::new(None, Code::Insert);

            app.global_shortcut()
                .on_shortcut(shortcut, move |_app, _shortcut, event| {
                    
                    if event.state() == ShortcutState::Pressed {
                        println!("🎯 INSERT pressed globally");

                        if let Ok(is_visible) = window_clone.is_visible() {
                            if is_visible {
                                let _ = window_clone.hide();
                                println!("Window hidden");
                            } else {
                                let _ = window_clone.show();
                                let _ = window_clone.set_always_on_top(true);
                                let _ = window_clone.set_focus();
                                println!("Window shown and set on top");
                            }
                        }
                    }
                })
                .unwrap();

            // Configurar janela para sempre no topo
            let _ = window.set_always_on_top(true);
            println!("✅ Window always on top enabled");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_game_loop(game: &mut GameProcess, client_base: usize, game_data: &Arc<Mutex<GameData>>) {
    let mut local_team = 0;
    let mut local_pawn_addr = 0;

    loop {
        if !game.is_running() {
            break;
        }

        let local_pawn = match game.read_ptr(client_base + Offsets::dwLocalPlayerPawn) {
            Some(pawn) => {
                pawn
            },
            None => {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
        };

        if local_pawn != local_pawn_addr {
            local_pawn_addr = local_pawn;
        }

        let health = game.read_i32(local_pawn + Offsets::m_iHealth).unwrap_or(0);
        let team = game.read_i32(local_pawn + Offsets::m_iTeamNum).unwrap_or(0);
        local_team = team;
        let local_pos = game
            .read_vec3(local_pawn + Offsets::m_vOldOrigin)
            .unwrap_or(Vec3::ZERO);

        let view_matrix = game.read_view_matrix(client_base);
        let local_controller_handle = game
            .read_i32(local_pawn + Offsets::m_hController)
            .unwrap_or(0);

        let entity_list = game
            .read_ptr(client_base + Offsets::dwEntityList)
            .unwrap_or(0);
        let controller_index = (local_controller_handle & 0x7FFF) >> 9;
        let controller_entry = game
            .read_ptr(entity_list + (8 * (controller_index as usize) + 16))
            .unwrap_or(0);
        let local_controller_ptr = game
            .read_ptr(controller_entry + (112 * (local_controller_handle & 0x1FF) as usize))
            .unwrap_or(0);

        let enemies = collect_enemies(
            game,
            client_base,
            local_pawn,
            local_team,
            local_pos,
            local_controller_ptr,
        );

        {
            let mut data = game_data.lock().unwrap();
            data.local_controller = local_controller_ptr;
            data.health = health;
            data.team = local_team;
            data.local_pos = local_pos;
            data.enemies = enemies;
            data.view_matrix = view_matrix.map(|vm| vm.matrix);
        }

        let data_clone = game_data.lock().unwrap();

        let enemy_entities: Vec<Entity> = data_clone
            .enemies
            .iter()
            .map(|e| {
                let mut entity = Entity::new(e.index);
                entity.name = e.name.clone();
                entity.health = e.health;
                entity.team = e.team;
                entity.position = e.position;
                entity.head_position = e.head_position;
                entity.dormant = e.dormant;
                entity.pawn = e.pawn;
                entity.controller = e.controller;
                entity
            })
            .collect();

        TriggerBot::run(&data_clone, game, client_base);

        let screen_size = get_screen_resolution();

        if let Some(vm) = view_matrix {
            Aimbot::run(game, &enemy_entities, local_team, &vm, screen_size);
        }

        drop(data_clone);
        thread::sleep(Duration::from_millis(7));
    }
}

fn collect_enemies(
    game: &mut GameProcess,
    client_base: usize,
    local_pawn: usize,
    local_team: i32,
    local_pos: Vec3,
    local_controller: usize,
) -> Vec<game::EnemyData> {
    let mut enemies = Vec::new();
    let config = CONFIG.lock().unwrap();

    for i in 1..65 {
        let mut entity = Entity::new(i);
        if !entity.update(game, client_base, local_controller) {
            continue;
        }

        if entity.pawn == local_pawn {
            continue;
        }

        let distance = local_pos.distance(entity.position);

        if entity.team != local_team && local_team != 0 && (entity.team == 2 || entity.team == 3) {
            let mut enemy_data = game::EnemyData::new(&entity, distance);

            if config.esp_skeleton_enabled {
                enemy_data.bone_positions = entity.get_bone_positions(game);
            }

            enemies.push(enemy_data);
        }
    }

    enemies.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    enemies
}
