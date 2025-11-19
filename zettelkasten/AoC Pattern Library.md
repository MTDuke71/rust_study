---
title: AoC Pattern Library
---

The AoC Pattern Library collects recurring problem patterns, data structures, and algorithmic tricks that appear in Advent of Code problems. It connects concrete day-level notes to higher-level algorithm and data-structure concepts.

Use this as a jump-off point when turning a specific AoC solution into a reusable pattern (for example grid traversal, graph search, dynamic programming, or string parsing).

## Featured Patterns

- [[message-passing-channels]] - Producer/consumer queueing for splitting AoC parsing, computation, and aggregation workloads across threads.
- [[shared-state-concurrency]] - Arc<Mutex<T>> leaderboards, memo caches, and checksum accumulators when AoC stages must mutate shared data.
- [[sync-send-traits]] - Trait-audit checklist to ensure custom AoC data structures safely cross thread boundaries before applying concurrency patterns.

*Links: [[AoC Patterns MOC]] [[Algorithms MOC]] [[Collections MOC]] [[Graph Theory MOC]] [[shared-state-concurrency]] [[sync-send-traits]]*
