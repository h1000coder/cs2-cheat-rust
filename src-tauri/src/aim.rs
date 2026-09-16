use crate::game::GameData;
use crate::math::ViewMatrix;
use crate::memory::GameProcess;
use crate::offsets::Offsets;
use crate::{config::CONFIG, entity::Entity};
use std::sync::MutexGuard;
use std::{mem, thread};
use std::time::Duration;
use winapi::um::winuser::{
    GetAsyncKeyState, INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, 
    MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput, mouse_event
};

pub fn move_mouse(x: i32, y: i32) {
    unsafe {
        let mut input = INPUT {
            type_: INPUT_MOUSE,
            u: mem::zeroed(),
        };
        *input.u.mi_mut() = MOUSEINPUT {
            dx: x,
            dy: y,
            mouseData: 0,
            dwFlags: MOUSEEVENTF_MOVE,
            time: 0,
            dwExtraInfo: 0,
        };
        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}


fn is_key_pressed(key_code: i32) -> bool {
    unsafe {
        let state = GetAsyncKeyState(key_code);
        (state & 0x8000u16 as i16) != 0
    }
}

pub struct Aimbot;

impl Aimbot {
    pub fn run(
        game_process: &mut GameProcess,
        entities: &[crate::entity::Entity],
        local_team: i32,
        view_matrix: &ViewMatrix,
        screen_size: (f32, f32),
    ) {
        let config = CONFIG.lock().unwrap();
        if !config.aimbot_enabled { 
            return; 
        }

        
        if !is_key_pressed(config.aimbot_key) {
            return;
        }

        let mut closest_dist = f32::MAX;
        let mut target_screen_pos: Option<(f32, f32)> = None;

        for entity in entities {
            if !entity.is_alive() || entity.team == local_team { 
                continue; 
            }

            let head_pos = match entity.get_head_position(game_process) {
                Some(pos) => pos,
                None => continue,
            };
            
            if let Some(screen_pos) = view_matrix.world_to_screen(head_pos, screen_size) {
                let center_x = screen_size.0 / 2.0;
                let center_y = screen_size.1 / 2.0;
                let dx = screen_pos.0 - center_x;
                let dy = screen_pos.1 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < closest_dist && dist < config.fov_size {
                    closest_dist = dist;
                    target_screen_pos = Some(screen_pos);
                }
            }
        }

        if let Some((tx, ty)) = target_screen_pos {
            let center_x = screen_size.0 / 2.0;
            let center_y = screen_size.1 / 2.0;
            
            let mut dx = tx - center_x;
            let mut dy = ty - center_y;
            
            if config.aimbot_smooth > 0.0 {
                dx /= config.aimbot_smooth;
                dy /= config.aimbot_smooth;
            }
            
            move_mouse(dx as i32, dy as i32);
        }
    }
}

pub struct TriggerBot;

impl TriggerBot {
    pub fn run(
        game_data: &MutexGuard<'_, GameData>,
        game_process: &mut GameProcess,
        client_base: usize,
    ) {
        let config = CONFIG.lock().unwrap();
        if !config.trigger_bot_enabled {
            return;
        }

        
        let key_state = unsafe { GetAsyncKeyState(config.trigger_key as i32) };
        let is_key_pressed = (key_state & 0x8000u16 as i16) != 0;

        if !is_key_pressed && config.trigger_hold_mode {
            return;
        }

        let local_pawn = match game_process.read_ptr(client_base + Offsets::dwLocalPlayerPawn) {
            Some(pawn) => pawn,
            None => {
                println!("❌ TriggerBot: Não conseguiu ler local_pawn");
                return;
            }
        };

        let entity_index = match game_process.read_i32(local_pawn + Offsets::m_iIDEntIndex) {
            Some(idx) => idx,
            None => {
                println!("❌ TriggerBot: Não conseguiu ler m_iIDEntIndex");
                return;
            }
        };

        println!("🎯 TriggerBot: Entity index = {}", entity_index);

        
        if entity_index <= 0 {
            println!(
                "📌 TriggerBot: Nenhum jogador na mira (index={})",
                entity_index
            );
            return;
        }

        let entity_list = match game_process.read_ptr(client_base + Offsets::dwEntityList) {
            Some(list) => list,
            None => return,
        };

        let entry_index = (entity_index & 0x7FFF) >> 9;
        let entry_ptr = entity_list + (8 * entry_index as usize) + 16;

        let list_entry = match game_process.read_ptr(entry_ptr) {
            Some(entry) => entry,
            None => return,
        };

        let controller =
            match game_process.read_ptr(list_entry + (112 * (entity_index & 0x1FF) as usize)) {
                Some(ctrl) => ctrl,
                None => return,
            };

        if controller == 0 {
            println!(
                "❌ TriggerBot: Controller é 0 para entity index {}",
                entity_index
            );
            return;
        }

        
        let entity_team = match game_process.read_i32(controller + Offsets::m_iTeamNum) {
            Some(team) => team,
            None => {
                println!("❌ TriggerBot: Não conseguiu ler team do controller");
                return;
            }
        };

        println!("🎯 Team do inimigo: {}", entity_team);
        println!("🎯 Seu time: {}", game_data.team);

        let pawn_handle = match game_process.read_i32(controller + Offsets::m_hPawn) {
            Some(handle) => handle,
            None => return,
        };

        let pawn_index = (pawn_handle & 0x7FFF) >> 9;
        let pawn_entry_ptr = entity_list + (8 * pawn_index as usize) + 16;

        let pawn_list_entry = match game_process.read_ptr(pawn_entry_ptr) {
            Some(entry) => entry,
            None => return,
        };

        let pawn =
            match game_process.read_ptr(pawn_list_entry + (112 * (pawn_handle & 0x1FF) as usize)) {
                Some(pawn) => pawn,
                None => return,
            };

        if pawn == 0 {
            println!("❌ TriggerBot: Pawn é 0 para entity index {}", entity_index);
            return;
        }

        let health = game_process
            .read_i32(pawn + Offsets::m_iHealth)
            .unwrap_or(0);

        println!(
            "🎯 TriggerBot: Alvo na mira - Health: {}, Team: {}",
            health, entity_team
        );

        // Verifica se é inimigo e está vivo
        if entity_team != 0 && entity_team != game_data.team {
            println!("✅ TriggerBot: Atirando no inimigo!");

            if config.trigger_delay_ms > 0 {
                thread::sleep(Duration::from_millis(config.trigger_delay_ms));
            }

            Self::shoot();
            thread::sleep(Duration::from_millis(config.trigger_scan_delay_ms));
        } else {
            println!(
                "❌ TriggerBot: Não atirou - Team: {}, Seu team: {}, Health: {}",
                entity_team, game_data.team, health
            );
        }
    }

    fn shoot() {
        unsafe {
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            thread::sleep(Duration::from_millis(1));
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        }
    }
}
