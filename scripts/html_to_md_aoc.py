"""Convert AoC problem HTML files to clean Markdown.

Usage: python scripts/html_to_md_aoc.py <year>
  e.g. python scripts/html_to_md_aoc.py 2016
"""

import re
import sys
from pathlib import Path


def html_to_md(html: str) -> str:
    """Extract article content from AoC HTML and convert to Markdown."""
    # Extract all <article class="day-desc"> blocks
    articles = re.findall(
        r'<article class="day-desc">(.*?)</article>', html, re.DOTALL
    )
    if not articles:
        return ""

    text = "\n\n".join(articles)

    # Convert HTML entities
    text = text.replace("&lt;", "<")
    text = text.replace("&gt;", ">")
    text = text.replace("&amp;", "&")
    text = text.replace("&apos;", "'")
    text = text.replace("&quot;", '"')
    text = text.replace("&nbsp;", " ")

    # Convert headings: <h2>--- Day X: Title ---</h2>
    text = re.sub(r"<h2>(.*?)</h2>", r"## \1\n", text)

    # Convert emphasis: <em>text</em> and <em class="...">text</em>
    text = re.sub(r'<em[^>]*>(.*?)</em>', r"*\1*", text)

    # Convert code: <code>text</code>
    text = re.sub(r"<code>(.*?)</code>", r"`\1`", text)

    # Convert links: <a href="...">text</a>
    text = re.sub(r'<a[^>]*href="([^"]*)"[^>]*>(.*?)</a>', r"[\2](\1)", text)

    # Convert <pre><code>...</code></pre> to fenced code blocks
    def convert_pre(m):
        code = m.group(1)
        # Remove any remaining tags inside code blocks
        code = re.sub(r"<[^>]+>", "", code)
        return f"\n```\n{code}\n```\n"

    text = re.sub(
        r"<pre><code>(.*?)</code></pre>", convert_pre, text, flags=re.DOTALL
    )

    # Convert unordered lists
    text = re.sub(r"<ul>\s*", "\n", text)
    text = re.sub(r"</ul>\s*", "\n", text)
    text = re.sub(r"<li>(.*?)</li>", r"- \1", text, flags=re.DOTALL)

    # Convert ordered lists
    text = re.sub(r"<ol>\s*", "\n", text)
    text = re.sub(r"</ol>\s*", "\n", text)
    # Simple ordered list conversion (number them)
    counter = [0]

    def ol_item(m):
        counter[0] += 1
        return f"{counter[0]}. {m.group(1)}"

    text = re.sub(r"<li>(.*?)</li>", ol_item, text, flags=re.DOTALL)

    # Convert paragraphs
    text = re.sub(r"<p>(.*?)</p>", r"\1\n", text, flags=re.DOTALL)

    # Remove span tags (keep content)
    text = re.sub(r'<span[^>]*>(.*?)</span>', r"\1", text, flags=re.DOTALL)

    # Remove any remaining HTML tags
    text = re.sub(r"<[^>]+>", "", text)

    # Clean up whitespace
    text = re.sub(r"\n{3,}", "\n\n", text)
    text = text.strip()

    return text


def main():
    if len(sys.argv) < 2:
        print("Usage: python scripts/html_to_md_aoc.py <year>")
        sys.exit(1)

    year = sys.argv[1]
    days_dir = Path(f"advent_of_code/aoc{year}/Problem_Statements/days")

    if not days_dir.exists():
        print(f"Directory not found: {days_dir}")
        sys.exit(1)

    converted = 0
    for day_num in range(1, 26):
        padded = f"{day_num:02d}"
        html_file = days_dir / f"day{padded}.html"
        md_file = days_dir / f"day{padded}.md"

        if not html_file.exists():
            print(f"  Day {padded}: SKIP (no HTML)")
            continue

        html = html_file.read_text(encoding="utf-8")
        md = html_to_md(html)

        if not md:
            print(f"  Day {padded}: SKIP (no article content found)")
            continue

        # Add navigation footer
        nav_parts = []
        if day_num > 1:
            prev = f"day{day_num - 1:02d}.md"
            nav_parts.append(f"[<- Day {day_num - 1}]({prev})")
        nav_parts.append("[All Days](README.md)")
        if day_num < 25:
            nxt = f"day{day_num + 1:02d}.md"
            nav_parts.append(f"[Day {day_num + 1} ->]({nxt})")

        nav_line = "**Navigation**: " + " | ".join(nav_parts)
        md += f"\n\n---\n\n{nav_line}\n"

        md_file.write_text(md, encoding="utf-8")
        converted += 1
        print(f"  Day {padded}: OK")

    print(f"\nConverted {converted}/25 days to Markdown.")


if __name__ == "__main__":
    main()
