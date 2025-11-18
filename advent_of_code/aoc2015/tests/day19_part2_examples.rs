/// Test file to verify Day 19 Part 2 works with the documented example
#[cfg(test)]
mod day19_part2_examples {
    use aoc2015::solver::day19;

    #[test]
    fn test_part2_documented_example() {
        // Example from the problem statement
        let input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH";

        // The problem states HOH can be made in 3 steps:
        // e => O (to get O)
        // O => HH (to get HH)
        // H => OH (on the second H to get HOH)
        let result = day19::solve_part2(input).unwrap();
        assert_eq!(result, "3", "HOH should take 3 steps from 'e'");
    }

    #[test]
    fn test_part2_hohoho_example() {
        // Santa's favorite molecule example
        let input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO";

        // The problem states HOHOHO can be made in 6 steps
        let result = day19::solve_part2(input).unwrap();
        assert_eq!(result, "6", "HOHOHO should take 6 steps from 'e'");
    }

    #[test]
    fn test_part1_documented_example() {
        // Verify Part 1 also works with the extended example
        let input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH";

        let result = day19::solve_part1(input).unwrap();

        // From HOH with rules e=>H, e=>O, H=>HO, H=>OH, O=>HH
        // We can make: (many possibilities)
        let count: usize = result.parse().unwrap();
        assert!(
            count > 0,
            "Should generate at least some molecules from HOH"
        );
    }

    #[test]
    fn test_reverse_construction_logic() {
        // Test that our reverse construction approach works
        let input = "e => A\nA => AB\n\nAB";

        // Forward: e -> A -> AB (2 steps)
        // Reverse: AB -> A -> e (2 steps)
        let result = day19::solve_part2(input).unwrap();
        assert_eq!(result, "2", "AB should take 2 steps from 'e'");
    }
}
