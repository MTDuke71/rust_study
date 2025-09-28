use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mission6::{Grid, Coord, PathFinder, FloodFill};

fn grid_creation_benchmark(c: &mut Criterion) {
    c.bench_function("grid_creation_1000x1000", |b| {
        b.iter(|| {
            Grid::new(black_box(1000), black_box(1000), black_box(0u32))
        })
    });

    c.bench_function("grid_creation_100x100", |b| {
        b.iter(|| {
            Grid::new(black_box(100), black_box(100), black_box(0u32))
        })
    });
}

fn grid_access_benchmark(c: &mut Criterion) {
    let grid = Grid::new(1000, 1000, 42u32);
    
    c.bench_function("grid_index_access", |b| {
        b.iter(|| {
            let coord = Coord::new(black_box(500), black_box(500));
            black_box(grid[coord])
        })
    });

    c.bench_function("grid_tuple_access", |b| {
        b.iter(|| {
            black_box(grid[(black_box(500), black_box(500))])
        })
    });

    c.bench_function("grid_bounds_check", |b| {
        b.iter(|| {
            let coord = Coord::new(black_box(999), black_box(999));
            black_box(grid.in_bounds(coord))
        })
    });
}

fn grid_iteration_benchmark(c: &mut Criterion) {
    let grid = Grid::new(100, 100, 1u32);
    
    c.bench_function("grid_iter_sum", |b| {
        b.iter(|| {
            black_box(grid.iter().sum::<u32>())
        })
    });

    c.bench_function("grid_enumerate", |b| {
        b.iter(|| {
            black_box(grid.enumerate().count())
        })
    });
}

fn pathfinding_benchmark(c: &mut Criterion) {
    // Create a maze with some obstacles
    let mut grid = Grid::new(50, 50, '.');
    
    // Add obstacles in a pattern
    for i in 5..45 {
        grid[(i, 10)] = '#';
        grid[(i, 30)] = '#';
    }
    grid[(25, 10)] = '.'; // Gap
    grid[(25, 30)] = '.'; // Gap
    
    let start = Coord::new(0, 0);
    let goal = Coord::new(49, 49);
    
    c.bench_function("bfs_pathfinding_50x50", |b| {
        b.iter(|| {
            black_box(PathFinder::bfs(&grid, start, goal, |&cell| cell != '#'))
        })
    });

    c.bench_function("astar_pathfinding_50x50", |b| {
        b.iter(|| {
            black_box(PathFinder::astar(&grid, start, goal, |&cell| cell != '#', |_| 1))
        })
    });
}

fn flood_fill_benchmark(c: &mut Criterion) {
    let grid = Grid::new(100, 100, '.');
    let start = Coord::new(50, 50);
    
    c.bench_function("flood_fill_region_detection", |b| {
        b.iter(|| {
            black_box(FloodFill::find_region_4(&grid, start, &'.'))
        })
    });

    c.bench_function("flood_fill_all_regions", |b| {
        b.iter(|| {
            black_box(FloodFill::find_all_regions(&grid, &'.'))
        })
    });
}

fn coordinate_benchmark(c: &mut Criterion) {
    let coord1 = Coord::new(100, 200);
    let coord2 = Coord::new(300, 400);
    
    c.bench_function("manhattan_distance", |b| {
        b.iter(|| {
            black_box(coord1.manhattan_distance(coord2))
        })
    });

    c.bench_function("euclidean_distance", |b| {
        b.iter(|| {
            black_box(coord1.euclidean_distance(coord2))
        })
    });

    c.bench_function("neighbors_4_generation", |b| {
        b.iter(|| {
            black_box(coord1.neighbors_4().count())
        })
    });

    c.bench_function("neighbors_8_generation", |b| {
        b.iter(|| {
            black_box(coord1.neighbors_8().count())
        })
    });
}

criterion_group!(
    benches,
    grid_creation_benchmark,
    grid_access_benchmark,
    grid_iteration_benchmark,
    pathfinding_benchmark,
    flood_fill_benchmark,
    coordinate_benchmark
);
criterion_main!(benches);