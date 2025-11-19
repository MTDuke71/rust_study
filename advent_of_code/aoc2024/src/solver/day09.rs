use anyhow::{bail, Context, Result};
use mission5::Dictionary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileInfo {
    start: usize,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GapSegment {
    start: usize,
    len: usize,
}

#[derive(Debug, Clone)]
struct DiskLayout {
    blocks: Vec<Option<u32>>,
    file_table: Dictionary<usize, FileInfo>,
    file_count: usize,
    gaps: Vec<GapSegment>,
}

#[derive(Debug)]
struct SnapshotRecorder<'a> {
    frames: &'a mut Vec<String>,
    limit: usize,
}

impl<'a> SnapshotRecorder<'a> {
    fn new(frames: &'a mut Vec<String>, limit: usize) -> Self {
        Self { frames, limit }
    }

    fn capture(&mut self, blocks: &[Option<u32>]) {
        if self.frames.len() < self.limit {
            self.frames.push(render_blocks(blocks));
        }
    }

    fn is_saturated(&self) -> bool {
        self.frames.len() >= self.limit
    }
}

/// Parse the dense disk map into explicit block state along with file metadata cached
/// inside the Mission 5 `Dictionary` for quick lookups during whole-file moves.
///
/// # Parsing Logic
/// The dense format alternates between file lengths and gap lengths (always starting with a file).
/// Even digits at positions 0, 2, 4, ... are file lengths.
/// Odd digits at positions 1, 3, 5, ... are gap lengths.
///
/// **Critical invariant**: `file_id` increments for every file position, even when the length is 0.
/// This ensures that the file ID sequence is contiguous (0, 1, 2, 3, ...) regardless of which files
/// are actually written to the disk. Zero-length files still consume an ID but produce no blocks.
/// This simplifies Part 2's reverse iteration (highest ID to lowest) without gaps in the sequence.
fn parse_disk(input: &str) -> Result<DiskLayout> {
    let mut digits: Vec<usize> = Vec::new();
    for ch in input.chars().filter(|c| !c.is_whitespace()) {
        let len = ch
            .to_digit(10)
            .with_context(|| format!("Invalid digit '{ch}' in disk map"))? as usize;
        digits.push(len);
    }
    if digits.is_empty() {
        bail!("Disk map empty");
    }
    let total_blocks: usize = digits.iter().sum();
    let mut blocks: Vec<Option<u32>> = Vec::with_capacity(total_blocks);
    let mut file_table = Dictionary::with_capacity(digits.len() / 2 + 2);
    let mut gaps: Vec<GapSegment> = Vec::new();

    let mut is_file = true;
    let mut file_id = 0usize;
    let mut cursor = 0usize;
    for len in digits {
        if is_file {
            // Only insert into file_table if len > 0, but always increment file_id
            // to maintain contiguous ID sequence
            if len > 0 {
                file_table.insert(file_id, FileInfo { start: cursor, len });
                for _ in 0..len {
                    blocks.push(Some(file_id as u32));
                }
                cursor += len;
            }
            // Unconditional increment: zero-length files still consume an ID
            file_id += 1;
        } else if len > 0 {
            gaps.push(GapSegment { start: cursor, len });
            for _ in 0..len {
                blocks.push(None);
            }
            cursor += len;
        }
        is_file = !is_file;
    }

    Ok(DiskLayout {
        blocks,
        file_table,
        file_count: file_id,
        gaps,
    })
}

/// Compute the filesystem checksum by summing `position * file_id` for every occupied block.
fn checksum(blocks: &[Option<u32>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(idx, cell)| cell.map(|id| (idx as u64) * (id as u64)))
        .sum()
}

/// Execute part 1's block-level compaction by pulling the rightmost file blocks
/// into the leftmost free slots until no holes remain between files.
///
/// # Two-Pointer Invariants
/// - `left` points to the leftmost gap (None) or advances past occupied blocks
/// - `right` points to the rightmost file block (Some) or retreats past gaps
/// - Loop continues while `left < right` to avoid crossing pointers
/// - Each swap moves one file block into one gap, then advances both pointers
/// - Termination: when `left >= right`, all file blocks are contiguous on the left
fn compact_blocks(blocks: &mut [Option<u32>]) {
    compact_blocks_with_recorder(blocks, None);
}

