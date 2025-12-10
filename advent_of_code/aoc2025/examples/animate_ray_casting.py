#!/usr/bin/env python3
"""
Animate the ray casting algorithm for AoC 2025 Day 9 Part 2
Shows how horizontal rays detect points inside/outside the polygon
"""

import matplotlib.pyplot as plt
import matplotlib.patches as patches
import matplotlib.animation as animation
from pathlib import Path
import numpy as np

# Read the input file
input_file = Path(__file__).parent.parent / "inputs" / "day09.txt"
points = []

with open(input_file) as f:
    for line in f:
        line = line.strip()
        if line:
            x, y = map(int, line.split(','))
            points.append((x, y))

print(f"Loaded {len(points)} polygon points")

# Find the largest valid rectangle corners (from our solution)
corner1 = (5398, 67501)
corner2 = (94737, 50273)

# Calculate rectangle bounds
min_x = min(corner1[0], corner2[0])
max_x = max(corner1[0], corner2[0])
min_y = min(corner1[1], corner2[1])
max_y = max(corner1[1], corner2[1])

print(f"Rectangle: ({min_x}, {min_y}) to ({max_x}, {max_y})")

# Create sample points within rectangle (fewer for animation clarity)
sample_rate = 5000  # Coarser for visibility
sample_points = []
for x in range(min_x, max_x + 1, sample_rate):
    for y in range(min_y, max_y + 1, sample_rate):
        sample_points.append((x, y))

print(f"Created {len(sample_points)} sample points for animation")

# Point-in-polygon test (horizontal ray casting)
def point_in_polygon(point, polygon):
    x, y = point
    inside = False
    crossings = []
    
    n = len(polygon)
    for i in range(n):
        x1, y1 = polygon[i]
        x2, y2 = polygon[(i + 1) % n]
        
        # Does this edge span our y-coordinate?
        if (y1 > y) != (y2 > y):
            # Calculate x-coordinate of intersection
            x_intersect = x1 + ((y - y1) / (y2 - y1)) * (x2 - x1)
            if x < x_intersect:
                inside = not inside
                crossings.append(x_intersect)
    
    return inside, crossings

# Prepare data for animation
animation_frames = []
for i, test_point in enumerate(sample_points[:50]):  # Limit to 50 points for reasonable animation time
    inside, crossings = point_in_polygon(test_point, points)
    animation_frames.append({
        'point': test_point,
        'inside': inside,
        'crossings': crossings
    })

print(f"Prepared {len(animation_frames)} animation frames")

# Create the animation
fig, ax = plt.subplots(figsize=(14, 10))

def init():
    ax.clear()
    ax.set_xlim(-5000, 105000)
    ax.set_ylim(-5000, 105000)
    ax.set_aspect('equal')
    ax.grid(True, alpha=0.3)
    ax.set_xlabel('X Coordinate', fontsize=12)
    ax.set_ylabel('Y Coordinate', fontsize=12)
    return []

def animate(frame_num):
    ax.clear()
    ax.set_xlim(-5000, 105000)
    ax.set_ylim(-5000, 105000)
    ax.set_aspect('equal')
    ax.grid(True, alpha=0.3)
    ax.set_xlabel('X Coordinate', fontsize=12)
    ax.set_ylabel('Y Coordinate', fontsize=12)
    
    # Draw polygon boundary
    poly_x = [p[0] for p in points] + [points[0][0]]
    poly_y = [p[1] for p in points] + [points[0][1]]
    ax.plot(poly_x, poly_y, 'r-', linewidth=2, label='Polygon Boundary', alpha=0.6)
    
    # Draw rectangle
    rect_width = max_x - min_x
    rect_height = max_y - min_y
    rectangle = patches.Rectangle((min_x, min_y), rect_width, rect_height,
                                   linewidth=2, edgecolor='blue', facecolor='cyan',
                                   alpha=0.2, linestyle='--', label='Test Rectangle')
    ax.add_patch(rectangle)
    
    # Get current frame data
    frame = animation_frames[frame_num]
    test_point = frame['point']
    inside = frame['inside']
    crossings = frame['crossings']
    
    # Draw the test point
    color = 'green' if inside else 'red'
    marker = 'o' if inside else 'x'
    ax.plot(test_point[0], test_point[1], marker=marker, color=color, 
            markersize=15, markeredgewidth=3, label=f'Test Point ({"Inside" if inside else "Outside"})')
    
    # Draw the horizontal ray
    ray_end = 105000
    ax.arrow(test_point[0], test_point[1], ray_end - test_point[0], 0,
             head_width=2000, head_length=3000, fc='orange', ec='orange',
             alpha=0.6, linewidth=2, label='Horizontal Ray')
    
    # Mark crossing points
    for i, cross_x in enumerate(crossings):
        ax.plot(cross_x, test_point[1], 'mo', markersize=10, 
                markeredgewidth=2, markerfacecolor='yellow',
                label='Edge Crossing' if i == 0 else '')
    
    # Add title with crossing count
    crossing_count = len(crossings)
    result = "INSIDE" if inside else "OUTSIDE"
    parity = "ODD" if crossing_count % 2 == 1 else "EVEN"
    ax.set_title(f'Ray Casting Algorithm - Frame {frame_num + 1}/{len(animation_frames)}\n'
                 f'Point: ({test_point[0]}, {test_point[1]}) - {crossing_count} crossings ({parity}) → {result}',
                 fontsize=14, fontweight='bold')
    
    # Add legend
    handles, labels = ax.get_legend_handles_labels()
    by_label = dict(zip(labels, handles))
    ax.legend(by_label.values(), by_label.keys(), loc='upper right', fontsize=10)
    
    # Add text annotation
    ax.text(0.02, 0.98, f'Crossings: {crossing_count}\nParity: {parity}\nResult: {result}',
            transform=ax.transAxes, fontsize=12, verticalalignment='top',
            bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.8))
    
    return []

print("Creating animation...")
anim = animation.FuncAnimation(fig, animate, init_func=init, 
                               frames=len(animation_frames), 
                               interval=500, blit=True, repeat=True)

# Save animation
output_file = Path(__file__).parent / "ray_casting_animation.gif"
print(f"Saving animation to {output_file}...")
anim.save(output_file, writer='pillow', fps=2, dpi=100)
print(f"Animation saved!")

# Also save as MP4 if ffmpeg is available
try:
    output_mp4 = Path(__file__).parent / "ray_casting_animation.mp4"
    print(f"Saving MP4 version to {output_mp4}...")
    anim.save(output_mp4, writer='ffmpeg', fps=2, dpi=100)
    print(f"MP4 animation saved!")
except Exception as e:
    print(f"Could not save MP4 (ffmpeg may not be installed): {e}")

print("Done!")
