use anyhow::Result;

// Day 2: I Was Told There Would Be No Math ---
// The elves are running low on wrapping paper, and so they need to submit an order for more.
//They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.
//
// Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier:
//find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.
//All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

pub fn solve_part1(input: &str) -> Result<String> {
    let mut total_paper = 0;

    // Cycle through all lines in the input
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let (l, w, h) = parse_dimensions(line)?;

        // Calculate surface area: 2*l*w + 2*w*h + 2*h*l
        let surface_area = 2 * (l * w) + 2 * (w * h) + 2 * (h * l);

        // Find smallest side area for extra paper
        let smallest_side = (l * w).min(w * h).min(h * l);

        total_paper += surface_area + smallest_side;
    }

    Ok(total_paper.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let mut total_ribbon = 0;

    // Cycle through all lines in the input
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let (l, w, h) = parse_dimensions(line)?;

        // Calculate ribbon needed: smallest perimeter + bow (volume)
        let mut sides = [l, w, h];
        sides.sort();
        let smallest_perimeter = 2 * sides[0] + 2 * sides[1];
        let bow = l * w * h;

        total_ribbon += smallest_perimeter + bow;
    }

    Ok(total_ribbon.to_string())
}

fn parse_dimensions(line: &str) -> Result<(i32, i32, i32)> {
    let parts: Vec<&str> = line.split('x').collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!(
            "Invalid format: expected 3 dimensions separated by 'x'"
        ));
    }

    let length = parts[0].parse::<i32>()?;
    let width = parts[1].parse::<i32>()?;
    let height = parts[2].parse::<i32>()?;

    Ok((length, width, height))
}
