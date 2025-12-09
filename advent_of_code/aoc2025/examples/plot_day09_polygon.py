#!/usr/bin/env python3
"""
Visualize the Day 9 polygon from Advent of Code 2025
Creates a JPEG showing the 496 red tile points connected in sequence
"""

import matplotlib.pyplot as plt
import matplotlib.patches as patches
from pathlib import Path

# Read the input file
input_file = Path(__file__).parent.parent / "inputs" / "day09.txt"
points = []

with open(input_file) as f:
    for line in f:
        line = line.strip()
        if line:
            x, y = map(int, line.split(','))
            points.append((x, y))

print(f"Loaded {len(points)} points")

# Create figure with good resolution
fig, ax = plt.subplots(figsize=(12, 12), dpi=150)

# Extract x and y coordinates
x_coords = [p[0] for p in points]
y_coords = [p[1] for p in points]

# Plot the polygon boundary
# Connect each point to the next, wrapping around
for i in range(len(points)):
    p1 = points[i]
    p2 = points[(i + 1) % len(points)]
    ax.plot([p1[0], p2[0]], [p1[1], p2[1]], 'g-', linewidth=0.5, alpha=0.6)

# Plot the red tile points
ax.scatter(x_coords, y_coords, c='red', s=20, zorder=5, alpha=0.8, edgecolors='darkred', linewidths=0.5)

# Mark the first point with a different color to show start/end
ax.scatter([points[0][0]], [points[0][1]], c='blue', s=100, zorder=6, marker='*', 
           edgecolors='darkblue', linewidths=1, label='Start/End point')

# Set equal aspect ratio and invert y-axis (to match typical AoC coordinate system)
ax.set_aspect('equal')
ax.invert_yaxis()

# Add grid
ax.grid(True, alpha=0.3, linestyle='--', linewidth=0.5)

# Labels and title
ax.set_xlabel('X Coordinate', fontsize=12)
ax.set_ylabel('Y Coordinate', fontsize=12)
ax.set_title('AoC 2025 Day 9: Polygon formed by 496 Red Tiles\nConnected in sequence with green boundary', 
             fontsize=14, fontweight='bold')

# Add legend
ax.legend(loc='best', fontsize=10)

# Add statistics text
min_x, max_x = min(x_coords), max(x_coords)
min_y, max_y = min(y_coords), max(y_coords)
width = max_x - min_x
height = max_y - min_y

stats_text = f"""Statistics:
Points: {len(points)}
X range: {min_x:,} to {max_x:,} (width: {width:,})
Y range: {min_y:,} to {max_y:,} (height: {height:,})
Bounding box area: {width * height:,}"""

ax.text(0.02, 0.98, stats_text, transform=ax.transAxes, 
        fontsize=9, verticalalignment='top',
        bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.8))

# Tight layout
plt.tight_layout()

# Save as JPEG
output_file = Path(__file__).parent / "day09_polygon.jpg"
plt.savefig(output_file, format='jpg', dpi=150, bbox_inches='tight')
print(f"Saved visualization to: {output_file}")

# Also save as PNG for better quality
output_png = Path(__file__).parent / "day09_polygon.png"
plt.savefig(output_png, format='png', dpi=150, bbox_inches='tight')
print(f"Also saved PNG version to: {output_png}")

# Don't show the plot in headless mode
# plt.show()
