//! Day 20 Part 1 — "in the long term" visualiser.
//!
//! Runs the full simulation for N ticks and shows how the Manhattan-distance
//! leaderboard changes over time. The goal is to see the ranking stabilise on
//! the particle predicted by `min_by_key(|a|, |v|, |p|)` — and to find the
//! tick `T₀` after which that particle is permanently closest.
//!
//! Usage:
//!     cargo run --release --example day20_longterm [TICKS]
//! Defaults to 2000 ticks.

use std::env;

type Vec3 = [i64; 3];

#[derive(Debug, Clone, Copy)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

fn manhattan(v: Vec3) -> i64 {
    v[0].abs() + v[1].abs() + v[2].abs()
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
    let ticks: usize = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);

    let input = include_str!("../inputs/day20.txt");
    let initial = parse_input(input);

    // The closed-form prediction.
    let predicted = initial
        .iter()
        .enumerate()
        .min_by_key(|(_, q)| (manhattan(q.a), manhattan(q.v), manhattan(q.p)))
        .map(|(i, _)| i)
        .unwrap();
    let p0 = &initial[predicted];
    println!(
        "Predicted long-term winner: particle #{predicted}  \
         |a|={}, |v|={}, |p0|={}",
        manhattan(p0.a),
        manhattan(p0.v),
        manhattan(p0.p),
    );
    println!("Total particles: {}\n", initial.len());

    // Simulate, snapshotting at the interesting ticks.
    let mut particles = initial.clone();
    let snapshot_ticks = [0usize, 1, 5, 10, 50, 100, 500, 1000, ticks];

    // T0: the latest tick where the predicted winner was NOT the single closest.
    // (Initialised to -1 meaning "predicted has always been closest".)
    let mut last_bad_tick: i64 = -1;

    for tick in 0..=ticks {
        // Step the simulation (skip the update on tick 0 — that's the initial state).
        if tick > 0 {
            for q in particles.iter_mut() {
                q.v[0] += q.a[0];
                q.v[1] += q.a[1];
                q.v[2] += q.a[2];
                q.p[0] += q.v[0];
                q.p[1] += q.v[1];
                q.p[2] += q.v[2];
            }
        }

        // Who's currently closest?
        let (current_closest, current_dist) = particles
            .iter()
            .enumerate()
            .map(|(i, q)| (i, manhattan(q.p)))
            .min_by_key(|&(_, d)| d)
            .unwrap();

        if current_closest != predicted {
            last_bad_tick = tick as i64;
        }

        if snapshot_ticks.contains(&tick) {
            let predicted_dist = manhattan(particles[predicted].p);
            let marker = if current_closest == predicted { "✓" } else { "✗" };
            println!(
                "  tick {tick:>5}  {marker}  closest=#{current_closest} (|p|={current_dist})  \
                 predicted=#{predicted} (|p|={predicted_dist})  \
                 gap={}",
                predicted_dist - current_dist,
            );
        }
    }

    println!();
    if last_bad_tick < 0 {
        println!("Predicted winner was closest at every single tick [0, {ticks}].");
    } else {
        println!(
            "Predicted winner became permanently closest at tick T₀ = {}  \
             (last contested tick).",
            last_bad_tick + 1,
        );
        println!(
            "From tick {} through {}, particle #{} is the closest at every tick.",
            last_bad_tick + 1,
            ticks,
            predicted,
        );
    }
}
