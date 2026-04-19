#!/usr/bin/env python3
"""
AoC 2017 Day 18 — Duet sort simulator

Runs the same "keep-running-min, ship-the-bigger" pass that the Duet
assembly implements, but on 10 values instead of 127, and with detailed
tracing so you can see exactly what each pass does.

Run:   python day18_sort_sim.py
"""

# Pick any 10 unsorted values here. Must be distinct for the trace to be
# easy to read, but the algorithm handles duplicates fine.
INITIAL = [4, 7, 1, 9, 3, 8, 2, 6, 5, 10]


def sort_pass(values, verbose=False):
    """Run one Duet sort pass on `values`.

    Duet bytecode (pc 23-38) does exactly this:
        set f 0
        rcv a                   # seed running-min
        loop N-1 times:
            rcv b
            if b > a: snd b; f = 1       ; ship the bigger
            else:     snd a; a = b       ; ship current min, update it
        snd a                            ; final running-min

    Returns (output_list, f_flag).
    """
    n = len(values)
    a = values[0]                # first rcv seeds the running min
    f = 0
    output = []

    if verbose:
        print(f"    rcv a       -> a = {a}")

    for i in range(1, n):
        b = values[i]            # rcv b
        if b > a:
            # Ship the BIGGER one, keep a (the smaller) as running min.
            if verbose:
                print(f"    rcv b={b:>2}  a={a:>2}  b>a YES -> snd b={b:>2}   a stays {a:>2}   [f=1]")
            output.append(b)
            f = 1
        else:
            # Ship a (which is >= b), and update running min to b.
            if verbose:
                print(f"    rcv b={b:>2}  a={a:>2}  b>a NO  -> snd a={a:>2}   a <- b = {b:>2}")
            output.append(a)
            a = b

    # Final snd a ships the overall minimum of all N values.
    if verbose:
        print(f"    snd a       -> emit final min {a}")
    output.append(a)

    return output, f


def visualize_state(values):
    """Return a compact string visualization of the current list."""
    return "[" + ", ".join(f"{v:>2}" for v in values) + "]"


def is_descending(values):
    return all(values[i] >= values[i + 1] for i in range(len(values) - 1))


def simulate(initial, detail_pass=1):
    """Simulate the full two-program ping-pong sort.

    The real Duet scheduler interleaves the two programs instruction-by-
    instruction, but at the pass level it's equivalent to strict alternation:
    each program consumes 10 values and emits 10, then the other goes.

    p1 runs first because p0 seeded p1's inbox via Block C before entering
    the sort loop (and p0 itself then blocks waiting for input).
    """
    print("=" * 70)
    print("AoC 2017 Day 18 -- Sort dynamics, 10-value cut-down")
    print("=" * 70)
    print(f"\nInitial values (p0 ships these into p1's inbox):\n  {visualize_state(initial)}")
    print(f"Descending-sorted target:\n  {visualize_state(sorted(initial, reverse=True))}")
    print(f"\nEach pass applies ONE sort function `f` to a 10-value list and")
    print(f"emits a permutation of the same multiset. The loop terminates")
    print(f"when a pass sees its input already descending (no b > a event).\n")

    current = list(initial)
    p1_passes = 0
    p0_passes = 0
    pass_num = 0
    turn = "p1"        # p1 runs first (its inbox has the initial batch)

    while True:
        pass_num += 1
        verbose = (pass_num == detail_pass)

        header = f"-- PASS {pass_num}  (program: {turn})  --"
        print(header)
        print(f"  input : {visualize_state(current)}")

        if verbose:
            print(f"  step-by-step trace:")
            out, f = sort_pass(current, verbose=True)
        else:
            out, f = sort_pass(current, verbose=False)

        print(f"  output: {visualize_state(out)}   f = {f}"
              + ("   <- input was already DESCENDING, terminating loop" if f == 0 else ""))
        print()

        if turn == "p1":
            p1_passes += 1
        else:
            p0_passes += 1

        current = out

        if f == 0:
            break

        # Next turn
        turn = "p0" if turn == "p1" else "p1"

        # Safety cap
        if pass_num > 100:
            print("(safety cap hit)")
            break

    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print(f"  Total passes   : {pass_num}")
    print(f"  p1 passes      : {p1_passes}  ->  sends {p1_passes * len(initial)} values")
    print(f"  p0 passes      : {p0_passes}  ->  sends {p0_passes * len(initial)} values")
    print(f"  Final state    : {visualize_state(current)}")
    print(f"  Is descending? : {is_descending(current)}")


def progression_table(initial):
    """Show the state after each pass as a stacked table (easy to see the sort)."""
    print("=" * 70)
    print("PROGRESSION TABLE -- state after each pass")
    print("=" * 70)
    print()
    current = list(initial)
    passes = [("start", current)]
    turn = "p1"
    for i in range(100):
        out, f = sort_pass(current, verbose=False)
        passes.append((f"p{1 if turn == 'p1' else 0} pass", out))
        current = out
        if f == 0:
            break
        turn = "p0" if turn == "p1" else "p1"

    # Column header: position index
    n = len(initial)
    print(f"  {'step':<12} " + " ".join(f"{i:>2}" for i in range(n)))
    print(f"  {'-' * 12} " + " ".join(f"{'--':>2}" for _ in range(n)))
    for label, state in passes:
        print(f"  {label:<12} " + " ".join(f"{v:>2}" for v in state))
    print()
    print(f"  (Descending target: " + " ".join(f"{v:>2}" for v in sorted(initial, reverse=True)) + ")")


if __name__ == "__main__":
    simulate(INITIAL, detail_pass=1)
    print()
    progression_table(INITIAL)
