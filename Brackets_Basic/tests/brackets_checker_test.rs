use std::{fs::File, io::{BufRead, BufReader}};
use brackets_basic::{validate_brackets, BracketErrorKind};

fn read_expected_csv(path: &str) -> anyhow::Result<Vec<ExpectedRow>> {
    let f = File::open(path)?;
    let mut lines = BufReader::new(f).lines();

    let header = lines.next().ok_or_else(|| anyhow::anyhow!("empty csv"))??;
    let expected_header = "line,ok,index,kind,expected,found,open_index";
    if header.trim() != expected_header {
        anyhow::bail!("unexpected header: got `{}`, want `{}`", header, expected_header);
    }

    let mut out = Vec::new();
    for line in lines {
        let line = line?;
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() != 7 {
            anyhow::bail!("bad row with {} cols: {}", cols.len(), line);
        }
        let line_no: usize = cols[0].parse()?;
        let ok = match cols[1] {
            "true" => true,
            "false" => false,
            other => anyhow::bail!("bad ok field: {}", other),
        };
        let index = if cols[2].is_empty() { None } else { Some(cols[2].parse::<usize>()?) };
        let kind = cols[3].to_string();
        let expected = if cols[4].is_empty() { None } else { Some(cols[4].chars().next().unwrap()) };
        let found = if cols[5].is_empty() { None } else { Some(cols[5].chars().next().unwrap()) };
        let open_index = if cols[6].is_empty() { None } else { Some(cols[6].parse::<usize>()?) };

        out.push(ExpectedRow { line_no, ok, index, kind, expected, found, open_index });
    }
    Ok(out)
}

#[derive(Debug)]
struct ExpectedRow {
    line_no: usize,
    ok: bool,
    index: Option<usize>, 
    kind: String,
    expected: Option<char>, 
    found: Option<char>, 
    open_index: Option<usize>,
}

fn run_file_check(input_path: &str, expected_csv: &str) -> anyhow::Result<()> {
    let f = File::open(input_path)?;
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let expected_rows = read_expected_csv(expected_csv)?;

    assert_eq!(lines.len(), expected_rows.len(), "line count mismatch");

    for (i, (line, exp)) in lines.into_iter().zip(expected_rows.into_iter()).enumerate() {
        assert_eq!(i + 1, exp.line_no, "line numbering mismatch");

        match validate_brackets(&line) {
            Ok(()) => {
                assert!(exp.ok, "line {} expected error but got OK", exp.line_no);
            }
            Err(e) => {
                assert!(!exp.ok, "line {} expected OK but got error {:?}", exp.line_no, e);
                assert_eq!(Some(e.index), exp.index, "line {}: index mismatch", exp.line_no);
                match (exp.kind.as_str(), &e.kind) {
                    ("UnexpectedClosing", BracketErrorKind::UnexpectedClosing { found }) => {
                        assert_eq!(exp.found, Some(*found), "line {}: found char mismatch", exp.line_no);
                    }
                    ("MismatchedPair", BracketErrorKind::MismatchedPair { expected, found }) => {
                        assert_eq!(exp.expected, Some(*expected), "line {}: expected closer mismatch", exp.line_no);
                        assert_eq!(exp.found, Some(*found), "line {}: found closer mismatch", exp.line_no);
                    }
                    ("UnclosedOpenings", BracketErrorKind::UnclosedOpenings { expected, open_index }) => {
                        assert_eq!(exp.expected, Some(*expected), "line {}: expected closer mismatch", exp.line_no);
                        assert_eq!(exp.open_index, Some(*open_index), "line {}: open_index mismatch", exp.line_no);
                        assert_eq!(exp.index, Some(*open_index), "line {}: index should equal open_index", exp.line_no);
                    }
                    (other, _) => panic!("line {}: unexpected kind `{}`", exp.line_no, other),
                }
            }
        }
    }
    Ok(())
}

#[test]
fn small_dataset_matches_expected() -> anyhow::Result<()> {
    run_file_check("tests/data/brackets_small.txt", "tests/data/brackets_small.expected.csv")
}

#[test]
fn large_dataset_matches_expected() -> anyhow::Result<()> {
    run_file_check("tests/data/brackets_large.txt", "tests/data/brackets_large.expected.csv")
}