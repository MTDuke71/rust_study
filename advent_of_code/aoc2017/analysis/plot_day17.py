"""Plot iterations where (pos + steps) % i == 0 for AoC 2017 Day 17."""
import csv
from pathlib import Path
import matplotlib.pyplot as plt

HERE = Path(__file__).parent
csv_path = HERE / "day17_hits.csv"
out_path = HERE / "day17_hits.png"

rows = []
with csv_path.open() as f:
    reader = csv.DictReader(f)
    for r in reader:
        rows.append((int(r["hit_num"]), int(r["i"])))

hit_nums = [r[0] for r in rows]
iters = [r[1] for r in rows]
gaps = [iters[0]] + [iters[k] - iters[k - 1] for k in range(1, len(iters))]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 5))

# Left: step plot of value_at_1 over iteration (log x)
# Extend to N = 50M so the final plateau is visible
N = 50_000_000
x = iters + [N]
y = iters + [iters[-1]]  # value_at_1 stays at the last hit
ax1.step(x, y, where="post", linewidth=2, color="tab:blue")
ax1.scatter(iters, iters, color="tab:red", zorder=3, label="hit (write at index 1)")
# annotate final hit
ax1.annotate(
    f"answer = {iters[-1]:,}\n(holds from i={iters[-1]:,} to i={N:,})",
    xy=(iters[-1], iters[-1]),
    xytext=(iters[-1] * 0.02, iters[-1] * 1.1),
    fontsize=9,
    arrowprops=dict(arrowstyle="->", color="black"),
)
ax1.set_xscale("log")
ax1.set_yscale("log")
ax1.set_xlabel("iteration i (log)")
ax1.set_ylabel("value currently at index 1 (log)")
ax1.set_title("Value at index 1 vs. iteration (only 17 updates in 50M steps)")
ax1.grid(True, which="both", alpha=0.3)
ax1.legend()

# Right: bar chart of gap sizes (how long each value persisted)
final_gap = N - iters[-1]
gap_labels = [f"#{k+1}\ni={iters[k]:,}" for k in range(len(iters))] + ["(tail)"]
gap_values = gaps + [final_gap]
colors = ["tab:blue"] * len(gaps) + ["tab:orange"]
ax2.bar(range(len(gap_values)), gap_values, color=colors)
ax2.set_yscale("log")
ax2.set_xticks(range(len(gap_values)))
ax2.set_xticklabels(gap_labels, rotation=45, ha="right", fontsize=7)
ax2.set_ylabel("iterations until next write (log)")
ax2.set_title("Persistence of each value at index 1")
ax2.grid(True, axis="y", alpha=0.3)

plt.tight_layout()
plt.savefig(out_path, dpi=130)
print(f"Saved {out_path}")
print(f"Total hits: {len(iters)}")
print(f"Final hit at i = {iters[-1]:,} (the Part 2 answer)")
print(f"Iterations with no writes after final hit: {final_gap:,}")
