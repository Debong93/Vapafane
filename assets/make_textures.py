"""Erzeugt die Pixel-Texturen für die Oberfläche (ui/textures.css) und Vorschaubilder.

Alles eigene Pixel-Art im 16×16-Stil – nichts von Mojang kopiert oder nachgezeichnet:
- Erde: Grundbraun mit unregelmäßigen dunkleren und helleren Pixel-Gruppen
- Grasblock-Seite: Grasrand mit gezackter Kante, die in die Erde hängt
- Crying Obsidian: fast schwarzes Violett mit Schimmer und leuchtenden violetten Tränen

Aufruf: python3 assets/make_textures.py [Ordner für PNG-Vorschauen]
"""
import random
import sys
import urllib.parse

N = 16  # Kachelgröße in Pixeln
SCALE = 4  # so groß wird ein Pixel im SVG gezeichnet


def svg(grid):
    h, w = len(grid), len(grid[0])
    rects = []
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c:
                rects.append(f"<rect x='{x * SCALE}' y='{y * SCALE}' width='{SCALE}' height='{SCALE}' fill='{c}'/>")
    s = (f"<svg xmlns='http://www.w3.org/2000/svg' width='{w * SCALE}' height='{h * SCALE}' "
         f"shape-rendering='crispEdges'>{''.join(rects)}</svg>")
    return 'url("data:image/svg+xml,' + urllib.parse.quote(s, safe=" =:/'") + '")'


def clusters(r, grid, colors, count, max_size):
    """Kleine unregelmäßige Pixel-Gruppen (1–max_size Pixel) setzen."""
    for _ in range(count):
        x, y = r.randrange(N), r.randrange(len(grid))
        c = r.choice(colors)
        for _ in range(r.randint(1, max_size)):
            grid[y % len(grid)][x % N] = c
            x += r.choice((-1, 0, 1))
            y += r.choice((0, 1))


def dirt(seed, base, darks, lights, rows=N):
    r = random.Random(seed)
    g = [[base] * N for _ in range(rows)]
    clusters(r, g, darks, 26, 3)
    clusters(r, g, lights, 16, 2)
    return g


def grass_side(seed, greens, dirt_rows):
    """Obere Kante eines Grasblocks: Gras mit gezackten „Tropfen“ über Erde."""
    r = random.Random(seed)
    g = [row[:] for row in dirt_rows]
    depth = [3] * N
    for x in range(N):
        depth[x] = r.choice((3, 3, 4, 4, 5, 6))
    for x in range(N):
        for y in range(depth[x]):
            g[y][x] = r.choice(greens)
        # einzelne Gras-Pixel darunter für eine unruhige Kante
        if r.random() < 0.3 and depth[x] + 1 < len(g):
            g[depth[x] + 1][x] = greens[-1]
    return g


def crying_obsidian(seed):
    r = random.Random(seed)
    base = ["#0d0717", "#110a1e", "#160c27", "#0a0512"]
    g = [[r.choice(base) for _ in range(N)] for _ in range(N)]
    # Obsidian-Schimmer: hellere violette Splitter
    clusters(r, g, ["#2a1747", "#22123a", "#3a2063"], 14, 3)
    # Tränen: senkrechte, leuchtende Spuren mit hellem Kern und Glimmen drumherum
    for _ in range(4):
        x, y = r.randrange(1, N - 1), r.randrange(0, N - 6)
        length = r.randint(3, 6)
        for i in range(length):
            yy = (y + i) % N
            g[yy][x] = "#b45cff" if i < length - 1 else "#e3b8ff"
            for dx in (-1, 1):
                if r.random() < 0.6:
                    g[yy][(x + dx) % N] = "#4a1f80"
        g[(y + length) % N][x] = "#7a2fd0"
    return g


def png(grid, path, scale=8):
    """Vorschau als PNG (nur zur eigenen Kontrolle)."""
    from PIL import Image
    h, w = len(grid), len(grid[0])
    img = Image.new("RGBA", (w * scale, h * scale), (0, 0, 0, 0))
    for y, row in enumerate(grid):
        for x, c in enumerate(row):
            if c:
                rgb = tuple(int(c[i:i + 2], 16) for i in (1, 3, 5))
                for yy in range(y * scale, (y + 1) * scale):
                    for xx in range(x * scale, (x + 1) * scale):
                        img.putpixel((xx, yy), rgb + (255,))
    img.save(path)


DIRT = dirt(11, "#7d573a", ["#5b3d27", "#644430", "#6c4a33"], ["#9a7150", "#a77d58"])
DIRT_DARK = dirt(11, "#4a3322", ["#33231a", "#3b291d", "#402c1f"], ["#5d412b", "#664a31"])
GREENS = ["#5d9e37", "#6db342", "#4f8a2e", "#78c04c", "#457a28"]
GREENS_DARK = ["#3f7a24", "#4a8c2c", "#356a1e", "#559a33", "#2f5c1a"]
GRASS = grass_side(5, GREENS, dirt(23, "#7d573a", ["#5b3d27", "#644430"], ["#9a7150"], rows=8))
GRASS_DARK = grass_side(5, GREENS_DARK, dirt(23, "#4a3322", ["#33231a", "#3b291d"], ["#5d412b"], rows=8))
OBSIDIAN = crying_obsidian(42)

css = [
    "/* Erzeugt von assets/make_textures.py – eigene Pixel-Art im 16×16-Stil */",
    ":root {",
    f"  --tex-dirt: {svg(DIRT)};",
    f"  --tex-dirt-dark: {svg(DIRT_DARK)};",
    f"  --tex-grass: {svg(GRASS)};",
    f"  --tex-grass-dark: {svg(GRASS_DARK)};",
    f"  --tex-obsidian: {svg(OBSIDIAN)};",
    "}",
]
open("ui/textures.css", "w").write("\n".join(css) + "\n")

if len(sys.argv) > 1:
    out = sys.argv[1]
    for name, grid in [("erde", DIRT), ("erde_dunkel", DIRT_DARK), ("gras", GRASS), ("obsidian", OBSIDIAN)]:
        png(grid, f"{out}/{name}.png")
print("ok")
