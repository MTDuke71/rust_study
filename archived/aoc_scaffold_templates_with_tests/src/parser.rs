
    use anyhow::Result;
    use std::fs;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    pub fn read_to_string<P: AsRef<Path>>(p: P) -> Result<String> {
        Ok(fs::read_to_string(p)?)
    }

    pub fn read_lines<P: AsRef<Path>>(p: P) -> Result<Vec<String>> {
        let f = fs::File::open(p)?;
        let r = BufReader::new(f);
        Ok(r.lines().collect::<Result<_, _>>()?)
    }

    pub fn parse_lines_i64(input: &str) -> Result<Vec<i64>> {
        input
            .lines()
            .filter(|s| !s.trim().is_empty())
            .map(|s| Ok(s.trim().parse::<i64>()?))
            .collect()
    }

    pub fn split_blocks(input: &str) -> Vec<&str> {
        input.split("

").collect()
    }
