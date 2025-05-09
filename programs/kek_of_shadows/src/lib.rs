use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext as GL, WebGlProgram, WebGlShader};
use serde::{Serialize, Deserialize};
use rand::prelude::*;
use js_sys::Array;

#[derive(Serialize, Deserialize, Clone)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Orc,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Profession {
    Warrior,
    Mage,
    Archer,
    Blacksmith,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Equipment {
    pub armor: Option<String>,
    pub helmet: Option<String>,
    pub amulet: Option<String>,
    pub gloves: Option<String>,
    pub ring: Option<String>,
    pub weapon: Option<String>,
    pub shield: Option<String>,
    pub legs: Option<String>,
    pub boots: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub item_id: u32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkillNode {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub skill: String,
    pub connections: Vec<u32>,
    pub unlocked: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub type_: String,
    pub slot: String,
    pub stats: std::collections::HashMap<String, u32>,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Monster {
    pub id: u32,
    pub name: String,
    pub level: u32,
    pub stats: std::collections::HashMap<String, u32>,
    pub exp_reward: u32,
    pub gold_reward: [u32; 2],
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<String>>,
    pub collisions: Vec<Vec<bool>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub wallet: String,
    pub name: String,
    pub level: u32,
    pub x: f32,
    pub y: f32,
    pub location: String,
    pub equipment: Equipment,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub race: Race,
    pub profession: Profession,
    pub level: u32,
    pub experience: u64,
    pub gold: u64,
    pub guild: Option<String>,
    pub skill_points: u32,
    pub strength: u32,
    pub endurance: u32,
    pub wisdom: u32,
    pub mystic: u32,
    pub agility: u32,
    pub accuracy: u32,
    pub intellect: u32,
    pub luck: u32,
    pub sword: u32,
    pub spear: u32,
    pub axe: u32,
    pub dagger: u32,
    pub bow: u32,
    pub shield_skill: u32,
    pub magic: u32,
    pub rune_magic: u32,
    pub magic_resistance: u32,
    pub healing: u32,
    pub mining: u32,
    pub fishing: u32,
    pub alchemy: u32,
    pub equipment: Equipment,
    pub inventory: Vec<InventoryItem>,
    pub skill_tree: Vec<SkillNode>,
    pub x: f32,
    pub y: f32,
    pub location: String,
}

#[wasm_bindgen]
pub struct Game {
    character: Option<Character>,
    rng: ThreadRng,
    other_players: Vec<Player>,
    items: Vec<Item>,
    monsters: Vec<Monster>,
    maps: std::collections::HashMap<String, Map>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            character: None,
            rng: thread_rng(),
            other_players: Vec::new(),
            items: Vec::new(),
            monsters: Vec::new(),
            maps: std::collections::HashMap::new(),
        }
    }

    pub fn load_game_data(&mut self, game_data: JsValue) -> Result<(), JsValue> {
        let data: serde_json::Value = game_data.into_serde().map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.items = serde_json::from_value(data["items"].clone()).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.monsters = serde_json::from_value(data["monsters"].clone()).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(())
    }

    pub fn load_map(&mut self, location: String, map_data: JsValue) -> Result<(), JsValue> {
        let map: Map = map_data.into_serde().map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.maps.insert(location, map);
        Ok(())
    }

    pub fn create_character(&mut self, name: String, race: String, profession: String, wallet: String) -> Result<(), JsValue> {
        let race = match race.as_str() {
            "Human" => Race::Human,
            "Elf" => Race::Elf,
            "Dwarf" => Race::Dwarf,
            "Orc" => Race::Orc,
            _ => return Err(JsValue::from_str("Invalid race")),
        };

        let profession = match profession.as_str() {
            "Warrior" => Profession::Warrior,
            "Mage" => Profession::Mage,
            "Archer" => Profession::Archer,
            "Blacksmith" => Profession::Blacksmith,
            _ => return Err(JsValue::from_str("Invalid profession")),
        };

        let (strength, endurance, wisdom, mystic, agility, accuracy, intellect, luck, equipment) = match (&race, &profession) {
            (Race::Human, Profession::Warrior) => (15, 15, 5, 5, 10, 10, 5, 5, Equipment {
                weapon: Some("Iron Sword".to_string()),
                shield: Some("Steel Shield".to_string()),
                ..Equipment::default()
            }),
            (Race::Elf, Profession::Mage) => (5, 5, 15, 15, 10, 5, 15, 5, Equipment {
                weapon: Some("Magic Staff".to_string()),
                ..Equipment::default()
            }),
            (Race::Orc, Profession::Archer) => (10, 10, 5, 5, 15, 15, 5, 5, Equipment {
                weapon: Some("Bow".to_string()),
                ..Equipment::default()
            }),
            (Race::Dwarf, Profession::Blacksmith) => (15, 15, 5, 5, 5, 5, 10, 10, Equipment {
                weapon: Some("Blacksmith Hammer".to_string()),
                ..Equipment::default()
            }),
            _ => (10, 10, 10, 10, 10, 10, 10, 10, Equipment::default()),
        };

        let skill_tree = vec![
            SkillNode { id: 0, x: 0.0, y: 0.0, skill: "+5 Strength".to_string(), connections: vec![1, 2], unlocked: true },
            SkillNode { id: 1, x: 50.0, y: 0.0, skill: "+5 Sword".to_string(), connections: vec![0, 3], unlocked: false },
            SkillNode { id: 2, x: 0.0, y: 50.0, skill: "+5 Endurance".to_string(), connections: vec![0, 3], unlocked: false },
            SkillNode { id: 3, x: 50.0, y: 50.0, skill: "+10 Health".to_string(), connections: vec![1, 2], unlocked: false },
        ];

        self.character = Some(Character {
            name,
            race,
            profession,
            level: 1,
            experience: 0,
            gold: 100,
            guild: None,
            skill_points: 1,
            strength,
            endurance,
            wisdom,
            mystic,
            agility,
            accuracy,
            intellect,
            luck,
            sword: if profession == Profession::Warrior { 10 } else { 0 },
            spear: 0,
            axe: 0,
            dagger: 0,
            bow: if profession == Profession::Archer { 10 } else { 0 },
            shield_skill: if profession == Profession::Warrior { 10 } else { 0 },
            magic: if profession == Profession::Mage { 10 } else { 0 },
            rune_magic: 0,
            magic_resistance: 0,
            healing: 0,
            mining: if profession == Profession::Blacksmith { 10 } else { 0 },
            fishing: 0,
            alchemy: 0,
            equipment,
            inventory: vec![InventoryItem { item_id: 1, quantity: 1 }, InventoryItem { item_id: 2, quantity: 1 }],
            skill_tree,
            x: 5.0,
            y: 5.0,
            location: "Town".to_string(),
        });

        self.other_players.push(Player {
            wallet,
            name: self.character.as_ref().unwrap().name.clone(),
            level: 1,
            x: 5.0,
            y: 5.0,
            location: "Town".to_string(),
            equipment,
        });

        Ok(())
    }

    pub fn add_to_inventory(&mut self, item_id: u32, quantity: u32) -> Result<(), JsValue> {
        let character = self.character.as_mut().ok_or_else(|| JsValue::from_str("No character created"))?;
        if let Some(item) = character.inventory.iter_mut().find(|i| i.item_id == item_id) {
            item.quantity += quantity;
        } else {
            character.inventory.push(InventoryItem { item_id, quantity });
        }
        Ok(())
    }

    pub fn remove_from_inventory(&mut self, item_id: u32, quantity: u32) -> Result<(), JsValue> {
        let character = self.character.as_mut().ok_or_else(|| JsValue::from_str("No character created"))?;
        if let Some(item) = character.inventory.iter_mut().find(|i| i.item_id == item_id) {
            if item.quantity >= quantity {
                item.quantity -= quantity;
                if item.quantity == 0 {
                    character.inventory.retain(|i| i.item_id != item_id);
                }
                Ok(())
            } else {
                Err(JsValue::from_str("Not enough items"))
            }
        } else {
            Err(JsValue::from_str("Item not in inventory"))
        }
    }

    pub fn update_position(&mut self, x: f32, y: f32, location: String) -> Result<(), JsValue> {
        let character = self.character.as_mut().ok_or_else(|| JsValue::from_str("No character created"))?;
        let map = self.maps.get(&location).ok_or_else(|| JsValue::from_str("Map not loaded"))?;

        let tile_x = x.floor() as usize;
        let tile_y = y.floor() as usize;
        if tile_x >= map.width as usize || tile_y >= map.height as usize || map.collisions[tile_y][tile_x] {
            return Err(JsValue::from_str("Invalid or impassable position"));
        }

        character.x = x;
        character.y = y;
        character.location = location;
        Ok(())
    }

    pub fn update_other_players(&mut self, players: JsValue) -> Result<(), JsValue> {
        let players: Vec<Player> = players.into_serde().map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.other_players = players;
        Ok(())
    }

    pub fn fight_monster(&mut self) -> Result<String, JsValue> {
        let character = self.character.as_mut().ok_or_else(|| JsValue::from_str("No character created"))?;
        if self.monsters.is_empty() {
            return Err(JsValue::from_str("No monsters loaded"));
        }

        let monster_idx = self.rng.gen_range(0..self.monsters.len());
        let monster = &self.monsters[monster_idx];
        let damage = (character.strength + character.accuracy) / 2;
        let health = *monster.stats.get("health").unwrap_or(&0);
        if damage >= health {
            let exp_gain = monster.exp_reward;
            let gold_gain = self.rng.gen_range(monster.gold_reward[0]..=monster.gold_reward[1]);
            let skill_gain = self.rng.gen_range(1..5);

            character.experience += exp_gain as u64;
            character.gold += gold_gain as u64;
            character.skill_points += 1;
            let item_drop = self.rng.gen_range(1..=4);
            self.add_to_inventory(item_drop, 1)?;

            let next_level_exp = (character.level * 100) as u64;
            if character.experience >= next_level_exp {
                character.level += 1;
                character.strength += 2;
                character.endurance += 2;
                character.agility += 2;
            }

            match character.profession {
                Profession::Warrior => character.sword += skill_gain,
                Profession::Mage => character.magic += skill_gain,
                Profession::Archer => character.bow += skill_gain,
                Profession::Blacksmith => character.mining += skill_gain,
            }

            Ok(format!("Defeated {} (Level {})! Gained {} EXP, {} gold, 1 skill point, and item {}.", monster.name, monster.level, exp_gain, gold_gain, item_drop))
        } else {
            Ok(format!("Attacked {} but it survived!", monster.name))
        }
    }

    pub fn fight_player(&mut self, target_wallet: String) -> Result<String, JsValue> {
        let character = self.character.as_mut().ok_or_else(|| JsValue::from_str("No character created"))?;
        if !is_pvp_zone(&character.location) {
            return Err(JsValue::from_str("Cannot fight in a non-PvP zone"));
        }

        let target = self.other_players.iter().find(|p| p.wallet == target_wallet).ok_or_else(|| JsValue::from_str("Player not found"))?;
        let damage = (character.strength + character.accuracy) / 2;
        let exp_gain = target.level * 5;
        character.experience += exp_gain as u64;
        character.gold += self.rng.gen_range(20..100) as u64;

        Ok(format!("Attacked {} (Level {})! Gained {} EXP and {} gold.", target.name, target.level, exp_gain, character.gold))
    }

    pub fn unlock_skill_node(&mut self, node_id: u32) -> Result<(), JsValue> {
        let character = self.character.as_mut().ok_or_else(|| JsValue::from_str("No character created"))?;
        if character.skill_points == 0 {
            return Err(JsValue::from_str("No skill points available"));
        }

        let node = character.skill_tree.iter_mut().find(|n| n.id == node_id).ok_or_else(|| JsValue::from_str("Node not found"))?;
        if node.unlocked {
            return Err(JsValue::from_str("Node already unlocked"));
        }

        let has_unlocked_neighbor = character.skill_tree.iter().any(|n| n.unlocked && n.connections.contains(&node_id));
        if !has_unlocked_neighbor && node_id != 0 {
            return Err(JsValue::from_str("Must unlock a connected node first"));
        }

        node.unlocked = true;
        character.skill_points -= 1;

        match node.skill.as_str() {
            "+5 Strength" => character.strength += 5,
            "+5 Sword" => character.sword += 5,
            "+5 Endurance" => character.endurance += 5,
            "+10 Health" => character.endurance += 10,
            _ => {}
        }

        Ok(())
    }

    pub fn equip_item(&mut self, item_id: u32) -> Result<(), JsValue> {
        let character = self.character.as_mut().ok_or_else(|| JsValue::from_str("No character created"))?;
        let item = self.items.iter().find(|i| i.id == item_id).ok_or_else(|| JsValue::from_str("Item not found"))?;

        self.remove_from_inventory(item_id, 1)?;

        match item.slot.as_str() {
            "armor" => character.equipment.armor = Some(item.name.clone()),
            "helmet" => character.equipment.helmet = Some(item.name.clone()),
            "amulet" => character.equipment.amulet = Some(item.name.clone()),
            "gloves" => character.equipment.gloves = Some(item.name.clone()),
            "ring" => character.equipment.ring = Some(item.name.clone()),
            "weapon" => character.equipment.weapon = Some(item.name.clone()),
            "shield" => character.equipment.shield = Some(item.name.clone()),
            "legs" => character.equipment.legs = Some(item.name.clone()),
            "boots" => character.equipment.boots = Some(item.name.clone()),
            _ => return Err(JsValue::from_str("Invalid equipment slot")),
        }

        for (stat, value) in &item.stats {
            match stat.as_str() {
                "strength" => character.strength += value,
                "endurance" => character.endurance += value,
                "accuracy" => character.accuracy += value,
                "magic" => character.magic += value,
                "shield_skill" => character.shield_skill += value,
                _ => {}
            }
        }

        Ok(())
    }

    pub fn get_character(&self) -> Result<JsValue, JsValue> {
        match &self.character {
            Some(character) => JsValue::from_serde(character).map_err(|e| JsValue::from_str(&e.to_string())),
            None => Err(JsValue::from_str("No character created")),
        }
    }

    pub fn get_inventory(&self) -> Result<JsValue, JsValue> {
        let character = self.character.as_ref().ok_or_else(|| JsValue::from_str("No character created"))?;
        JsValue::from_serde(&character.inventory).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn render_game(&self, canvas_id: String, show_skill_tree: bool) -> Result<(), JsValue> {
        let character = self.character.as_ref().ok_or_else(|| JsValue::from_str("No character created"))?;
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).ok_or_else(|| JsValue::from_str("Canvas not found"))?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| JsValue::from_str("Not a canvas"))?;
        let gl = canvas.get_context("webgl")?.ok_or_else(|| JsValue::from_str("WebGL not supported"))?.dyn_into::<GL>()?;

        if show_skill_tree {
            return self.render_skill_tree(canvas_id);
        }

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).ok_or_else(|| JsValue::from_str("Unable to create shader"))?;
        gl.shader_source(&vert_shader, r#"
            attribute vec2 a_position;
            uniform vec2 u_resolution;
            void main() {
                vec2 iso_pos = vec2(a_position.x - a_position.y, (a_position.x + a_position.y) * 0.5);
                vec2 normalized = iso_pos / u_resolution * 2.0 - 1.0;
                gl_Position = vec4(normalized, 0.0, 1.0);
                gl_PointSize = 20.0;
            }
        "#);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).ok_or_else(|| JsValue::from_str("Unable to create shader"))?;
        gl.shader_source(&frag_shader, r#"
            precision mediump float;
            uniform vec4 u_color;
            void main() {
                gl_FragColor = u_color;
            }
        "#);
        gl.compile_shader(&frag_shader);

        let program = gl.create_program().ok_or_else(|| JsValue::from_str("Unable to create program"))?;
        gl.attach_shader(&program, &vert_shader);
        gl.attach_shader(&program, &frag_shader);
        gl.link_program(&program);
        gl.use_program(Some(&program));

        let resolution_location = gl.get_uniform_location(&program, "u_resolution").ok_or_else(|| JsValue::from_str("Uniform not found"))?;
        gl.uniform2f(Some(&resolution_location), canvas.width() as f32, canvas.height() as f32);

        let map = self.maps.get(&character.location).ok_or_else(|| JsValue::from_str("Map not loaded"))?;
        let position_location = gl.get_attrib_location(&program, "a_position") as u32;
        let color_location = gl.get_uniform_location(&program, "u_color").ok_or_else(|| JsValue::from_str("Uniform not found"))?;
        let position_buffer = gl.create_buffer().ok_or_else(|| JsValue::from_str("Buffer creation failed"))?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        gl.enable_vertex_attrib_array(position_location);
        gl.vertex_attrib_pointer_with_i32(position_location, 2, GL::FLOAT, false, 0, 0);

        let tile_size = 32.0;
        for y in 0..map.height as usize {
            for x in 0..map.width as usize {
                let tile = &map.tiles[y][x];
                let color = match tile.as_str() {
                    "grass" => [0.0, 1.0, 0.0, 1.0],
                    "stone" => [0.5, 0.5, 0.5, 1.0],
                    "water" => [0.0, 0.0, 1.0, 1.0],
                    _ => [1.0, 1.0, 1.0, 1.0],
                };
                gl.uniform4fv_with_f32_array(Some(&color_location), &color);
                let cx = (x as f32 + 0.5) * tile_size;
                let cy = (y as f32 + 0.5) * tile_size;
                let positions = [
                    cx - tile_size / 2.0, cy,
                    cx, cy - tile_size / 4.0,
                    cx + tile_size / 2.0, cy,
                    cx, cy + tile_size / 4.0,
                ];
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_sys::Float32Array::from(positions.as_ref()).into(), GL::STATIC_DRAW);
                gl.draw_arrays(GL::TRIANGLE_FAN, 0, 4);
            }
        }

        for player in &self.other_players {
            if player.location == character.location {
                gl.uniform4fv_with_f32_array(Some(&color_location), &[1.0, 0.0, 0.0, 1.0]);
                let cx = player.x * tile_size;
                let cy = player.y * tile_size;
                let positions = [
                    cx - tile_size / 4.0, cy,
                    cx, cy - tile_size / 8.0,
                    cx + tile_size / 4.0, cy,
                    cx, cy + tile_size / 8.0,
                ];
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_sys::Float32Array::from(positions.as_ref()).into(), GL::STATIC_DRAW);
                gl.draw_arrays(GL::TRIANGLE_FAN, 0, 4);
            }
        }

        gl.uniform4fv_with_f32_array(Some(&color_location), &[0.0, 1.0, 0.0, 1.0]);
        let cx = character.x * tile_size;
        let cy = character.y * tile_size;
        let positions = [
            cx - tile_size / 4.0, cy,
            cx, cy - tile_size / 8.0,
            cx + tile_size / 4.0, cy,
            cx, cy + tile_size / 8.0,
        ];
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_sys::Float32Array::from(positions.as_ref()).into(), GL::STATIC_DRAW);
        gl.draw_arrays(GL::TRIANGLE_FAN, 0, 4);

        Ok(())
    }

    pub fn render_skill_tree(&self, canvas_id: String) -> Result<(), JsValue> {
        let character = self.character.as_ref().ok_or_else(|| JsValue::from_str("No character created"))?;
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).ok_or_else(|| JsValue::from_str("Canvas not found"))?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| JsValue::from_str("Not a canvas"))?;
        let gl = canvas.get_context("webgl")?.ok_or_else(|| JsValue::from_str("WebGL not supported"))?.dyn_into::<GL>()?;

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).ok_or_else(|| JsValue::from_str("Unable to create shader"))?;
        gl.shader_source(&vert_shader, r#"
            attribute vec2 a_position;
            uniform vec2 u_resolution;
            void main() {
                vec2 normalized = a_position / u_resolution * 2.0 - 1.0;
                gl_Position = vec4(normalized, 0.0, 1.0);
                gl_PointSize = 20.0;
            }
        "#);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).ok_or_else(|| JsValue::from_str("Unable to create shader"))?;
        gl.shader_source(&frag_shader, r#"
            precision mediump float;
            uniform vec4 u_color;
            void main() {
                gl_FragColor = u_color;
            }
        "#);
        gl.compile_shader(&frag_shader);

        let program = gl.create_program().ok_or_else(|| JsValue::from_str("Unable to create program"))?;
        gl.attach_shader(&program, &vert_shader);
        gl.attach_shader(&program, &frag_shader);
        gl.link_program(&program);
        gl.use_program(Some(&program));

        let resolution_location = gl.get_uniform_location(&program, "u_resolution").ok_or_else(|| JsValue::from_str("Uniform not found"))?;
        gl.uniform2f(Some(&resolution_location), canvas.width() as f32, canvas.height() as f32);

        let position_location = gl.get_attrib_location(&program, "a_position") as u32;
        let color_location = gl.get_uniform_location(&program, "u_color").ok_or_else(|| JsValue::from_str("Uniform not found"))?;
        let position_buffer = gl.create_buffer().ok_or_else(|| JsValue::from_str("Buffer creation failed"))?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        gl.enable_vertex_attrib_array(position_location);
        gl.vertex_attrib_pointer_with_i32(position_location, 2, GL::FLOAT, false, 0, 0);

        for node in &character.skill_tree {
            let color = if node.unlocked { [1.0, 1.0, 0.0, 1.0] } else { [0.5, 0.5, 0.5, 1.0] };
            gl.uniform4fv_with_f32_array(Some(&color_location), &color);
            let positions = [node.x + 300.0, node.y + 200.0];
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_sys::Float32Array::from(positions.as_ref()).into(), GL::STATIC_DRAW);
            gl.draw_arrays(GL::POINTS, 0, 1);
        }

        for node in &character.skill_tree {
            for &conn_id in &node.connections {
                let other_node = character.skill_tree.iter().find(|n| n.id == conn_id).unwrap();
                let positions = [
                    node.x + 300.0, node.y + 200.0,
                    other_node.x + 300.0, other_node.y + 200.0,
                ];
                gl.uniform4fv_with_f32_array(Some(&color_location), &[0.7, 0.7, 0.7, 1.0]);
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &js_sys::Float32Array::from(positions.as_ref()).into(), GL::STATIC_DRAW);
                gl.draw_arrays(GL::LINES, 0, 2);
            }
        }

        Ok(())
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment {
            armor: None,
            helmet: None,
            amulet: None,
            gloves: None,
            ring: None,
            weapon: None,
            shield: None,
            legs: None,
            boots: None,
        }
    }
}

fn is_pvp_zone(location: &str) -> bool {
    match location {
        "Town" | "Temple" | "Building" => false,
        "Wilderness" | "HuntingGround" | "BossArea" | "QuestArea" | "Castle" | "Island" => true,
        _ => true,
    }
}
