"""
Day 25: Snowverload - Graph Minimum Cut Visualization

Visualizes the graph with 1495 nodes, showing:
- Two components in different colors after the 3-edge cut
- The 3 cut edges highlighted in red
- Interactive zoom/pan to explore the structure
"""

import networkx as nx
import plotly.graph_objects as go
from collections import deque

def parse_input(filename):
    """Parse the AoC input into a graph."""
    G = nx.Graph()
    
    with open(filename, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            
            source, targets = line.split(': ')
            targets = targets.split()
            
            for target in targets:
                G.add_edge(source, target)
    
    return G

def find_cut_edges_manually(G):
    """
    Find the 3 cut edges by testing combinations.
    This replicates the Rust solver logic in Python.
    """
    # Calculate edge betweenness (simplified - count BFS tree edges)
    betweenness = {}
    
    for source in G.nodes():
        visited = set()
        queue = deque([source])
        visited.add(source)
        
        while queue:
            current = queue.popleft()
            
            for neighbor in G.neighbors(current):
                if neighbor not in visited:
                    visited.add(neighbor)
                    queue.append(neighbor)
                    
                    # Count this edge in betweenness
                    edge = tuple(sorted([current, neighbor]))
                    betweenness[edge] = betweenness.get(edge, 0) + 1
    
    # Get top 20 candidate edges
    candidates = sorted(betweenness.items(), key=lambda x: x[1], reverse=True)[:20]
    candidate_edges = [edge for edge, _ in candidates]
    
    print(f"Top 5 candidate edges by betweenness:")
    for i, (edge, count) in enumerate(candidates[:5]):
        print(f"  {i+1}. {edge}: {count} paths")
    
    # Try all combinations of 3 edges
    from itertools import combinations
    
    for cut_edges in combinations(candidate_edges, 3):
        # Create test graph without these edges
        G_test = G.copy()
        G_test.remove_edges_from(cut_edges)
        
        # Check if it splits into exactly 2 components
        components = list(nx.connected_components(G_test))
        
        if len(components) == 2:
            sizes = [len(c) for c in components]
            print(f"\nFound cut! Edges: {cut_edges}")
            print(f"Component sizes: {sizes}")
            print(f"Product: {sizes[0] * sizes[1]}")
            return cut_edges
    
    return None

def get_component_colors(G, cut_edges):
    """
    Remove cut edges and assign colors to the two components.
    Returns dict mapping node -> color.
    """
    G_cut = G.copy()
    G_cut.remove_edges_from(cut_edges)
    
    components = list(nx.connected_components(G_cut))
    
    colors = {}
    component_colors = ['#3498db', '#e74c3c']  # Blue and Red
    
    for i, component in enumerate(components):
        for node in component:
            colors[node] = component_colors[i]
    
    return colors, components

def create_visualization(G, cut_edges):
    """Create interactive plotly visualization."""
    
    print("\nCalculating layout...")
    
    # Use spring layout - adjust iterations based on graph size
    iterations = 30 if G.number_of_nodes() < 50 else 50
    pos = nx.spring_layout(G, k=1.0, iterations=iterations, seed=42)
    
    # Get component colors
    node_colors, components = get_component_colors(G, cut_edges)
    
    # Prepare node trace
    node_x = []
    node_y = []
    node_color = []
    node_text = []
    node_size = []
    
    # Adjust node size based on graph size
    base_size = 20 if G.number_of_nodes() < 50 else 8
    
    for node in G.nodes():
        x, y = pos[node]
        node_x.append(x)
        node_y.append(y)
        node_color.append(node_colors[node])
        
        # Find which component
        comp_idx = 0 if node in components[0] else 1
        node_text.append(f"{node}<br>Component {comp_idx + 1}")
        node_size.append(base_size)
    
    # Prepare edge traces (separate regular and cut edges)
    regular_edge_x = []
    regular_edge_y = []
    
    cut_edge_x = []
    cut_edge_y = []
    
    for edge in G.edges():
        x0, y0 = pos[edge[0]]
        x1, y1 = pos[edge[1]]
        
        edge_tuple = tuple(sorted(edge))
        
        if edge_tuple in cut_edges:
            # Cut edge
            cut_edge_x.extend([x0, x1, None])
            cut_edge_y.extend([y0, y1, None])
        else:
            # Regular edge
            regular_edge_x.extend([x0, x1, None])
            regular_edge_y.extend([y0, y1, None])
    
    # Create traces
    regular_edge_trace = go.Scatter(
        x=regular_edge_x, y=regular_edge_y,
        line=dict(width=1.5 if G.number_of_nodes() < 50 else 0.3, color='#888'),
        hoverinfo='none',
        mode='lines',
        name='Edges'
    )
    
    cut_edge_trace = go.Scatter(
        x=cut_edge_x, y=cut_edge_y,
        line=dict(width=4 if G.number_of_nodes() < 50 else 3, color='red'),
        hoverinfo='none',
        mode='lines',
        name='Cut Edges (3)'
    )
    
    node_trace = go.Scatter(
        x=node_x, y=node_y,
        mode='markers+text' if G.number_of_nodes() < 50 else 'markers',
        text=[node for node in G.nodes()] if G.number_of_nodes() < 50 else None,
        textposition='top center',
        hoverinfo='text',
        hovertext=node_text,
        marker=dict(
            showscale=False,
            color=node_color,
            size=node_size,
            line_width=1,
            line_color='white'
        ),
        name='Nodes'
    )
    
    # Create figure
    fig = go.Figure(
        data=[regular_edge_trace, cut_edge_trace, node_trace],
        layout=go.Layout(
            title=dict(
                text=f'AoC 2023 Day 25: Graph Minimum Cut<br>' +
                     f'<sub>{G.number_of_nodes()} nodes, 2 components after removing 3 edges</sub>',
                x=0.5,
                xanchor='center',
                font=dict(size=16)
            ),
            showlegend=True,
            hovermode='closest',
            margin=dict(b=20, l=5, r=5, t=60),
            annotations=[
                dict(
                    text=f"Component 1 (Blue): {len(components[0])} nodes<br>" +
                         f"Component 2 (Red): {len(components[1])} nodes<br>" +
                         f"Answer: {len(components[0]) * len(components[1])}",
                    showarrow=False,
                    xref="paper", yref="paper",
                    x=0.01, y=0.99,
                    xanchor='left', yanchor='top',
                    bgcolor='rgba(255,255,255,0.8)',
                    bordercolor='black',
                    borderwidth=1
                )
            ],
            xaxis=dict(showgrid=False, zeroline=False, showticklabels=False),
            yaxis=dict(showgrid=False, zeroline=False, showticklabels=False),
            plot_bgcolor='white'
        )
    )
    
    return fig

def main():
    import os
    import sys
    
    # Paths - use example input by default
    use_example = len(sys.argv) > 1 and sys.argv[1] == '--example'
    
    if use_example:
        input_file = '../inputs/day25_example.txt'
        output_file = 'day25_graph_cut_example.html'
    else:
        input_file = '../inputs/day25.txt'
        output_file = 'day25_graph_cut.html'
    
    print("=== AoC 2023 Day 25: Graph Minimum Cut Visualization ===\n")
    
    # Parse input
    print("Parsing input...")
    G = parse_input(input_file)
    print(f"Graph: {G.number_of_nodes()} nodes, {G.number_of_edges()} edges")
    
    # Find cut edges
    print("\nFinding cut edges...")
    cut_edges = find_cut_edges_manually(G)
    
    if not cut_edges:
        print("ERROR: Could not find 3-edge cut!")
        return
    
    # Create visualization
    fig = create_visualization(G, cut_edges)
    
    # Save
    print(f"\nSaving visualization to {output_file}...")
    fig.write_html(output_file)
    
    print(f"\n✅ Done! Open {output_file} in your browser.")
    print("   - Drag to pan")
    print("   - Scroll to zoom")
    print("   - Hover over nodes to see details")

if __name__ == '__main__':
    main()
