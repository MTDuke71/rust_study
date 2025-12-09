#!/usr/bin/env python3
"""
Find and visualize the largest rectangle from Day 9 Part 1
Shows which two points created the maximum area rectangle
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

# Find the largest rectangle (Part 1 solution)
def rectangle_area(p1, p2):
    width = abs(p1[0] - p2[0]) + 1
    height = abs(p1[1] - p2[1]) + 1
    return width * height

max_area = 0
max_p1 = None
max_p2 = None

print("Finding largest rectangle...")
for i in range(len(points)):
    for j in range(i + 1, len(points)):
        area = rectangle_area(points[i], points[j])
        if area > max_area:
            max_area = area
            max_p1 = points[i]
            max_p2 = points[j]
    
    if (i + 1) % 50 == 0:
        print(f"  Checked {i+1}/{len(points)} points...")

print(f"\nLargest rectangle found:")
print(f"  Point 1: {max_p1}")
print(f"  Point 2: {max_p2}")
print(f"  Area: {max_area:,}")

# Calculate rectangle bounds
min_x = min(max_p1[0], max_p2[0])
max_x = max(max_p1[0], max_p2[0])
min_y = min(max_p1[1], max_p2[1])
max_y = max(max_p1[1], max_p2[1])
width = max_x - min_x + 1
height = max_y - min_y + 1

print(f"  Rectangle: ({min_x}, {min_y}) to ({max_x}, {max_y})")
print(f"  Dimensions: {width:,} × {height:,} = {max_area:,}")

# Create figure with good resolution
fig, ax = plt.subplots(figsize=(14, 14), dpi=150)

# Extract x and y coordinates
x_coords = [p[0] for p in points]
y_coords = [p[1] for p in points]

# Plot the polygon boundary first (underneath)
for i in range(len(points)):
    p1 = points[i]
    p2 = points[(i + 1) % len(points)]
    ax.plot([p1[0], p2[0]], [p1[1], p2[1]], 'g-', linewidth=0.5, alpha=0.4, zorder=1)

# Draw the largest rectangle as a filled semi-transparent region
rect = patches.Rectangle(
    (min_x, min_y),  # Bottom-left corner
    width - 1,       # Width (subtract 1 because Rectangle uses width, not inclusive)
    height - 1,      # Height
    linewidth=3,
    edgecolor='blue',
    facecolor='cyan',
    alpha=0.3,
    zorder=2,
    label=f'Largest Rectangle\nArea: {max_area:,}'
)
ax.add_patch(rect)

# Plot all red tile points
ax.scatter(x_coords, y_coords, c='red', s=15, zorder=3, alpha=0.6, 
           edgecolors='darkred', linewidths=0.3)

# Highlight the two points that form the largest rectangle
ax.scatter([max_p1[0]], [max_p1[1]], c='yellow', s=200, zorder=5, marker='*', 
           edgecolors='orange', linewidths=2, label=f'Corner 1: {max_p1}')
ax.scatter([max_p2[0]], [max_p2[1]], c='magenta', s=200, zorder=5, marker='*', 
           edgecolors='purple', linewidths=2, label=f'Corner 2: {max_p2}')

# Draw lines from corners to show rectangle edges
ax.plot([min_x, max_x], [min_y, min_y], 'b--', linewidth=2, alpha=0.7, zorder=4)  # Bottom
ax.plot([min_x, max_x], [max_y, max_y], 'b--', linewidth=2, alpha=0.7, zorder=4)  # Top
ax.plot([min_x, min_x], [min_y, max_y], 'b--', linewidth=2, alpha=0.7, zorder=4)  # Left
ax.plot([max_x, max_x], [min_y, max_y], 'b--', linewidth=2, alpha=0.7, zorder=4)  # Right

# Set equal aspect ratio and invert y-axis
ax.set_aspect('equal')
ax.invert_yaxis()

# Add grid
ax.grid(True, alpha=0.2, linestyle='--', linewidth=0.5)

# Labels and title
ax.set_xlabel('X Coordinate', fontsize=12, fontweight='bold')
ax.set_ylabel('Y Coordinate', fontsize=12, fontweight='bold')
ax.set_title('AoC 2025 Day 9 Part 1: Largest Rectangle\n' + 
             f'Maximum Area: {max_area:,} tiles', 
             fontsize=16, fontweight='bold')

# Add legend
ax.legend(loc='upper left', fontsize=10, framealpha=0.9)

# Add detailed statistics text
stats_text = f"""Largest Rectangle Analysis:
Corner 1: ({max_p1[0]:,}, {max_p1[1]:,})
Corner 2: ({max_p2[0]:,}, {max_p2[1]:,})

Rectangle bounds:
  X: {min_x:,} to {max_x:,} (width: {width:,})
  Y: {min_y:,} to {max_y:,} (height: {height:,})

Total area: {max_area:,} tiles
Answer: {max_area}"""

ax.text(0.98, 0.02, stats_text, transform=ax.transAxes, 
        fontsize=9, verticalalignment='bottom', horizontalalignment='right',
        bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.9),
        family='monospace')

# Tight layout
plt.tight_layout()

# Save the visualization
output_file = Path(__file__).parent / "day09_largest_rectangle.jpg"
plt.savefig(output_file, format='jpg', dpi=150, bbox_inches='tight')
print(f"\nSaved visualization to: {output_file}")

output_png = Path(__file__).parent / "day09_largest_rectangle.png"
plt.savefig(output_png, format='png', dpi=150, bbox_inches='tight')
print(f"Also saved PNG version to: {output_png}")
