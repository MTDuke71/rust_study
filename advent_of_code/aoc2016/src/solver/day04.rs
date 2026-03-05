//! Day 4: Security Through Obscurity
//!
//! Validate encrypted room names via checksum (letter frequency),
//! then decrypt with Caesar cipher to find the northpole storage.

struct Room<'a> {
    name: &'a str,
    sector_id: u32,
    checksum: &'a str,
}

fn parse_input(input: &str) -> Vec<Room<'_>> {
    input
        .lines()
        .map(|line| {
            let bracket = line.find('[').unwrap();
            let checksum = &line[bracket + 1..line.len() - 1];
            let rest = &line[..bracket];
            let dash = rest.rfind('-').unwrap();
            let name = &rest[..dash];
            let sector_id = rest[dash + 1..].parse().unwrap();
            Room { name, sector_id, checksum }
        })
        .collect()
}

fn is_real(room: &Room<'_>) -> bool {
    let mut counts = [0u32; 26];
    for b in room.name.bytes() {
        if b != b'-' {
            counts[(b - b'a') as usize] += 1;
        }
    }
    let mut letters: Vec<(u32, u8)> = counts
        .iter()
        .enumerate()
        .filter(|(_, &c)| c > 0)
        .map(|(i, &c)| (c, i as u8))
        .collect();
    // Sort by count descending, then letter ascending
    letters.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let computed: String = letters.iter().take(5).map(|&(_, i)| (b'a' + i) as char).collect();
    computed == room.checksum
}

fn real_rooms<'a>(rooms: &'a [Room<'a>]) -> Vec<&'a Room<'a>> {
    rooms.iter().filter(|r| is_real(r)).collect()
}

fn solve_part1_with_real(real: &[&Room<'_>]) -> u32 {
    real.iter().map(|r| r.sector_id).sum()
}

fn decrypt(name: &str, shift: u32) -> String {
    let shift = (shift % 26) as u8;
    name.bytes()
        .map(|b| {
            if b == b'-' {
                ' '
            } else {
                (b'a' + (b - b'a' + shift) % 26) as char
            }
        })
        .collect()
}

fn solve_part2_with_real(real: &[&Room<'_>]) -> u32 {
    real.iter()
        .find(|r| decrypt(r.name, r.sector_id).contains("northpole"))
        .map(|r| r.sector_id)
        .expect("northpole room not found")
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    let real = real_rooms(&data);
    let p1 = solve_part1_with_real(&real);
    let p2 = solve_part2_with_real(&real);
    (p1.to_string(), p2.to_string())
}

pub fn solve_part1(input: &str) -> String {
    let data = parse_input(input);
    solve_part1_with_real(&real_rooms(&data)).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let data = parse_input(input);
    solve_part2_with_real(&real_rooms(&data)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_room() {
        let rooms = parse_input("aaaaa-bbb-z-y-x-123[abxyz]");
        assert!(is_real(&rooms[0]));
    }

    #[test]
    fn test_real_room_tie() {
        let rooms = parse_input("a-b-c-d-e-f-g-h-987[abcde]");
        assert!(is_real(&rooms[0]));
    }

    #[test]
    fn test_real_room_freq() {
        let rooms = parse_input("not-a-real-room-404[oarel]");
        assert!(is_real(&rooms[0]));
    }

    #[test]
    fn test_decoy() {
        let rooms = parse_input("totally-real-room-200[decoy]");
        assert!(!is_real(&rooms[0]));
    }

    #[test]
    fn test_part1_examples() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]";
        assert_eq!(solve_part1(input), "1514");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("qzmt-zixmtkozy-ivhz", 343), "very encrypted name");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day04.txt");
        assert_eq!(solve_part1(input), "245102");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day04.txt");
        assert_eq!(solve_part2(input), "324");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day04.txt");
        assert_eq!(solve(input), ("245102".to_string(), "324".to_string()));
    }
}
