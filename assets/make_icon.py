"""Erzeugt das Programm-Symbol (assets/icon.png, 1024x1024) als Pixel-Block."""
from PIL import Image, ImageDraw

S = 1024
img = Image.new("RGBA", (S, S), (0, 0, 0, 0))
d = ImageDraw.Draw(img)
# Block mit abgerundeten Ecken
d.rounded_rectangle([64, 64, S - 64, S - 64], radius=150, fill=(92, 64, 51, 255))
# Pixelraster 14x14 auf dem Block
grid = [
    "RRRRRRRRRRRRRR",
    "RrRRRRrRRRRrRR",
    "RRRRRRRRRRRRRR",
    "RRrRRRRRRrRRRR",
    "RRRRrRRRRRRRRR",
    "RRRRRRRRRRRRRR",
    "RRbRRRRbRRRRbR",
    "RbbbbbRbbbRbbb",
    "bbbbdbbbbbbbbb",
    "bbbbbbbbbdbbbb",
    "bdbbbbbbbbbbbb",
    "bbbbbbbdbbbbdb",
    "bbbbbbbbbbbbbb",
    "bbdbbbbbbbbbbb",
]
colors = {"R": (200, 40, 40), "r": (240, 80, 70), "b": (121, 85, 61), "d": (92, 62, 42)}
x0, y0, cell = 128, 128, (S - 256) // 14
for y, row in enumerate(grid):
    for x, c in enumerate(row):
        d.rectangle([x0 + x * cell, y0 + y * cell, x0 + (x + 1) * cell - 1, y0 + (y + 1) * cell - 1],
                    fill=colors[c] + (255,))
# Statuspunkt rechts unten
d.ellipse([S - 380, S - 380, S - 120, S - 120], fill=(46, 160, 67, 255), outline=(255, 255, 255, 255), width=28)
mask = Image.new("L", (S, S), 0)
ImageDraw.Draw(mask).rounded_rectangle([64, 64, S - 64, S - 64], radius=150, fill=255)
block = Image.composite(img, Image.new("RGBA", (S, S), (0, 0, 0, 0)), mask)
ImageDraw.Draw(block).ellipse([S - 380, S - 380, S - 120, S - 120], fill=(46, 160, 67, 255),
                              outline=(255, 255, 255, 255), width=28)
block.save(__import__("sys").argv[1])