/// Same logic as [`compact_blocks`] but optionally records intermediate states for visualization.
fn compact_blocks_with_recorder<'a>(
    blocks: &mut [Option<u32>],
    mut recorder: Option<SnapshotRecorder<'a>>,
) {
    if blocks.is_empty() {
        return;
    }
    if let Some(rec) = recorder.as_mut() {
        rec.capture(blocks);
    }
    let mut left = 0usize;
    let mut right = blocks.len() - 1;
    
    // Two-pointer compaction: left finds gaps, right finds file blocks
    while left < right {
        // Advance left to the first gap
        while left < blocks.len() && blocks[left].is_some() {
            left += 1;
        }
        // Retreat right to the last file block
        while right > left && blocks[right].is_none() {
            right = right.saturating_sub(1);
        }
        if left >= right {
            break;
        }
        // Swap: move file block from right into gap at left
        blocks[left] = blocks[right];
        blocks[right] = None;
        if let Some(rec) = recorder.as_mut() {
            rec.capture(blocks);
            if rec.is_saturated() {
                recorder = None;
            }
        }
        left += 1;
        if right == 0 {
            break;
        }
        right -= 1;
    }
}

/// Locate the earliest gap that sits to the left of `file_start` and can fully contain the file.
///
/// # Early Break Optimization
/// Gaps are stored in sorted order by start position. Once we encounter a gap that starts
/// at or after `file_start`, we can stop searching immediately—all subsequent gaps will also
/// be too far to the right. This prevents unnecessary scans of the gap list.
fn find_gap(gaps: &[GapSegment], file_start: usize, file_len: usize) -> Option<usize> {
    for (idx, gap) in gaps.iter().enumerate() {
        // Early break: gaps are sorted by start, so no later gap can be left of file_start
        if gap.start >= file_start {
            break;
        }
        if gap.len >= file_len {
            return Some(idx);
        }
    }
    None
}

/// Clear the source range for a file and write its ID contiguously at the new destination.
fn relocate_file(blocks: &mut [Option<u32>], fid: u32, from: FileInfo, to_start: usize) {
    for offset in 0..from.len {
        blocks[from.start + offset] = None;
    }
    for offset in 0..from.len {
        debug_assert!(blocks[to_start + offset].is_none());
        blocks[to_start + offset] = Some(fid);
    }
}

/// Implement part 2's whole-file compaction by scanning files from highest ID to lowest
/// and sliding each one into the earliest suitable gap recorded in `DiskLayout`.
///
/// # Gap Update Logic
/// After relocating a file into a gap:
/// 1. Update the gap's start: `gap.start += file.len` (consumed portion)
/// 2. Shrink the gap's length: `gap.len -= file.len` (remaining space)
/// 3. If `gap.len == 0`, remove the gap entirely to prevent future scans from considering it
///
/// This ensures gaps shrink correctly and are removed when fully consumed, avoiding
/// off-by-one errors and maintaining sorted gap order for `find_gap`.
fn compact_whole_files(layout: &mut DiskLayout) {
    compact_whole_files_with_recorder(layout, None);
}

fn compact_whole_files_with_recorder<'a>(
    layout: &mut DiskLayout,
    mut recorder: Option<SnapshotRecorder<'a>>,
) {
    if let Some(rec) = recorder.as_mut() {
        rec.capture(&layout.blocks);
    }
    for fid in (0..layout.file_count).rev() {
        let info = match layout.file_table.get(&fid) {
            Some(info) if info.len > 0 => *info,
            _ => continue,
        };
        if let Some(gap_idx) = find_gap(&layout.gaps, info.start, info.len) {
            let target_start = layout.gaps[gap_idx].start;
            relocate_file(&mut layout.blocks, fid as u32, info, target_start);
            if let Some(entry) = layout.file_table.get_mut(&fid) {
                entry.start = target_start;
            }
            // Gap update: advance start, shrink length, remove if fully consumed
            layout.gaps[gap_idx].start += info.len;
            layout.gaps[gap_idx].len -= info.len;
            if layout.gaps[gap_idx].len == 0 {
                layout.gaps.remove(gap_idx);
            }
            if let Some(rec) = recorder.as_mut() {
                rec.capture(&layout.blocks);
                if rec.is_saturated() {
                    recorder = None;
                }
            }
        }
    }
}

/// Solve part 1 by parsing the dense disk map, running block-level compaction, and returning the checksum.
pub fn solve_part1(input: &str) -> Result<String> {
    let mut layout = parse_disk(input)?;
    compact_blocks(&mut layout.blocks);
    Ok(checksum(&layout.blocks).to_string())
}

/// Solve part 2 by parsing the disk, relocating whole files using Mission 5 dictionary lookups, and computing the checksum.
pub fn solve_part2(input: &str) -> Result<String> {
    let mut layout = parse_disk(input)?;
    compact_whole_files(&mut layout);
    Ok(checksum(&layout.blocks).to_string())
}

/// Return the rendered disk states for the first `max_frames` snapshots of part 1 compaction.
pub fn visualize_part1(input: &str, max_frames: usize) -> Result<Vec<String>> {
    if max_frames == 0 {
        let mut layout = parse_disk(input)?;
        compact_blocks(&mut layout.blocks);
        return Ok(Vec::new());
    }
    let mut layout = parse_disk(input)?;
    let mut frames = Vec::new();
    let recorder = SnapshotRecorder::new(&mut frames, max_frames);
    compact_blocks_with_recorder(&mut layout.blocks, Some(recorder));
    Ok(frames)
}

