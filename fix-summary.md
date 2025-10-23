## Changed Files:
- advent_of_code/aoc2015/examples/day04_unit_test_examples.rs
- advent_of_code/aoc2015/examples/day08_example.rs
- advent_of_code/aoc2015/examples/day13_complete_analysis.rs
- advent_of_code/aoc2015/examples/day13_graph_analysis.rs
- advent_of_code/aoc2015/examples/day13_optimization_verification.rs
- advent_of_code/aoc2015/examples/day13_optimized_solver.rs
- advent_of_code/aoc2015/tests/day06_examples.rs
- advent_of_code/aoc2015/tests/day07_tests.rs
- advent_of_code/aoc2015/tests/day_template.rs
- advent_of_code/aoc_pattern_recognition/examples/real_aoc_solutions.rs
- advent_of_code/aoc_pattern_recognition/examples/state_patterns_demo.rs
- advent_of_code/initial-clippy-output.json
- daily_study/rust_learning_week5_notes/examples/day33_panic_recovery.rs
- initial-clippy-output.json
- missions/Mission1/src/integration_tests.rs
- missions/Mission2/tests/queue_alt_test.rs
- missions/Mission2/tests/queue_test.rs
- missions/Mission3/examples/demo.rs
- missions/Mission3/tests/requirements_test.rs
- missions/Mission4/examples/interior_mutability_demo.rs
- missions/Mission5/src/dictionary.rs
- missions/Mission5/tests/hash_structures_tests.rs
- missions/Mission6/examples/aoc_examples.rs
- missions/Mission6/examples/size_hint_demo.rs
- missions/Mission6/src/grid.rs
- missions/Mission6/src/lib.rs
- missions/Mission9/src/dijkstra.rs
- missions/initial-clippy-output.json
- tutorials/Mission1_tut/examples/step2_generic_stack.rs
- tutorials/Mission1_tut/examples/step7_final_project.rs
- tutorials/Mission2_tut/examples/step1_queue_basics.rs
- tutorials/Mission2_tut/examples/step3_ring_buffer_basics.rs
- tutorials/Mission3_tut/examples/step1_basic_binary_search.rs
- tutorials/Mission4_tut/examples/get_data_comparison.rs
- tutorials/Mission4_tut/examples/memory_location_demo.rs
- tutorials/Mission4_tut/examples/simple_memory_demo.rs
- tutorials/Mission4_tut/examples/step1_basic_structure.rs
- tutorials/Mission4_tut/examples/step2_enhanced_no_warnings.rs
- tutorials/Mission4_tut/examples/step2_push_front.rs
- tutorials/Mission4_tut/examples/step3_enhanced_no_warnings.rs
- tutorials/Mission4_tut/examples/step3_peeking.rs
- tutorials/Mission4_tut/examples/step4_popping.rs
- tutorials/Mission4_tut/examples/step5_enhanced_no_warnings.rs
- tutorials/Mission4_tut/examples/step5_rc_basics.rs
- tutorials/Mission4_tut/examples/step6_borrow_checking.rs
- tutorials/Mission4_tut/examples/step6_enhanced_no_warnings.rs
- tutorials/Mission4_tut/examples/step7_performance.rs
- tutorials/Mission4_tut/examples/type_breakdown_demo.rs
- tutorials/Mission5_tut/examples/automotive_brake_safety_analysis.rs
- tutorials/Mission5_tut/examples/challenge3_multiplayer.rs
- tutorials/Mission5_tut/examples/debug_day5_part2.rs
- tutorials/Mission5_tut/examples/dictionary_wrapper_analysis.rs
- tutorials/Mission5_tut/examples/file_reading_patterns.rs
- tutorials/Mission5_tut/examples/final_project.rs
- tutorials/Mission5_tut/examples/hashmap_order_variance.rs
- tutorials/Mission5_tut/examples/hashmap_value_support.rs
- tutorials/Mission5_tut/examples/hashset_implementation_proof.rs
- tutorials/Mission5_tut/examples/impl_block_organization.rs
- tutorials/Mission5_tut/examples/step4_multi_value_patterns.rs
- tutorials/Mission5_tut/examples/type_inference_demo.rs
- tutorials/Mission6_tut/examples/step4_pathfinding.rs
- tutorials/Mission6_tut/examples/step7_documentation.rs
- tutorials/Mission7_tut/examples/step1_graph_fundamentals.rs
- tutorials/Mission7_tut/examples/step7_integration_project.rs
- tutorials/Mission8_tut/examples/day3_exercises_solutions.rs
- tutorials/Mission8_tut/examples/day4_exercises_solutions.rs
- tutorials/Mission8_tut/examples/step3_composition.rs
- tutorials/Mission8_tut/examples/step6_integration_testing.rs
- tutorials/initial-clippy-output.json

