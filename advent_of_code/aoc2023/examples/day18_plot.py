#!/usr/bin/env python3
"""
Day 18 Polygon Visualization
Plots the lagoon shape from vertices data.

Usage: python day18_plot.py
"""

import matplotlib.pyplot as plt
import pandas as pd

# Read vertices
df = pd.read_csv('day18_vertices.csv')

# Create plot
fig, ax = plt.subplots(figsize=(10, 8))

# Plot polygon
ax.plot(df['x'], df['y'], 'b-', linewidth=2, label='Trench')
ax.fill(df['x'], df['y'], alpha=0.3, color='lightblue', label='Lagoon Area')

# Plot vertices
ax.scatter(df['x'], df['y'], c='red', s=50, zorder=5, label='Vertices')

# Annotations
ax.set_xlabel('X Coordinate')
ax.set_ylabel('Y Coordinate')
ax.set_title('AoC 2023 Day 18: Lavaduct Lagoon Polygon')
ax.legend()
ax.grid(True, alpha=0.3)
ax.set_aspect('equal')

# Invert y-axis to match grid convention (y increases downward)
ax.invert_yaxis()

plt.tight_layout()
plt.savefig('day18_polygon.png', dpi=150)
print("✅ Plot saved to day18_polygon.png")
plt.show()