/// Return the rendered disk states for the first `max_frames` snapshots of part 2 file relocation.
pub fn visualize_part2(input: &str, max_frames: usize) -> Result<Vec<String>> {
    if max_frames == 0 {
        let mut layout = parse_disk(input)?;
        compact_whole_files(&mut layout);
        return Ok(Vec::new());
    }
    let mut layout = parse_disk(input)?;
    let mut frames = Vec::new();
    let recorder = SnapshotRecorder::new(&mut frames, max_frames);
    compact_whole_files_with_recorder(&mut layout, Some(recorder));
    Ok(frames)
}

fn render_blocks(blocks: &[Option<u32>]) -> String {
    blocks
        .iter()
        .map(|cell| match cell {
            Some(id) if *id < 10 => char::from(b'0' + *id as u8),
            Some(id) if *id < 36 => char::from(b'A' + (*id as u8 - 10)),
            Some(_) => '#',
            None => '.',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(SAMPLE).unwrap();
        assert_eq!(result, "1928");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(SAMPLE).unwrap();
        assert_eq!(result, "2858");
    }

    #[test]
    fn test_parse_preserves_lengths() {
        let layout = parse_disk("90909").unwrap();
        assert_eq!(layout.blocks.len(), 27);
        assert_eq!(layout.file_count, 3);
        assert!(layout.gaps.is_empty());
    }

    #[test]
    fn test_visualize_part1_frames() {
        let frames = visualize_part1(SAMPLE, 2).unwrap();
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0], "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_visualize_part2_frames() {
        let frames = visualize_part2(SAMPLE, 2).unwrap();
        assert_eq!(frames[0], "00...111...2...333.44.5555.6666.777.888899");
    }

    // Edge case: zero-length files in the input should still increment file_id
    #[test]
    fn test_zero_length_file_increments_id() {
        // "103" = file0 len=1, gap len=0, file1 len=3
        // file_count should be 2 (file0 and file1), even though gap is zero
        let layout = parse_disk("103").unwrap();
        assert_eq!(layout.file_count, 2);
        assert_eq!(layout.blocks.len(), 4);
        assert_eq!(layout.file_table.len(), 2);
        assert_eq!(layout.blocks, vec![Some(0), Some(1), Some(1), Some(1)]);
    }

    // Edge case: zero-length file at the start
    #[test]
    fn test_zero_length_file_at_start() {
        // "012" = file0 len=0, gap len=1, file1 len=2
        let layout = parse_disk("012").unwrap();
        assert_eq!(layout.file_count, 2);
        assert_eq!(layout.blocks.len(), 3);
        // file0 has no entry in file_table (len=0), file1 starts at position 1
        assert_eq!(layout.file_table.len(), 1);
        assert_eq!(layout.file_table.get(&1).unwrap().start, 1);
        assert_eq!(layout.blocks, vec![None, Some(1), Some(1)]);
    }

    // Edge case: gap consumption in part2 should shrink gap correctly
    #[test]
    fn test_gap_shrinks_after_partial_consumption() {
        // "151" = file0 len=1, gap len=5, file1 len=1
        // After part2, file1 (len=1) should move into the gap, shrinking it to len=4
        let mut layout = parse_disk("151").unwrap();
        assert_eq!(layout.gaps.len(), 1);
        assert_eq!(layout.gaps[0].len, 5);
        compact_whole_files(&mut layout);
        // file1 moved into gap, gap shrinks from 5 to 4
        assert_eq!(layout.gaps.len(), 1);
        assert_eq!(layout.gaps[0].len, 4);
        assert_eq!(layout.gaps[0].start, 2); // start advanced from 1 to 2
    }

    // Edge case: gap fully consumed should be removed
    #[test]
    fn test_gap_removed_when_fully_consumed() {
        // "131" = file0 len=1, gap len=3, file1 len=1
        // file1 (len=1) moves into gap, shrinking it to 2
        // Then if we had another file of len=2, it would consume the rest
        // For this test, we verify partial consumption first
        let mut layout = parse_disk("131").unwrap();
        assert_eq!(layout.gaps.len(), 1);
        assert_eq!(layout.gaps[0].len, 3);
        compact_whole_files(&mut layout);
        // file1 moved, gap shrinks from 3 to 2
        assert_eq!(layout.gaps.len(), 1);
        assert_eq!(layout.gaps[0].len, 2);
    }

    // Edge case: gap exactly matches file size, should be removed
    #[test]
    fn test_gap_removed_when_exact_fit() {
        // "212" = file0 len=2, gap len=1, file1 len=2
        // file1 can't fit in gap (needs 2, gap is 1), so gap remains
        // Better test: "121" = file0 len=1, gap len=2, file1 len=1
        let mut layout = parse_disk("121").unwrap();
        assert_eq!(layout.gaps.len(), 1);
        assert_eq!(layout.gaps[0].len, 2);
        compact_whole_files(&mut layout);
        // file1 (len=1) moves into gap (len=2), shrinking it to 1
        assert_eq!(layout.gaps.len(), 1);
        assert_eq!(layout.gaps[0].len, 1);

        // Better exact-fit test: "111" = file0 len=1, gap len=1, file1 len=1
        let mut layout2 = parse_disk("111").unwrap();
        assert_eq!(layout2.gaps.len(), 1);
        assert_eq!(layout2.gaps[0].len, 1);
        compact_whole_files(&mut layout2);
        // file1 (len=1) exactly fits gap (len=1), gap should be removed
        assert_eq!(layout2.gaps.len(), 0);
    }

    // Edge case: single file, no gaps
    #[test]
    fn test_single_file_no_gaps() {
        let layout = parse_disk("5").unwrap();
        assert_eq!(layout.file_count, 1);
        assert_eq!(layout.blocks.len(), 5);
        assert!(layout.gaps.is_empty());
        assert_eq!(layout.blocks, vec![Some(0); 5]);
    }

    // Edge case: all gaps, no files (starts with gap)
    #[test]
    fn test_all_gaps_no_files() {
        // This is tricky: the format always starts with a file length
        // So "0303" = file0 len=0, gap len=3, file1 len=0, gap len=3
        let layout = parse_disk("0303").unwrap();
        assert_eq!(layout.file_count, 2);
        assert_eq!(layout.blocks.len(), 6);
        assert_eq!(layout.gaps.len(), 2);
        assert!(layout.blocks.iter().all(|b| b.is_none()));
    }

    // Edge case: checksum with large file IDs
    #[test]
    fn test_checksum_large_file_ids() {
        // "1111111111" = 5 files of len=1 separated by gaps of len=1
        // file IDs: 0,1,2,3,4 at positions 0,2,4,6,8
        let layout = parse_disk("1111111111").unwrap();
        assert_eq!(layout.file_count, 5);
        let cs = checksum(&layout.blocks);
        // positions: 0*0 + 2*1 + 4*2 + 6*3 + 8*4 = 0 + 2 + 8 + 18 + 32 = 60
        assert_eq!(cs, 60);
    }

    // Edge case: part1 compaction on already-compact disk
    #[test]
    fn test_part1_already_compact() {
        let mut layout = parse_disk("30").unwrap(); // file0 len=3, no gap
        let original = layout.blocks.clone();
        compact_blocks(&mut layout.blocks);
        assert_eq!(layout.blocks, original); // no change
    }

    // Edge case: part2 relocation when no suitable gaps exist
    #[test]
    fn test_part2_no_suitable_gaps() {
        // "312" = file0 len=3, gap len=1, file1 len=2
        // file1 (len=2) can't fit in gap (len=1), so it stays put
        let mut layout = parse_disk("312").unwrap();
        let file1_start = layout.file_table.get(&1).unwrap().start;
        compact_whole_files(&mut layout);
        assert_eq!(layout.file_table.get(&1).unwrap().start, file1_start);
    }

    // Edge case: part2 with zero-length file should skip it
    #[test]
    fn test_part2_skips_zero_length_files() {
        // "1030" = file0 len=1, gap len=0, file1 len=3, no final gap
        let layout = parse_disk("1030").unwrap();
        assert_eq!(layout.file_count, 2);
        // file1 has len=3, exists in file_table
        assert!(layout.file_table.get(&1).is_some());
        
        // "0131" = file0 len=0, gap len=1, file1 len=3, gap len=1
        // file0 has len=0, should NOT be in file_table because len=0
        // Gap at position 0 with len=1 is too small for file1 (len=3), so file1 stays put
        let mut layout2 = parse_disk("0131").unwrap();
        assert_eq!(layout2.file_count, 2);
        assert!(layout2.file_table.get(&0).is_none());
        
        let file1_start_before = layout2.file_table.get(&1).unwrap().start;
        compact_whole_files(&mut layout2);
        let file1_start_after = layout2.file_table.get(&1).unwrap().start;
        
        // file1 should NOT move because gap (len=1) is too small for file (len=3)
        assert_eq!(file1_start_after, file1_start_before);
    }

    // Edge case: empty input should fail gracefully
    #[test]
    fn test_empty_input_fails() {
        let result = parse_disk("");
        assert!(result.is_err());
    }

    // Edge case: invalid characters in input
    #[test]
    fn test_invalid_character_fails() {
        let result = parse_disk("12a34");
        assert!(result.is_err());
    }
}
