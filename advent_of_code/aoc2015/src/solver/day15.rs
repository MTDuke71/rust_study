use anyhow::Result;

/// AoC 2015 Day 15: Science for Hungry People
///
/// Part 1: Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?
/// Part 2: What is the total score of the highest-scoring cookie you can make with a calorie total of 500?
pub fn solve_part1(input: &str) -> Result<String> {
    let ingredients = parse_input(input)?;
    let max_score = find_best_score(&ingredients, None);
    Ok(max_score.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let ingredients = parse_input(input)?;
    let max_score = find_best_score(&ingredients, Some(500));
    Ok(max_score.to_string())
}

#[derive(Debug, Clone)]
pub struct Ingredient {
    #[allow(dead_code)]
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    pub fn new(
        name: String,
        capacity: i64,
        durability: i64,
        flavor: i64,
        texture: i64,
        calories: i64,
    ) -> Self {
        Self {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

/// Calculate the score for a given recipe
/// Recipe is represented as amounts[i] = teaspoons of ingredients[i]
fn calculate_score(ingredients: &[Ingredient], amounts: &[i64]) -> i64 {
    let mut capacity_total = 0i64;
    let mut durability_total = 0i64;
    let mut flavor_total = 0i64;
    let mut texture_total = 0i64;

    for (i, ingredient) in ingredients.iter().enumerate() {
        capacity_total += ingredient.capacity * amounts[i];
        durability_total += ingredient.durability * amounts[i];
        flavor_total += ingredient.flavor * amounts[i];
        texture_total += ingredient.texture * amounts[i];
    }

    // If any property is negative, the total score is 0
    if capacity_total < 0 || durability_total < 0 || flavor_total < 0 || texture_total < 0 {
        return 0;
    }

    capacity_total * durability_total * flavor_total * texture_total
}

/// Calculate total calories for a recipe
fn calculate_calories(ingredients: &[Ingredient], amounts: &[i64]) -> i64 {
    ingredients
        .iter()
        .enumerate()
        .map(|(i, ingredient)| ingredient.calories * amounts[i])
        .sum()
}

/// Find the best score by trying all valid combinations
/// calorie_target: if Some(n), only consider recipes with exactly n calories
fn find_best_score(ingredients: &[Ingredient], calorie_target: Option<i64>) -> i64 {
    let num_ingredients = ingredients.len();

    // Handle different numbers of ingredients dynamically
    match num_ingredients {
        2 => find_best_score_2_ingredients(ingredients, calorie_target),
        3 => find_best_score_3_ingredients(ingredients, calorie_target),
        4 => find_best_score_4_ingredients(ingredients, calorie_target),
        _ => panic!("Unsupported number of ingredients: {}", num_ingredients),
    }
}

/// Optimized version for 4 ingredients
fn find_best_score_4_ingredients(ingredients: &[Ingredient], calorie_target: Option<i64>) -> i64 {
    let mut max_score = 0i64;

    for a in 0..=100 {
        for b in 0..=(100 - a) {
            for c in 0..=(100 - a - b) {
                let d = 100 - a - b - c;
                let amounts = [a, b, c, d];

                // Check calorie constraint if specified
                if let Some(target_calories) = calorie_target {
                    if calculate_calories(ingredients, &amounts) != target_calories {
                        continue;
                    }
                }

                let score = calculate_score(ingredients, &amounts);
                max_score = max_score.max(score);
            }
        }
    }

    max_score
}

/// Version for 3 ingredients
fn find_best_score_3_ingredients(ingredients: &[Ingredient], calorie_target: Option<i64>) -> i64 {
    let mut max_score = 0i64;

    for a in 0..=100 {
        for b in 0..=(100 - a) {
            let c = 100 - a - b;
            let amounts = [a, b, c];

            if let Some(target_calories) = calorie_target {
                if calculate_calories(ingredients, &amounts) != target_calories {
                    continue;
                }
            }

            let score = calculate_score(ingredients, &amounts);
            max_score = max_score.max(score);
        }
    }

    max_score
}

/// Version for 2 ingredients
fn find_best_score_2_ingredients(ingredients: &[Ingredient], calorie_target: Option<i64>) -> i64 {
    let mut max_score = 0i64;

    for a in 0..=100 {
        let b = 100 - a;
        let amounts = [a, b];

        if let Some(target_calories) = calorie_target {
            if calculate_calories(ingredients, &amounts) != target_calories {
                continue;
            }
        }

        let score = calculate_score(ingredients, &amounts);
        max_score = max_score.max(score);
    }

    max_score
}

fn parse_input(_input: &str) -> Result<Vec<Ingredient>> {
    let mut ingredients = Vec::new();

    for line in _input.lines() {
        // Add 'let' binding for line_split
        let mut line_split = line.split_whitespace();

        let name = line_split.next().unwrap().trim_end_matches(':').to_string();

        // Skip "capacity" label, then get the value
        line_split.next(); // skip "capacity"
        let capacity = line_split
            .next()
            .unwrap()
            .trim_end_matches(',')
            .parse::<i64>()?;

        // Skip "durability" label, then get the value
        line_split.next(); // skip "durability"
        let durability = line_split
            .next()
            .unwrap()
            .trim_end_matches(',')
            .parse::<i64>()?;

        // Skip "flavor" label, then get the value
        line_split.next(); // skip "flavor"
        let flavor = line_split
            .next()
            .unwrap()
            .trim_end_matches(',')
            .parse::<i64>()?;

        // Skip "texture" label, then get the value
        line_split.next(); // skip "texture"
        let texture = line_split
            .next()
            .unwrap()
            .trim_end_matches(',')
            .parse::<i64>()?;

        // Skip "calories" label, then get the value
        line_split.next(); // skip "calories"
        let calories = line_split.next().unwrap().parse::<i64>()?;

        ingredients.push(Ingredient::new(
            name, capacity, durability, flavor, texture, calories,
        ));
    }
    Ok(ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ingredient() {
        let input = "Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2";
        let ingredients = parse_input(input).unwrap();
        assert_eq!(ingredients.len(), 1);
        assert_eq!(ingredients[0].name, "Sugar");
        assert_eq!(ingredients[0].capacity, 3);
        assert_eq!(ingredients[0].durability, 0);
        assert_eq!(ingredients[0].flavor, 0);
        assert_eq!(ingredients[0].texture, -3);
        assert_eq!(ingredients[0].calories, 2);
    }

    #[test]
    fn test_parse_multiple_ingredients() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                     Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let ingredients = parse_input(input).unwrap();
        assert_eq!(ingredients.len(), 2);
        assert_eq!(ingredients[0].name, "Butterscotch");
        assert_eq!(ingredients[1].name, "Cinnamon");
    }

    #[test]
    fn test_example_part1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                     Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let result = solve_part1(input).unwrap();
        assert_eq!(result, "62842880");
    }

    #[test]
    fn test_example_part2() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                     Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "57600000");
    }

    #[test]
    fn test_calculate_score() {
        let ingredients = vec![
            Ingredient::new("Butterscotch".to_string(), -1, -2, 6, 3, 8),
            Ingredient::new("Cinnamon".to_string(), 2, 3, -2, -1, 3),
        ];
        // 44 teaspoons Butterscotch, 56 teaspoons Cinnamon
        let amounts = [44, 56];
        let score = calculate_score(&ingredients, &amounts);
        assert_eq!(score, 62842880);
    }
}
