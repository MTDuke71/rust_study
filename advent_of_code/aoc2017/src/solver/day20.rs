//! Day 20: Particle Swarm
//!
//! A swarm of particles in 3D space, each with position `p`, velocity `v`,
//! and acceleration `a`. Each tick: `v += a`, then `p += v`.
//!
//! - **Part 1**: which particle stays closest to the origin (Manhattan) in
//!   the long term? Since position grows like `½ a t²`, the particle with
//!   the smallest `|a|` dominates as `t → ∞`; ties fall through to `|v|`
//!   then `|p|`.
//! - **Part 2**: particles that share a position at the same tick are
//!   destroyed together. Simulate until collisions stop happening; report
//!   the survivor count.
//!
//! Input format: `p=<x,y,z>, v=<x,y,z>, a=<x,y,z>` per line.
//!
//! Part 1 is O(n) arithmetic, no simulation needed.
//! Part 2 is O(n log n) per tick (hashing positions); we bail out after a
//! window with no further collisions, so the total work depends on the
//! input's mixing time, not a fixed tick budget.

use std::collections::HashMap;

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
    // Expects text like `<-6,36,-84>` (leading '<' already stripped or not).
    let inner = s.trim_start_matches('<').trim_end_matches('>');
    let mut parts = inner.split(',').map(|n| n.trim().parse::<i64>().unwrap());
    [
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    ]
}

fn parse_line(line: &str) -> Particle {
    // `p=<..>, v=<..>, a=<..>`
    let mut p = [0i64; 3];
    let mut v = [0i64; 3];
    let mut a = [0i64; 3];
    for field in line.split(", ") {
        let (tag, rest) = field.split_once('=').expect("field must contain '='");
        let triple = parse_triple(rest);
        match tag.trim() {
            "p" => p = triple,
            "v" => v = triple,
            "a" => a = triple,
            other => panic!("unexpected field tag: {other}"),
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

fn closest_long_term(particles: &[Particle]) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, q)| (manhattan(q.a), manhattan(q.v), manhattan(q.p)))
        .map(|(i, _)| i)
        .expect("at least one particle")
}

fn survivors_after_collisions(particles: &[Particle]) -> usize {
    let mut alive: Vec<Particle> = particles.to_vec();

    // Termination heuristic: once every remaining pair is monotonically
    // diverging (same-sign acceleration, velocity, and position differences
    // in every axis), no future collision is possible. Cheap proxy: run a
    // fixed window with no collisions. 60 ticks is plenty for typical AoC
    // inputs where accelerations are ≤ ~20.
    let mut ticks_since_collision = 0usize;
    const QUIET_WINDOW: usize = 60;

    while ticks_since_collision < QUIET_WINDOW {
        for q in alive.iter_mut() {
            q.v[0] += q.a[0];
            q.v[1] += q.a[1];
            q.v[2] += q.a[2];
            q.p[0] += q.v[0];
            q.p[1] += q.v[1];
            q.p[2] += q.v[2];
        }

        let mut buckets: HashMap<Vec3, usize> = HashMap::with_capacity(alive.len());
        for q in &alive {
            *buckets.entry(q.p).or_insert(0) += 1;
        }

        let before = alive.len();
        alive.retain(|q| buckets[&q.p] == 1);
        if alive.len() == before {
            ticks_since_collision += 1;
        } else {
            ticks_since_collision = 0;
        }
    }

    alive.len()
}

pub fn solve(input: &str) -> (usize, usize) {
    let particles = parse_input(input);
    let p1 = closest_long_term(&particles);
    let p2 = survivors_after_collisions(&particles);
    (p1, p2)
}

pub fn solve_part1(input: &str) -> usize {
    closest_long_term(&parse_input(input))
}

pub fn solve_part2(input: &str) -> usize {
    survivors_after_collisions(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = concat!(
        "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\n",
        "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>\n",
    );

    const EXAMPLE_PART2: &str = concat!(
        "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\n",
        "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\n",
        "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\n",
        "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>\n",
    );

    #[test]
    fn test_parse() {
        let particles = parse_input(EXAMPLE_PART1);
        assert_eq!(particles.len(), 2);
        assert_eq!(particles[0].p, [3, 0, 0]);
        assert_eq!(particles[0].v, [2, 0, 0]);
        assert_eq!(particles[0].a, [-1, 0, 0]);
        assert_eq!(particles[1].a, [-2, 0, 0]);
    }

    #[test]
    fn test_part1_example() {
        // Particle 0 has |a| = 1, particle 1 has |a| = 2 → 0 wins long-term.
        assert_eq!(solve_part1(EXAMPLE_PART1), 0);
    }

    #[test]
    fn test_part2_example() {
        // Particles 0/1/2 collide at x=-6, leaving particle 3.
        assert_eq!(solve_part2(EXAMPLE_PART2), 1);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day20.txt");
        assert_eq!(solve_part1(input), 300);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day20.txt");
        assert_eq!(solve_part2(input), 502);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day20.txt");
        assert_eq!(solve(input), (300, 502));
    }
}