## Summary of Changes:
 .../aoc2015/examples/day04_unit_test_examples.rs   |   1 -
 advent_of_code/aoc2015/examples/day08_example.rs   |   3 +-
 .../aoc2015/examples/day13_complete_analysis.rs    |   2 +-
 .../aoc2015/examples/day13_graph_analysis.rs       |   1 -
 .../examples/day13_optimization_verification.rs    |   2 +-
 .../aoc2015/examples/day13_optimized_solver.rs     |   2 +-
 advent_of_code/aoc2015/tests/day06_examples.rs     |  27 +-
 advent_of_code/aoc2015/tests/day07_tests.rs        |   8 +-
 advent_of_code/aoc2015/tests/day_template.rs       |   1 -
 .../examples/real_aoc_solutions.rs                 |  12 +-
 .../examples/state_patterns_demo.rs                |   7 +-
 advent_of_code/initial-clippy-output.json          | 226 ++++++++++
 .../examples/day33_panic_recovery.rs               |   2 +-
 initial-clippy-output.json                         | 484 +++++++++++++++++++++
 missions/Mission1/src/integration_tests.rs         |   2 +-
 missions/Mission2/tests/queue_alt_test.rs          |   9 +-
 missions/Mission2/tests/queue_test.rs              |   2 +-
 missions/Mission3/examples/demo.rs                 |   4 +-
 missions/Mission3/tests/requirements_test.rs       |   4 +-
 .../Mission4/examples/interior_mutability_demo.rs  |   2 +-
 missions/Mission5/src/dictionary.rs                |   8 +-
 missions/Mission5/tests/hash_structures_tests.rs   |   2 +-
 missions/Mission6/examples/aoc_examples.rs         |   6 +-
 missions/Mission6/examples/size_hint_demo.rs       |   2 +-
 missions/Mission6/src/grid.rs                      |   2 +-
 missions/Mission6/src/lib.rs                       |   4 +-
 missions/Mission9/src/dijkstra.rs                  |   2 +-
 missions/initial-clippy-output.json                | 480 ++++++++++++++++++++
 .../Mission1_tut/examples/step2_generic_stack.rs   |   2 +-
 .../Mission1_tut/examples/step7_final_project.rs   |  11 +-
 .../Mission2_tut/examples/step1_queue_basics.rs    |   2 +-
 .../examples/step3_ring_buffer_basics.rs           |   2 +-
 .../examples/step1_basic_binary_search.rs          |   2 +-
 .../Mission4_tut/examples/get_data_comparison.rs   |  12 +
 .../Mission4_tut/examples/memory_location_demo.rs  |  12 +-
 .../Mission4_tut/examples/simple_memory_demo.rs    |  12 +-
 .../Mission4_tut/examples/step1_basic_structure.rs |   6 +
 .../examples/step2_enhanced_no_warnings.rs         |   6 +
 .../Mission4_tut/examples/step2_push_front.rs      |   6 +
 .../examples/step3_enhanced_no_warnings.rs         |   6 +
 tutorials/Mission4_tut/examples/step3_peeking.rs   |   6 +
 tutorials/Mission4_tut/examples/step4_popping.rs   |   8 +-
 .../examples/step5_enhanced_no_warnings.rs         |   6 +
 tutorials/Mission4_tut/examples/step5_rc_basics.rs |   6 +
 .../Mission4_tut/examples/step6_borrow_checking.rs |   6 +
 .../examples/step6_enhanced_no_warnings.rs         |   6 +
 .../Mission4_tut/examples/step7_performance.rs     |  12 +
 .../Mission4_tut/examples/type_breakdown_demo.rs   |   6 +
 .../examples/automotive_brake_safety_analysis.rs   |  10 +-
 .../examples/challenge3_multiplayer.rs             |  15 +-
 .../Mission5_tut/examples/debug_day5_part2.rs      |   1 -
 .../examples/dictionary_wrapper_analysis.rs        |   4 +-
 .../Mission5_tut/examples/file_reading_patterns.rs |   2 +-
 tutorials/Mission5_tut/examples/final_project.rs   |  14 +-
 .../examples/hashmap_order_variance.rs             |   2 +-
 .../Mission5_tut/examples/hashmap_value_support.rs |  12 +-
 .../examples/hashset_implementation_proof.rs       |   9 +
 .../examples/impl_block_organization.rs            |  18 +
 .../examples/step4_multi_value_patterns.rs         |  30 +-
 .../Mission5_tut/examples/type_inference_demo.rs   |   4 +-
 .../Mission6_tut/examples/step4_pathfinding.rs     |   8 +-
 .../Mission6_tut/examples/step7_documentation.rs   |   4 +-
 .../examples/step1_graph_fundamentals.rs           |   3 +-
 .../examples/step7_integration_project.rs          |  20 +-
 .../examples/day3_exercises_solutions.rs           |   9 +-
 .../examples/day4_exercises_solutions.rs           |  15 +-
 .../Mission8_tut/examples/step3_composition.rs     |   5 +-
 .../examples/step6_integration_testing.rs          |   9 +
 tutorials/initial-clippy-output.json               | 350 +++++++++++++++
 69 files changed, 1819 insertions(+), 167 deletions(-)
