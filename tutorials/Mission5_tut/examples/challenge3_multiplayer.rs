//! # Challenge 3: Multiplayer Game State Manager
//!
//! **Objective**: Design a comprehensive multiplayer game state management system
//!
//! **Key Features**:
//! - Player position tracking with HashMap<PlayerId, Position>
//! - Room membership management with HashMap<RoomId, HashSet<PlayerId>>
//! - Memoized pathfinding between rooms for performance
//! - Real-time proximity detection for player interactions
//! - Event system for game state changes
//!
//! **Hash Data Structures Used**:
//! - HashMap for key-value associations (players->positions, rooms->members)
//! - HashSet for unique collections (room membership, visited locations)
//! - Caching with HashMap for performance optimization

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

pub type PlayerId = String;
pub type RoomId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance_to(&self, other: &Position) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    pub fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x + 1, self.y),
            Position::new(self.x - 1, self.y),
            Position::new(self.x, self.y + 1),
            Position::new(self.x, self.y - 1),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub position: Position,
    pub current_room: Option<RoomId>,
    pub health: u32,
    pub last_move: Instant,
}

impl Player {
    pub fn new(id: PlayerId, name: String, position: Position) -> Self {
        Player {
            id,
            name,
            position,
            current_room: None,
            health: 100,
            last_move: Instant::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub bounds: (Position, Position), // (top_left, bottom_right)
    pub capacity: usize,
    pub room_type: RoomType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoomType {
    Lobby,
    GameRoom,
    PrivateRoom,
    BattleArena,
}

#[derive(Debug, Clone)]
pub struct GameEvent {
    pub event_type: GameEventType,
    pub player_id: PlayerId,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub enum GameEventType {
    PlayerJoined {
        room_id: RoomId,
    },
    PlayerLeft {
        room_id: RoomId,
    },
    PlayerMoved {
        from: Position,
        to: Position,
    },
    PlayerProximity {
        other_player: PlayerId,
        distance: f32,
    },
    RoomFull {
        room_id: RoomId,
    },
}

/// Comprehensive multiplayer game state manager
#[derive(Debug)]
pub struct MultiplayerGameManager {
    // Core player and room data
    players: HashMap<PlayerId, Player>,
    rooms: HashMap<RoomId, Room>,

    // Spatial relationships
    player_positions: HashMap<PlayerId, Position>,
    room_memberships: HashMap<RoomId, HashSet<PlayerId>>,
    player_room_lookup: HashMap<PlayerId, RoomId>,

    // Performance optimization caches
    pathfinding_cache: HashMap<(RoomId, RoomId), Option<Vec<RoomId>>>,
    proximity_cache: HashMap<(PlayerId, PlayerId), (f32, Instant)>,

    // Spatial indexing for fast proximity queries
    position_grid: HashMap<(i32, i32), HashSet<PlayerId>>, // Grid-based spatial index

    // Event system
    recent_events: VecDeque<GameEvent>,
    max_event_history: usize,

    // Statistics
    stats: GameStats,
}

#[derive(Debug, Default, Clone)]
pub struct GameStats {
    pub total_players_joined: u32,
    pub total_moves: u32,
    pub total_room_changes: u32,
    pub cache_hits: u32,
    pub cache_misses: u32,
    pub proximity_checks: u32,
}

impl Default for MultiplayerGameManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiplayerGameManager {
    pub fn new() -> Self {
        MultiplayerGameManager {
            players: HashMap::new(),
            rooms: HashMap::new(),
            player_positions: HashMap::new(),
            room_memberships: HashMap::new(),
            player_room_lookup: HashMap::new(),
            pathfinding_cache: HashMap::new(),
            proximity_cache: HashMap::new(),
            position_grid: HashMap::new(),
            recent_events: VecDeque::new(),
            max_event_history: 100,
            stats: GameStats::default(),
        }
    }

    /// Add a new room to the game world
    pub fn add_room(&mut self, room: Room) {
        println!("🏠 Adding room: {} ({})", room.name, room.id);
        self.room_memberships
            .insert(room.id.clone(), HashSet::new());
        self.rooms.insert(room.id.clone(), room);

        // Clear pathfinding cache when topology changes
        self.pathfinding_cache.clear();
    }

    /// Add a player to the game
    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        let player_id = player.id.clone();
        let position = player.position;

        if self.players.contains_key(&player_id) {
            return Err(format!("Player {} already exists", player_id));
        }

        println!("👤 Player {} joined at {:?}", player.name, position);

        // Add to core collections
        self.players.insert(player_id.clone(), player);
        self.player_positions.insert(player_id.clone(), position);

        // Update spatial index
        let grid_pos = self.position_to_grid(position);
        self.position_grid
            .entry(grid_pos)
            .or_default()
            .insert(player_id.clone());

        // Try to assign to appropriate room
        if let Some(room_id) = self.find_room_for_position(position) {
            self.assign_player_to_room(player_id.clone(), room_id)?;
        }

        self.stats.total_players_joined += 1;

        Ok(())
    }

    /// Move a player to a new position
    pub fn move_player(
        &mut self,
        player_id: &PlayerId,
        new_position: Position,
    ) -> Result<(), String> {
        let old_position = *self
            .player_positions
            .get(player_id)
            .ok_or_else(|| format!("Player {} not found", player_id))?;

        // Update position
        self.player_positions
            .insert(player_id.clone(), new_position);

        // Update player struct
        if let Some(player) = self.players.get_mut(player_id) {
            player.position = new_position;
            player.last_move = Instant::now();
        }

        // Update spatial index
        let old_grid_pos = self.position_to_grid(old_position);
        let new_grid_pos = self.position_to_grid(new_position);

        if old_grid_pos != new_grid_pos {
            if let Some(old_grid) = self.position_grid.get_mut(&old_grid_pos) {
                old_grid.remove(player_id);
            }
            self.position_grid
                .entry(new_grid_pos)
                .or_default()
                .insert(player_id.clone());
        }

        // Check if player moved to a different room
        let new_room = self.find_room_for_position(new_position);
        let current_room = self.player_room_lookup.get(player_id).cloned();

        if new_room != current_room {
            // Remove from current room
            if let Some(old_room) = current_room {
                self.remove_player_from_room(player_id, &old_room);
            }

            // Add to new room
            if let Some(new_room_id) = new_room {
                self.assign_player_to_room(player_id.clone(), new_room_id)?;
            }
        }

        // Add move event
        self.add_event(GameEvent {
            event_type: GameEventType::PlayerMoved {
                from: old_position,
                to: new_position,
            },
            player_id: player_id.clone(),
            timestamp: Instant::now(),
        });

        // Clear proximity cache for this player (positions changed)
        self.proximity_cache
            .retain(|(p1, p2), _| p1 != player_id && p2 != player_id);

        // Check for nearby players
        self.check_player_proximity(player_id);

        self.stats.total_moves += 1;

        println!(
            "🚶 Player {} moved from {:?} to {:?}",
            player_id, old_position, new_position
        );

        Ok(())
    }

    /// Assign a player to a specific room
    pub fn assign_player_to_room(
        &mut self,
        player_id: PlayerId,
        room_id: RoomId,
    ) -> Result<(), String> {
        let room = self
            .rooms
            .get(&room_id)
            .ok_or_else(|| format!("Room {} not found", room_id))?;

        let current_members = self.room_memberships.get(&room_id).unwrap().len();
        if current_members >= room.capacity {
            self.add_event(GameEvent {
                event_type: GameEventType::RoomFull {
                    room_id: room_id.clone(),
                },
                player_id: player_id.clone(),
                timestamp: Instant::now(),
            });
            return Err(format!("Room {} is at capacity", room_id));
        }

        // Add to room membership
        self.room_memberships
            .get_mut(&room_id)
            .unwrap()
            .insert(player_id.clone());
        self.player_room_lookup
            .insert(player_id.clone(), room_id.clone());

        // Update player's current room
        if let Some(player) = self.players.get_mut(&player_id) {
            player.current_room = Some(room_id.clone());
        }

        self.add_event(GameEvent {
            event_type: GameEventType::PlayerJoined {
                room_id: room_id.clone(),
            },
            player_id: player_id.clone(),
            timestamp: Instant::now(),
        });

        self.stats.total_room_changes += 1;

        println!("🏠 Player {} joined room {}", player_id, room_id);

        Ok(())
    }

    /// Remove a player from a room
    fn remove_player_from_room(&mut self, player_id: &PlayerId, room_id: &RoomId) {
        if let Some(members) = self.room_memberships.get_mut(room_id) {
            members.remove(player_id);
        }
        self.player_room_lookup.remove(player_id);

        if let Some(player) = self.players.get_mut(player_id) {
            player.current_room = None;
        }

        self.add_event(GameEvent {
            event_type: GameEventType::PlayerLeft {
                room_id: room_id.clone(),
            },
            player_id: player_id.clone(),
            timestamp: Instant::now(),
        });

        println!("🚪 Player {} left room {}", player_id, room_id);
    }

    /// Find players within a given radius of a position
    pub fn find_players_in_radius(
        &mut self,
        center: Position,
        radius: f32,
    ) -> Vec<(PlayerId, Position, f32)> {
        let mut nearby_players = Vec::new();

        // Use spatial grid to limit search space
        let grid_radius = (radius / 10.0).ceil() as i32; // Assuming grid size of 10
        let center_grid = self.position_to_grid(center);

        for dx in -grid_radius..=grid_radius {
            for dy in -grid_radius..=grid_radius {
                let grid_pos = (center_grid.0 + dx, center_grid.1 + dy);
                if let Some(players_in_grid) = self.position_grid.get(&grid_pos) {
                    for player_id in players_in_grid {
                        if let Some(&position) = self.player_positions.get(player_id) {
                            let distance = center.distance_to(&position);
                            if distance <= radius {
                                nearby_players.push((player_id.clone(), position, distance));
                            }
                        }
                    }
                }
            }
        }

        nearby_players.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        nearby_players
    }

    /// Memoized pathfinding between rooms
    pub fn find_path_between_rooms(
        &mut self,
        from_room: &RoomId,
        to_room: &RoomId,
    ) -> Option<Vec<RoomId>> {
        let cache_key = (from_room.clone(), to_room.clone());

        // Check cache first
        if let Some(cached_path) = self.pathfinding_cache.get(&cache_key) {
            self.stats.cache_hits += 1;
            println!("🎯 Cache HIT: Path from {} to {}", from_room, to_room);
            return cached_path.clone();
        }

        self.stats.cache_misses += 1;
        println!(
            "🔄 Cache MISS: Computing path from {} to {}",
            from_room, to_room
        );

        // Simple BFS pathfinding (in a real game, rooms would have connections)
        let path = self.bfs_room_pathfinding(from_room, to_room);

        // Cache the result
        self.pathfinding_cache.insert(cache_key, path.clone());

        path
    }

    /// Get all players in a specific room
    pub fn get_players_in_room(&self, room_id: &RoomId) -> Option<&HashSet<PlayerId>> {
        self.room_memberships.get(room_id)
    }

    /// Get room occupancy statistics
    pub fn get_room_stats(&self) -> Vec<(RoomId, usize, usize)> {
        self.rooms
            .iter()
            .map(|(room_id, room)| {
                let current_occupancy = self
                    .room_memberships
                    .get(room_id)
                    .map(|members| members.len())
                    .unwrap_or(0);
                (room_id.clone(), current_occupancy, room.capacity)
            })
            .collect()
    }

    /// Real-time proximity detection
    fn check_player_proximity(&mut self, player_id: &PlayerId) {
        let player_pos = if let Some(&pos) = self.player_positions.get(player_id) {
            pos
        } else {
            return;
        };

        let nearby_players = self.find_players_in_radius(player_pos, 50.0); // 50 unit proximity

        for (other_player_id, _other_pos, distance) in nearby_players {
            if other_player_id != *player_id {
                // Check cache for recent proximity check
                let cache_key = if player_id < &other_player_id {
                    (player_id.clone(), other_player_id.clone())
                } else {
                    (other_player_id.clone(), player_id.clone())
                };

                let should_notify = if let Some((cached_distance, cached_time)) =
                    self.proximity_cache.get(&cache_key)
                {
                    // Only notify if distance changed significantly or enough time passed
                    (distance - cached_distance).abs() > 5.0
                        || cached_time.elapsed() > Duration::from_secs(2)
                } else {
                    true
                };

                if should_notify {
                    self.proximity_cache
                        .insert(cache_key, (distance, Instant::now()));

                    self.add_event(GameEvent {
                        event_type: GameEventType::PlayerProximity {
                            other_player: other_player_id.clone(),
                            distance,
                        },
                        player_id: player_id.clone(),
                        timestamp: Instant::now(),
                    });

                    println!(
                        "👥 Proximity: {} and {} are {:.1} units apart",
                        player_id, other_player_id, distance
                    );
                }
            }
        }

        self.stats.proximity_checks += 1;
    }

    /// Get comprehensive game statistics
    pub fn get_game_statistics(&self) -> GameStatistics {
        let cache_hit_ratio = if self.stats.cache_hits + self.stats.cache_misses > 0 {
            self.stats.cache_hits as f32 / (self.stats.cache_hits + self.stats.cache_misses) as f32
        } else {
            0.0
        };

        GameStatistics {
            total_players: self.players.len(),
            total_rooms: self.rooms.len(),
            active_players: self
                .players
                .values()
                .filter(|p| p.current_room.is_some())
                .count(),
            cache_hit_ratio,
            total_events: self.recent_events.len(),
            pathfinding_cache_size: self.pathfinding_cache.len(),
            proximity_cache_size: self.proximity_cache.len(),
            stats: self.stats.clone(),
        }
    }

    // Helper methods

    fn position_to_grid(&self, pos: Position) -> (i32, i32) {
        (pos.x / 10, pos.y / 10) // Grid size of 10x10
    }

    fn find_room_for_position(&self, position: Position) -> Option<RoomId> {
        for (room_id, room) in &self.rooms {
            let (tl, br) = room.bounds;
            if position.x >= tl.x && position.x <= br.x && position.y >= tl.y && position.y <= br.y
            {
                return Some(room_id.clone());
            }
        }
        None
    }

    fn bfs_room_pathfinding(&self, from: &RoomId, to: &RoomId) -> Option<Vec<RoomId>> {
        if from == to {
            return Some(vec![from.clone()]);
        }

        // Simplified BFS - in reality, you'd have room connections
        // For demo purposes, assume all rooms are connected
        Some(vec![from.clone(), to.clone()])
    }

    fn add_event(&mut self, event: GameEvent) {
        self.recent_events.push_back(event);
        if self.recent_events.len() > self.max_event_history {
            self.recent_events.pop_front();
        }
    }
}

#[derive(Debug)]
pub struct GameStatistics {
    pub total_players: usize,
    pub total_rooms: usize,
    pub active_players: usize,
    pub cache_hit_ratio: f32,
    pub total_events: usize,
    pub pathfinding_cache_size: usize,
    pub proximity_cache_size: usize,
    pub stats: GameStats,
}

/// Demonstration of the multiplayer game state manager
fn multiplayer_demo() {
    println!("🎮 MULTIPLAYER GAME STATE MANAGER DEMO");
    println!("======================================\n");

    let mut game = MultiplayerGameManager::new();

    // Create game rooms
    println!("🏗️ Setting up game world...");

    game.add_room(Room {
        id: "lobby".to_string(),
        name: "Main Lobby".to_string(),
        bounds: (Position::new(0, 0), Position::new(100, 100)),
        capacity: 50,
        room_type: RoomType::Lobby,
    });

    game.add_room(Room {
        id: "arena1".to_string(),
        name: "Battle Arena 1".to_string(),
        bounds: (Position::new(100, 0), Position::new(200, 100)),
        capacity: 10,
        room_type: RoomType::BattleArena,
    });

    game.add_room(Room {
        id: "private1".to_string(),
        name: "Private Room 1".to_string(),
        bounds: (Position::new(0, 100), Position::new(100, 200)),
        capacity: 4,
        room_type: RoomType::PrivateRoom,
    });

    // Add players
    println!("\n👥 Adding players...");

    let players_data = vec![
        ("alice", "Alice", Position::new(25, 25)),
        ("bob", "Bob", Position::new(30, 30)),
        ("charlie", "Charlie", Position::new(150, 50)),
        ("diana", "Diana", Position::new(155, 55)),
        ("eve", "Eve", Position::new(50, 150)),
    ];

    for (id, name, pos) in players_data {
        let player = Player::new(id.to_string(), name.to_string(), pos);
        if let Err(e) = game.add_player(player) {
            println!("❌ Error adding player: {}", e);
        }
    }

    // Demonstrate player movement and proximity detection
    println!("\n🚶 Demonstrating player movement...");

    // Move Alice closer to Bob
    let _ = game.move_player(&"alice".to_string(), Position::new(28, 28));
    let _ = game.move_player(&"alice".to_string(), Position::new(31, 31));

    // Move Charlie to a different room
    let _ = game.move_player(&"charlie".to_string(), Position::new(50, 50));

    // Demonstrate proximity detection
    println!("\n🔍 Finding players near Alice (radius 10):");
    let nearby = game.find_players_in_radius(Position::new(31, 31), 10.0);
    for (player_id, pos, distance) in nearby {
        println!("  {} at {:?} - distance: {:.1}", player_id, pos, distance);
    }

    // Demonstrate pathfinding with caching
    println!("\n🗺️ Pathfinding demonstration:");
    for _ in 0..3 {
        if let Some(path) =
            game.find_path_between_rooms(&"lobby".to_string(), &"arena1".to_string())
        {
            println!("  Path from lobby to arena1: {:?}", path);
        }
    }

    // Room statistics
    println!("\n🏠 Room occupancy:");
    let room_stats = game.get_room_stats();
    for (room_id, occupancy, capacity) in room_stats {
        println!("  {}: {}/{} players", room_id, occupancy, capacity);
    }

    // Show players in each room
    println!("\n👥 Players by room:");
    for room_id in ["lobby", "arena1", "private1"] {
        if let Some(players) = game.get_players_in_room(&room_id.to_string()) {
            println!("  {}: {:?}", room_id, players);
        }
    }

    // Final statistics
    println!("\n📊 Game Statistics:");
    let stats = game.get_game_statistics();
    println!("  Total players: {}", stats.total_players);
    println!("  Active players: {}", stats.active_players);
    println!("  Total rooms: {}", stats.total_rooms);
    println!("  Cache hit ratio: {:.1}%", stats.cache_hit_ratio * 100.0);
    println!("  Total moves: {}", stats.stats.total_moves);
    println!("  Proximity checks: {}", stats.stats.proximity_checks);
    println!("  Pathfinding cache size: {}", stats.pathfinding_cache_size);

    println!("\n🎉 Multiplayer demo complete!");
    println!("💡 This demonstrates HashMap/HashSet usage for:");
    println!("   - Player position tracking");
    println!("   - Room membership management");
    println!("   - Spatial indexing for performance");
    println!("   - Memoized pathfinding");
    println!("   - Real-time proximity detection");
}

fn main() {
    multiplayer_demo();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation_and_movement() {
        let mut game = MultiplayerGameManager::new();

        // Add a room
        game.add_room(Room {
            id: "test_room".to_string(),
            name: "Test Room".to_string(),
            bounds: (Position::new(0, 0), Position::new(100, 100)),
            capacity: 10,
            room_type: RoomType::Lobby,
        });

        // Add a player
        let player = Player::new(
            "test_player".to_string(),
            "Test Player".to_string(),
            Position::new(50, 50),
        );
        assert!(game.add_player(player).is_ok());

        // Move player
        assert!(game
            .move_player(&"test_player".to_string(), Position::new(60, 60))
            .is_ok());

        // Check position was updated
        assert_eq!(
            game.player_positions.get("test_player"),
            Some(&Position::new(60, 60))
        );
    }

    #[test]
    fn test_room_assignment_and_capacity() {
        let mut game = MultiplayerGameManager::new();

        // Add a small capacity room
        game.add_room(Room {
            id: "small_room".to_string(),
            name: "Small Room".to_string(),
            bounds: (Position::new(0, 0), Position::new(50, 50)),
            capacity: 1,
            room_type: RoomType::PrivateRoom,
        });

        // Add first player - should succeed and auto-assign to room
        let player1 = Player::new(
            "player1".to_string(),
            "Player 1".to_string(),
            Position::new(25, 25),
        );
        assert!(game.add_player(player1).is_ok());

        // Add second player outside room bounds to avoid auto-assignment
        let player2 = Player::new(
            "player2".to_string(),
            "Player 2".to_string(),
            Position::new(75, 75),
        );
        assert!(game.add_player(player2).is_ok());

        // Try to manually assign second player to full room - should fail
        assert!(game
            .assign_player_to_room("player2".to_string(), "small_room".to_string())
            .is_err());
    }

    #[test]
    fn test_proximity_detection() {
        let mut game = MultiplayerGameManager::new();

        // Add players at different distances
        let player1 = Player::new(
            "player1".to_string(),
            "Player 1".to_string(),
            Position::new(0, 0),
        );
        let player2 = Player::new(
            "player2".to_string(),
            "Player 2".to_string(),
            Position::new(10, 0),
        ); // 10 units away
        let player3 = Player::new(
            "player3".to_string(),
            "Player 3".to_string(),
            Position::new(100, 0),
        ); // 100 units away

        assert!(game.add_player(player1).is_ok());
        assert!(game.add_player(player2).is_ok());
        assert!(game.add_player(player3).is_ok());

        // Find players within radius 20
        let nearby = game.find_players_in_radius(Position::new(0, 0), 20.0);

        // Should find player1 and player2, but not player3
        assert!(nearby.iter().any(|(id, _, _)| id == "player1"));
        assert!(nearby.iter().any(|(id, _, _)| id == "player2"));
        assert!(!nearby.iter().any(|(id, _, _)| id == "player3"));
    }

    #[test]
    fn test_pathfinding_cache() {
        let mut game = MultiplayerGameManager::new();

        game.add_room(Room {
            id: "room1".to_string(),
            name: "Room 1".to_string(),
            bounds: (Position::new(0, 0), Position::new(50, 50)),
            capacity: 10,
            room_type: RoomType::Lobby,
        });

        game.add_room(Room {
            id: "room2".to_string(),
            name: "Room 2".to_string(),
            bounds: (Position::new(50, 0), Position::new(100, 50)),
            capacity: 10,
            room_type: RoomType::GameRoom,
        });

        // First pathfinding call should be a cache miss
        let path1 = game.find_path_between_rooms(&"room1".to_string(), &"room2".to_string());
        assert!(path1.is_some());
        assert_eq!(game.stats.cache_misses, 1);
        assert_eq!(game.stats.cache_hits, 0);

        // Second call should be a cache hit
        let path2 = game.find_path_between_rooms(&"room1".to_string(), &"room2".to_string());
        assert!(path2.is_some());
        assert_eq!(game.stats.cache_misses, 1);
        assert_eq!(game.stats.cache_hits, 1);
    }
}
