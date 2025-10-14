// Day 28: Week 4 Review & Integration
// Complete integrated example combining all Week 4 concepts:
// Grid fundamentals, navigation, BFS, Dijkstra, flood fill, and parsing

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

fn main() {
    println!("=== Day 28: Week 4 Integration - Dungeon Explorer ===\n");
    
    // Step 1: Parse input (Day 27 - String Parsing)
    let input = "\
####################
#S.................#
#.####.###~~###....#
#.#..#.#.~~~~#.....#
#.#..#.#.~~~~#.###.#
#.####.###~~###.#G.#
#..................#
####################";
    
    println!("🔷 Task 1: Parse Dungeon");
    println!("========================");
    println!("{}", input);
    
    let dungeon = DungeonGrid::parse(input);
    println!("Parsed: {}x{} grid", dungeon.height(), dungeon.width());
    println!("Start: {:?}", dungeon.start);
    println!("Goal: {:?}\n", dungeon.goal);
    
    // Step 2: Find shortest path (Day 26 - Dijkstra)
    println!("🔷 Task 2: Find Shortest Path (Dijkstra)");
    println!("=========================================");
    
    if let Some((cost, path)) = dungeon.shortest_path() {
        println!("Path found!");
        println!("  Total cost: {}", cost);
        println!("  Steps: {}", path.len() - 1);
        
        // Visualize path
        println!("\nPath visualization:");
        dungeon.visualize_path(&path);
    } else {
        println!("No path exists!");
    }
    println!();
    
    // Step 3: Count rooms (Day 24 - Connected Components)
    println!("🔷 Task 3: Analyze Rooms");
    println!("========================");
    
    let rooms = dungeon.find_rooms();
    println!("Found {} rooms", rooms.len());
    
    for (i, room) in rooms.iter().enumerate() {
        let info = dungeon.analyze_component(room);
        println!("  Room {}: {} cells, perimeter {}",
            i + 1, info.size, info.perimeter);
    }
    println!();
    
    // Step 4: Find largest room
    println!("🔷 Task 4: Largest Room");
    println!("=======================");
    
    if let Some(largest) = rooms.iter().max_by_key(|r| r.len()) {
        let info = dungeon.analyze_component(largest);
        println!("Largest room:");
        println!("  Size: {} cells", info.size);
        println!("  Perimeter: {}", info.perimeter);
        println!("  Bounds: {:?}", info.bounds);
    }
    println!();
    
    // Step 5: Water coverage
    println!("🔷 Task 5: Terrain Analysis");
    println!("===========================");
    
    let terrain_stats = dungeon.analyze_terrain();
    println!("Terrain distribution:");
    println!("  Walls: {} ({:.1}%)",
        terrain_stats.walls,
        100.0 * terrain_stats.walls as f64 / terrain_stats.total as f64);
    println!("  Floor: {} ({:.1}%)",
        terrain_stats.floor,
        100.0 * terrain_stats.floor as f64 / terrain_stats.total as f64);
    println!("  Water: {} ({:.1}%)",
        terrain_stats.water,
        100.0 * terrain_stats.water as f64 / terrain_stats.total as f64);
    println!();
    
    // Bonus: BFS distance map
    println!("🔷 Bonus: Distance Map from Start (BFS)");
    println!("========================================");
    
    let distances = dungeon.bfs_distances(dungeon.start);
    println!("Reachable cells: {}", distances.len());
    
    if let Some(&max_dist) = distances.values().max() {
        println!("Farthest reachable point: {} steps away", max_dist);
    }
    
    // Show distance map (first few rows)
    println!("\nDistance visualization (excerpt):");
    for row in 0..8.min(dungeon.height()) {
        for col in 0..dungeon.width() {
            let pos = Coord::new(row as isize, col as isize);
            if dungeon[(row, col)] == '#' {
                print!(" ## ");
            } else if let Some(&dist) = distances.get(&pos) {
                print!("{:3} ", dist);
            } else {
                print!(" -- ");
            }
        }
        println!();
    }
}

// === Complete Integrated Data Structures ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

pub struct DungeonGrid {
    data: Vec<char>,
    width: usize,
    height: usize,
    start: Coord,
    goal: Coord,
}

