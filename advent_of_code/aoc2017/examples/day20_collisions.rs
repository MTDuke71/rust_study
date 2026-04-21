//! Day 20 Part 2 — collision timeline.
//!
//! Runs the Part 2 simulation and prints one line for every tick at which
//! at least one particle was destroyed. Ends with a summary of total ticks
//! run, last-collision tick, and surviving particle count.
//!
//! Usage:
//!     cargo run --release --example day20_collisions [QUIET_WINDOW]
//! Defaults to 60 ticks of quiet (same as the production solver).

use std::collections::HashMap;
use std::env;

type Vec3 = [i64; 3];

#[derive(Debug, Clone, Copy)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

fn parse_triple(s: &str) -> Vec3 {
    let inner = s.trim_start_matches('<').trim_end_matches('>');
    let mut parts = inner.split(',').map(|n| n.trim().parse::<i64>().unwrap());
    [
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    ]
}

fn parse_line(line: &str) -> Particle {
    let mut p = [0i64; 3];
    let mut v = [0i64; 3];
    let mut a = [0i64; 3];
    for field in line.split(", ") {
        let (tag, rest) = field.split_once('=').unwrap();
        let triple = parse_triple(rest);
        match tag.trim() {
            "p" => p = triple,
            "v" => v = triple,
            "a" => a = triple,
            other => panic!("unexpected tag: {other}"),
        }
    }
    Particle { p, v, a }
}

fn parse_input(input: &str) -> Vec<Particle> {
    input
        .replace("\r\n", "\n")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect()
}

fn main() {
    let quiet_window: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    let input = include_str!("../inputs/day20.txt");
    let initial = parse_input(input);
    let starting = initial.len();

    println!("Starting particles: {starting}");
    println!("Quiet window:       {quiet_window} ticks\n");
    println!("Collision events:");
    println!("  {:>5}  {:>6}  {:>6}  {:>6}", "tick", "died", "alive", "frac");

    let mut alive: Vec<Particle> = initial;
    let mut ticks_since_collision = 0usize;
    let mut tick: usize = 0;
    let mut last_collision_tick: usize = 0;
    let mut total_deaths: usize = 0;
    let mut collision_events: usize = 0;

    while ticks_since_collision < quiet_window {
        tick += 1;

        // Motion: v += a, p += v, for every alive particle.
        for q in alive.iter_mut() {
            q.v[0] += q.a[0];
            q.v[1] += q.a[1];
            q.v[2] += q.a[2];
            q.p[0] += q.v[0];
            q.p[1] += q.v[1];
            q.p[2] += q.v[2];
        }

        // Bucket by position and retain only the lonely ones.
        let mut buckets: HashMap<Vec3, usize> = HashMap::with_capacity(alive.len());
        for q in &alive {
            *buckets.entry(q.p).or_insert(0) += 1;
        }

        let before = alive.len();
        alive.retain(|q| buckets[&q.p] == 1);
        let after = alive.len();
        let died = before - after;

        if died > 0 {
            let frac = (after as f64) / (starting as f64) * 100.0;
            println!("  {tick:>5}  {died:>6}  {after:>6}  {frac:>5.1}%");
            last_collision_tick = tick;
            total_deaths += died;
            collision_events += 1;
            ticks_since_collision = 0;
        } else {
            ticks_since_collision += 1;
        }
    }

    let survivors = alive.len();
    let quiet_run = tick - last_collision_tick;

    println!();
    println!("--- Summary ---");
    println!("  Total ticks simulated:     {tick}");
    println!("  Last collision tick:       {last_collision_tick}");
    println!("  Quiet ticks since last:    {quiet_run}  (hit window of {quiet_window})");
    println!("  Collision events:          {collision_events} distinct ticks had at least one collision");
    println!("  Total particles destroyed: {total_deaths}  ({} starting → {} surviving)", starting, survivors);
    println!("  Final survivor count:      {survivors}  ← this is the Part 2 answer");
}
