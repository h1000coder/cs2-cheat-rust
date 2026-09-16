use glam::Vec3;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
pub struct GameData {
    pub local_controller: usize,
    pub health: i32,
    pub team: i32,
    pub local_pos: Vec3,
    pub enemies: Vec<EnemyData>,
    pub view_matrix: Option<[[f32; 4]; 4]>,
    pub screen_width: f32,
    pub screen_height: f32,
}

#[derive(Clone)]
pub struct EnemyData {
    pub index: i32,           // Adicionado
    pub controller: usize,    // Adicionado
    pub pawn: usize,          // Adicionado
    pub name: String,
    pub health: i32,
    pub team: i32,
    pub position: Vec3,
    pub head_position: Vec3,
    pub bone_positions: HashMap<usize, Vec3>,
    pub distance: f32,
    pub dormant: bool,        // Adicionado
}

impl EnemyData {
    pub fn new(entity: &crate::entity::Entity, distance: f32) -> Self {
        Self {
            index: entity.index,           // Adicionado
            controller: entity.controller, // Adicionado
            pawn: entity.pawn,             // Adicionado
            name: entity.name.clone(),
            health: entity.health,
            team: entity.team,
            position: entity.position,
            head_position: entity.head_position,
            bone_positions: HashMap::new(),
            distance,
            dormant: entity.dormant,       // Adicionado
        }
    }
}