impl DungeonGrid {
    /// Day 27: String Parsing - Parse grid from text
    pub fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);
        let mut start = Coord::new(0, 0);
        let mut goal = Coord::new(0, 0);
        
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                data.push(ch);
                if ch == 'S' {
                    start = Coord::new(row as isize, col as isize);
                } else if ch == 'G' {
                    goal = Coord::new(row as isize, col as isize);
                }
            }
        }
        
        Self { data, width, height, start, goal }
    }
    
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
    
    /// Day 22: Grid Fundamentals - Bounds checking
    pub fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0 && coord.row < self.height as isize &&
        coord.col >= 0 && coord.col < self.width as isize
    }
    
    pub fn get_coord(&self, coord: Coord) -> Option<char> {
        if self.contains(coord) {
            Some(self[(coord.row as usize, coord.col as usize)])
        } else {
            None
        }
    }
    
    /// Day 23: Grid Navigation - 4-connected neighbors
    pub fn neighbors_4(&self, coord: Coord) -> Vec<Coord> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.iter()
            .map(|(dr, dc)| Coord::new(coord.row + dr, coord.col + dc))
            .filter(|&pos| self.contains(pos))
            .collect()
    }
    
    fn get_cost(&self, coord: Coord) -> usize {
        match self.get_coord(coord) {
            Some('#') => usize::MAX, // Impassable
            Some('~') => 3,           // Water
            Some('.') | Some('S') | Some('G') => 1, // Floor
            _ => usize::MAX,
        }
    }
    
    /// Day 26: Dijkstra's Algorithm - Shortest path with costs
    pub fn shortest_path(&self) -> Option<(usize, Vec<Coord>)> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            cost: usize,
            position: Coord,
        }
        
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost)
                    .then_with(|| self.position.cmp(&other.position))
            }
        }
        
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        
        let mut dist: HashMap<Coord, usize> = HashMap::new();
        let mut parent: HashMap<Coord, Coord> = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        dist.insert(self.start, 0);
        parent.insert(self.start, self.start);
        heap.push(State { cost: 0, position: self.start });
        
        while let Some(State { cost, position }) = heap.pop() {
            if position == self.goal {
                let path = self.reconstruct_path(&parent, self.start, self.goal);
                return Some((cost, path));
            }
            
            if let Some(&best) = dist.get(&position) {
                if cost > best {
                    continue;
                }
            }
            
            for neighbor in self.neighbors_4(position) {
                let move_cost = self.get_cost(neighbor);
                if move_cost == usize::MAX {
                    continue;
                }
                
                let next_cost = cost + move_cost;
                let is_shorter = dist.get(&neighbor)
                    .map_or(true, |&current| next_cost < current);
                
                if is_shorter {
                    dist.insert(neighbor, next_cost);
                    parent.insert(neighbor, position);
                    heap.push(State { cost: next_cost, position: neighbor });
                }
            }
        }
        
        None
    }
    
    fn reconstruct_path(
        &self,
        parent: &HashMap<Coord, Coord>,
        start: Coord,
        goal: Coord,
    ) -> Vec<Coord> {
        let mut path = Vec::new();
        let mut current = goal;
        
        while current != start {
            path.push(current);
            current = parent[&current];
        }
        path.push(start);
        path.reverse();
        path
    }
    
    /// Day 25: BFS - Distance computation
    pub fn bfs_distances(&self, start: Coord) -> HashMap<Coord, usize> {
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        
        queue.push_back((start, 0));
        distances.insert(start, 0);
        
        while let Some((current, dist)) = queue.pop_front() {
            for neighbor in self.neighbors_4(current) {
                if distances.contains_key(&neighbor) {
                    continue;
                }
                if self.get_cost(neighbor) == usize::MAX {
                    continue;
                }
                
                distances.insert(neighbor, dist + 1);
                queue.push_back((neighbor, dist + 1));
            }
        }
        
        distances
    }
    
    /// Day 24: Connected Components - Find all rooms
    pub fn find_rooms(&self) -> Vec<Vec<Coord>> {
        let mut visited = HashSet::new();
        let mut rooms = Vec::new();
        
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = Coord::new(row as isize, col as isize);
                
                if visited.contains(&pos) {
                    continue;
                }
                
                let cell = self[(row, col)];
                if cell == '#' {
                    continue;
                }
                
                let room = self.explore_room(pos, &mut visited);
                if !room.is_empty() {
                    rooms.push(room);
                }
            }
        }
        
        rooms
    }
    
    fn explore_room(&self, start: Coord, visited: &mut HashSet<Coord>) -> Vec<Coord> {
        let mut stack = vec![start];
        let mut room = Vec::new();
        
        while let Some(pos) = stack.pop() {
            if visited.contains(&pos) {
                continue;
            }
            if !self.contains(pos) {
                continue;
            }
            if self.get_coord(pos) == Some('#') {
                continue;
            }
            
            visited.insert(pos);
            room.push(pos);
            
            for neighbor in self.neighbors_4(pos) {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        
        room
    }
    
    /// Day 24: Component analysis
    pub fn analyze_component(&self, component: &[Coord]) -> ComponentInfo {
        let mut min_row = isize::MAX;
        let mut max_row = isize::MIN;
        let mut min_col = isize::MAX;
        let mut max_col = isize::MIN;
        let mut perimeter = 0;
        
        let component_set: HashSet<_> = component.iter().copied().collect();
        
        for &pos in component {
            min_row = min_row.min(pos.row);
            max_row = max_row.max(pos.row);
            min_col = min_col.min(pos.col);
            max_col = max_col.max(pos.col);
            
            for neighbor in self.neighbors_4(pos) {
                if !component_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
        
        ComponentInfo {
            size: component.len(),
            perimeter,
            bounds: (
                Coord::new(min_row, min_col),
                Coord::new(max_row, max_col),
            ),
        }
    }
    
    /// Day 27: Terrain analysis
    pub fn analyze_terrain(&self) -> TerrainStats {
        let mut walls = 0;
        let mut floor = 0;
        let mut water = 0;
        
        for &cell in &self.data {
            match cell {
                '#' => walls += 1,
                '.' | 'S' | 'G' => floor += 1,
                '~' => water += 1,
                _ => {}
            }
        }
        
        TerrainStats {
            total: self.data.len(),
            walls,
            floor,
            water,
        }
    }
    
    pub fn visualize_path(&self, path: &[Coord]) {
        let path_set: HashSet<_> = path.iter().copied().collect();
        
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = Coord::new(row as isize, col as isize);
                let cell = self[(row, col)];
                
                if path_set.contains(&pos) && cell != 'S' && cell != 'G' {
                    print!("*");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

impl std::ops::Index<(usize, usize)> for DungeonGrid {
    type Output = char;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.index(row, col)]
    }
}

#[derive(Debug)]
pub struct ComponentInfo {
    pub size: usize,
    pub perimeter: usize,
    pub bounds: (Coord, Coord),
}

#[derive(Debug)]
pub struct TerrainStats {
    pub total: usize,
    pub walls: usize,
    pub floor: usize,
    pub water: usize,
}
