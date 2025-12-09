#!/usr/bin/env python3
"""
Find and visualize the largest rectangle from Day 9 Part 2
Shows which two points created the maximum area rectangle WITHIN the polygon
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

# Build polygon boundary (from our Rust solution)
def add_line_to_set(tiles, p1, p2):
    """Bresenham's line algorithm"""
    dx = abs(p2[0] - p1[0])
    dy = abs(p2[1] - p1[1])
    sx = 1 if p1[0] < p2[0] else -1
    sy = 1 if p1[1] < p2[1] else -1
    
    err = dx - dy
    x, y = p1
    
    while True:
        tiles.add((x, y))
        if x == p2[0] and y == p2[1]:
            break
        
        e2 = 2 * err
        if e2 > -dy:
            err -= dy
            x += sx
        if e2 < dx:
            err += dx
            y += sy

def build_polygon_boundary(points):
    tiles = set()
    for point in points:
        tiles.add(point)
    
    for i in range(len(points)):
        p1 = points[i]
        p2 = points[(i + 1) % len(points)]
        add_line_to_set(tiles, p1, p2)
    
    return tiles

def point_in_polygon(point, polygon):
    """Ray casting algorithm"""
    x, y = point
    inside = False
    
    for i in range(len(polygon)):
        x1, y1 = polygon[i]
        x2, y2 = polygon[(i + 1) % len(polygon)]
        
        if (y1 > y) != (y2 > y):
            x_intersect = x1 + ((y - y1) / (y2 - y1)) * (x2 - x1)
            if x < x_intersect:
                inside = not inside
    
    return inside

def rectangle_in_polygon_optimized(p1, p2, boundary, polygon):
    """AABB sampling check"""
    min_x = min(p1[0], p2[0])
    max_x = max(p1[0], p2[0])
    min_y = min(p1[1], p2[1])
    max_y = max(p1[1], p2[1])
    
    sample_rate = max(10, max(max_x - min_x, max_y - min_y) // 100)
    
    # Check corners first
    for x, y in [(min_x, min_y), (min_x, max_y), (max_x, min_y), (max_x, max_y)]:
        if (x, y) not in boundary and not point_in_polygon((x, y), polygon):
            return False
    
    # Sample along edges
    for x in range(min_x, max_x + 1, sample_rate):
        for y in [min_y, max_y]:
            if (x, y) not in boundary and not point_in_polygon((x, y), polygon):
                return False
    
    for y in range(min_y, max_y + 1, sample_rate):
        for x in [min_x, max_x]:
            if (x, y) not in boundary and not point_in_polygon((x, y), polygon):
                return False
    
    # Sample interior
    for x in range(min_x, max_x + 1, sample_rate):
        for y in range(min_y, max_y + 1, sample_rate):
            if (x, y) not in boundary and not point_in_polygon((x, y), polygon):
                return False
    
    return True

def rectangle_area(p1, p2):
    width = abs(p1[0] - p2[0]) + 1
    height = abs(p1[1] - p2[1]) + 1
    return width * height

# Build the boundary
print("Building polygon boundary...")
boundary = build_polygon_boundary(points)
print(f"Boundary contains {len(boundary):,} tiles")

# Find the largest valid rectangle (Part 2)
print("\nFinding largest rectangle within polygon...")
max_area = 0
max_p1 = None
max_p2 = None

for i in range(len(points)):
    for j in range(i + 1, len(points)):
        if rectangle_in_polygon_optimized(points[i], points[j], boundary, points):
            area = rectangle_area(points[i], points[j])
            if area > max_area:
                max_area = area
                max_p1 = points[i]
                max_p2 = points[j]
    
    if (i + 1) % 50 == 0:
        print(f"  Checked {i+1}/{len(points)} points...")

print(f"\nLargest valid rectangle found:")
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

# Create figure
fig, ax = plt.subplots(figsize=(14, 14), dpi=150)

x_coords = [p[0] for p in points]
y_coords = [p[1] for p in points]

# Plot the polygon boundary
for i in range(len(points)):
    p1 = points[i]
    p2 = points[(i + 1) % len(points)]
    ax.plot([p1[0], p2[0]], [p1[1], p2[1]], 'g-', linewidth=1.5, alpha=0.6, zorder=1)

# Draw the largest VALID rectangle
rect = patches.Rectangle(
    (min_x, min_y),
    width - 1,
    height - 1,
    linewidth=3,
    edgecolor='blue',
    facecolor='cyan',
    alpha=0.4,
    zorder=2,
    label=f'Largest Valid Rectangle\nArea: {max_area:,}'
)
ax.add_patch(rect)

# Plot all red tile points
ax.scatter(x_coords, y_coords, c='red', s=15, zorder=3, alpha=0.7, 
           edgecolors='darkred', linewidths=0.3)

# Highlight the two points
ax.scatter([max_p1[0]], [max_p1[1]], c='yellow', s=250, zorder=5, marker='*', 
           edgecolors='orange', linewidths=2, label=f'Corner 1: {max_p1}')
ax.scatter([max_p2[0]], [max_p2[1]], c='magenta', s=250, zorder=5, marker='*', 
           edgecolors='purple', linewidths=2, label=f'Corner 2: {max_p2}')

# Draw rectangle edges
ax.plot([min_x, max_x], [min_y, min_y], 'b--', linewidth=2.5, alpha=0.8, zorder=4)
ax.plot([min_x, max_x], [max_y, max_y], 'b--', linewidth=2.5, alpha=0.8, zorder=4)
ax.plot([min_x, min_x], [min_y, max_y], 'b--', linewidth=2.5, alpha=0.8, zorder=4)
ax.plot([max_x, max_x], [min_y, max_y], 'b--', linewidth=2.5, alpha=0.8, zorder=4)

ax.set_aspect('equal')
ax.invert_yaxis()
ax.grid(True, alpha=0.2, linestyle='--', linewidth=0.5)

ax.set_xlabel('X Coordinate', fontsize=12, fontweight='bold')
ax.set_ylabel('Y Coordinate', fontsize=12, fontweight='bold')
ax.set_title('AoC 2025 Day 9 Part 2: Largest Valid Rectangle\n' + 
             f'(Only Red/Green Tiles) - Area: {max_area:,}', 
             fontsize=16, fontweight='bold')

ax.legend(loc='upper left', fontsize=11, framealpha=0.95)

stats_text = f"""Part 2 Largest Rectangle:
Corner 1: ({max_p1[0]:,}, {max_p1[1]:,})
Corner 2: ({max_p2[0]:,}, {max_p2[1]:,})

Rectangle bounds:
  X: {min_x:,} to {max_x:,} (width: {width:,})
  Y: {min_y:,} to {max_y:,} (height: {height:,})

Total area: {max_area:,} tiles
Answer: {max_area}

Constraint: Rectangle must be
entirely within polygon boundary"""

ax.text(0.98, 0.02, stats_text, transform=ax.transAxes, 
        fontsize=9, verticalalignment='bottom', horizontalalignment='right',
        bbox=dict(boxstyle='round', facecolor='lightgreen', alpha=0.9),
        family='monospace')

plt.tight_layout()

output_file = Path(__file__).parent / "day09_part2_largest_rectangle.jpg"
plt.savefig(output_file, format='jpg', dpi=150, bbox_inches='tight')
print(f"\nSaved visualization to: {output_file}")

output_png = Path(__file__).parent / "day09_part2_largest_rectangle.png"
plt.savefig(output_png, format='png', dpi=150, bbox_inches='tight')
print(f"Also saved PNG version to: {output_png}")
