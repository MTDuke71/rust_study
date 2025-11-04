#!/usr/bin/env python3
"""
JSON Formatter Script
=====================

Formats JSON files with proper indentation and line breaks for readability.
Useful for AoC input files that are pasted as single-line JSON.

Usage:
    python format_json.py <input_file> [output_file]
    
    If output_file is omitted, prints to stdout.

Examples:
    # Format and print to console
    python format_json.py day12_example.txt
    
    # Format and save to new file
    python format_json.py day12_example.txt day12_example_formatted.txt
    
    # Format and overwrite original (use with caution)
    python format_json.py day12_example.txt day12_example.txt
"""

import json
import sys
from pathlib import Path


def format_json_file(input_path: str, output_path: str = None, indent: int = 2):
    """
    Format a JSON file with proper indentation.
    
    Args:
        input_path: Path to input JSON file
        output_path: Path to output file (None = print to stdout)
        indent: Number of spaces for indentation (default: 2)
    """
    try:
        # Read input file
        with open(input_path, 'r', encoding='utf-8') as f:
            content = f.read().strip()
        
        # Parse JSON
        data = json.loads(content)
        
        # Format with indentation
        formatted = json.dumps(data, indent=indent, ensure_ascii=False)
        
        # Output
        if output_path:
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write(formatted)
                f.write('\n')  # Add trailing newline
            print(f"✅ Formatted JSON written to: {output_path}")
        else:
            print(formatted)
            
    except FileNotFoundError:
        print(f"❌ Error: File not found: {input_path}", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"❌ Error: Invalid JSON in {input_path}", file=sys.stderr)
        print(f"   {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        sys.exit(1)


def main():
    if len(sys.argv) < 2:
        print("Usage: python format_json.py <input_file> [output_file]")
        print("\nExamples:")
        print("  python format_json.py day12_example.txt")
        print("  python format_json.py day12_example.txt day12_formatted.txt")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) > 2 else None
    
    format_json_file(input_file, output_file)


if __name__ == "__main__":
    main()
