// BFS Path Selection Demo
// This demonstrates how neighbor order affects which path is selected
// when multiple equally-short paths exist.

use std::collections::{VecDeque, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_pattern(lines: Vec<&str>) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);
        
        for line in lines {
            for ch in line.chars() {
                data.push(ch);
            }
        }
        
        Self { data, width, height }
    }
    
    fn is_passable(&self, coord: Coord) -> bool {
        if let Some(&ch) = self.get_coord(coord) {
            ch != '#'
        } else {
            false
        }
    }
    
    fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0 && coord.row < self.height as isize &&
        coord.col >= 0 && coord.col < self.width as isize
    }
    
    fn get_coord(&self, coord: Coord) -> Option<&char> {
        if self.contains(coord) {
            let idx = (coord.row as usize) * self.width + (coord.col as usize);
            Some(&self.data[idx])
        } else {
            None
        }
    }
    
    // Standard order: UP, DOWN, LEFT, RIGHT
    fn neighbors_4_standard(&self, coord: Coord) -> Vec<Coord> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        //              UP      DOWN    LEFT     RIGHT
        offsets.iter()
            .map(|(dr, dc)| Coord::new(coord.row + dr, coord.col + dc))
            .filter(|&pos| self.contains(pos))
            .collect()
    }
    
    // Alternative order: UP, RIGHT, DOWN, LEFT
    fn neighbors_4_right_first(&self, coord: Coord) -> Vec<Coord> {
        let offsets = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        //              UP      RIGHT   DOWN    LEFT
        offsets.iter()
            .map(|(dr, dc)| Coord::new(coord.row + dr, coord.col + dc))
            .filter(|&pos| self.contains(pos))
            .collect()
    }
    
    fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                print!("{}", self.data[idx]);
            }
            println!();
        }
    }
}

fn bfs_shortest_path<F>(
    grid: &Grid,
    start: Coord,
    goal: Coord,
    neighbor_fn: F,
) -> Option<Vec<Coord>>
where
    F: Fn(&Grid, Coord) -> Vec<Coord>,
{
    let mut queue = VecDeque::new();
    let mut parent: HashMap<Coord, Coord> = HashMap::new();
    
    queue.push_back(start);
    parent.insert(start, start);
    
    println!("  BFS Trace:");
    let mut step = 0;
    
    while let Some(current) = queue.pop_front() {
        step += 1;
        println!("  Step {}: Dequeue {:?}", step, current);
        
        if current == goal {
            println!("  ✓ Goal reached!");
            return Some(reconstruct_path(&parent, start, goal));
        }
        
        let neighbors = neighbor_fn(grid, current);
        print!("    Neighbors: ");
        for neighbor in &neighbors {
            print!("{:?} ", neighbor);
        }
        println!();
        
        for neighbor in neighbors {
            if parent.contains_key(&neighbor) {
                println!("      {:?} - already visited, skip", neighbor);
                continue;
            }
            if !grid.is_passable(neighbor) {
                println!("      {:?} - wall, skip", neighbor);
                continue;
            }
            
            println!("      {:?} - ✓ enqueue, parent={:?}", neighbor, current);
            parent.insert(neighbor, current);
            queue.push_back(neighbor);
        }
        println!();
    }
    
    None
}

fn reconstruct_path(
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

fn print_path(grid: &Grid, path: &[Coord]) {
    for row in 0..grid.height {
        for col in 0..grid.width {
            let pos = Coord::new(row as isize, col as isize);
            if path.contains(&pos) {
                if pos == path[0] {
                    print!("S");
                } else if pos == path[path.len() - 1] {
                    print!("G");
                } else {
                    print!("*");
                }
            } else {
                let idx = row * grid.width + col;
                print!("{}", grid.data[idx]);
            }
        }
        println!();
    }
}

fn main() {
    println!("=== BFS Path Selection Demo ===\n");
    
    // The example grid from Day25.md
    let grid = Grid::from_pattern(vec![
        "#####",
        "#S..#",
        "#.#.#",
        "#..G#",
        "#####",
    ]);
    
    println!("Grid:");
    grid.print();
    println!();
    
    let start = Coord::new(1, 1);  // S at (1,1)
    let goal = Coord::new(3, 3);   // G at (3,3)
    
    println!("Start: {:?} (S)", start);
    println!("Goal:  {:?} (G)", goal);
    println!("Wall:  (2,2)");
    println!();
    
    // Test 1: Standard order [UP, DOWN, LEFT, RIGHT]
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Test 1: Neighbor order [UP, DOWN, LEFT, RIGHT]");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    if let Some(path) = bfs_shortest_path(&grid, start, goal, Grid::neighbors_4_standard) {
        println!("Path found: {:?}", path);
        println!("Path length: {} nodes = {} steps", path.len(), path.len() - 1);
        
        // Describe the path
        print!("Moves: ");
        for i in 1..path.len() {
            let prev = path[i - 1];
            let curr = path[i];
            if curr.row < prev.row {
                print!("UP ");
            } else if curr.row > prev.row {
                print!("DOWN ");
            } else if curr.col < prev.col {
                print!("LEFT ");
            } else if curr.col > prev.col {
                print!("RIGHT ");
            }
        }
        println!();
        
        println!("\nPath visualization:");
        print_path(&grid, &path);
    }
    
    println!("\n\n");
    
    // Test 2: Alternative order [UP, RIGHT, DOWN, LEFT]
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Test 2: Neighbor order [UP, RIGHT, DOWN, LEFT]");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    if let Some(path) = bfs_shortest_path(&grid, start, goal, Grid::neighbors_4_right_first) {
        println!("Path found: {:?}", path);
        println!("Path length: {} nodes = {} steps", path.len(), path.len() - 1);
        
        // Describe the path
        print!("Moves: ");
        for i in 1..path.len() {
            let prev = path[i - 1];
            let curr = path[i];
            if curr.row < prev.row {
                print!("UP ");
            } else if curr.row > prev.row {
                print!("DOWN ");
            } else if curr.col < prev.col {
                print!("LEFT ");
            } else if curr.col > prev.col {
                print!("RIGHT ");
            }
        }
        println!();
        
        println!("\nPath visualization:");
        print_path(&grid, &path);
    }
    
    println!("\n\n=== Summary ===");
    println!("Both paths are equally short (4 steps), but BFS picks ONE");
    println!("based on the order in which neighbors are explored:");
    println!("  • [UP, DOWN, LEFT, RIGHT] → Explores DOWN before RIGHT → Bottom path (DDRR)");
    println!("  • [UP, RIGHT, DOWN, LEFT] → Explores RIGHT before DOWN → Top path (RRDD)");
    println!("\nThis demonstrates that neighbor order is DETERMINISTIC but ARBITRARY.");
    println!("The 'first' path found depends on implementation, not optimality.");
}
