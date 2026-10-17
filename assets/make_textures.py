"""Erzeugt die Pixel-Texturen (Erde, Gras) als CSS-Variablen für ui/textures.css.
Eigene Zeichnung aus Zufallspixeln – keine Original-Texturen von Mojang."""
import random, urllib.parse

def svg(w, h, cells, size=4):
    rects = "".join(f'<rect x="{x*size}" y="{y*size}" width="{size}" height="{size}" fill="{c}"/>' for (x, y, c) in cells)
    s = f'<svg xmlns="http://www.w3.org/2000/svg" width="{w*size}" height="{h*size}" shape-rendering="crispEdges">{rects}</svg>'
    s = s.replace('"', "'")
    return 'url("data:image/svg+xml,' + urllib.parse.quote(s, safe=" =:/'") + '")'

def dirt(seed, palette):
    r = random.Random(seed)
    return [(x, y, r.choice(palette)) for y in range(16) for x in range(16)]

def grass(seed, greens, dirts):
    r = random.Random(seed)
    cells = []
    hang = [r.choice([1, 1, 2, 2, 3]) for _ in range(16)]  # wie weit das Gras herunterhängt
    for x in range(16):
        for y in range(5):
            if y < 2 + hang[x]:
                cells.append((x, y, r.choice(greens)))
            else:
                cells.append((x, y, r.choice(dirts)))
    return cells

out = [":root {"]
out.append("  --tex-dirt: " + svg(16, 16, dirt(7, ["#8b5d3a", "#7b5031", "#96673f", "#6f462a", "#a2724a", "#865837"])) + ";")
out.append("  --tex-grass: " + svg(16, 5, grass(3, ["#5aa53a", "#4c9431", "#67b544", "#448a2b"], ["#8b5d3a", "#7b5031", "#96673f"])) + ";")
out.append("  --tex-grass-dark: " + svg(16, 5, grass(3, ["#3f7f28", "#356f22", "#4a8e30", "#2f6520"], ["#5d3f27", "#523621", "#66462b"])) + ";")
out.append("}")
open("ui/textures.css", "w").write("/* Erzeugt von assets/make_textures.py – eigene Pixel-Grafik */\n" + "\n".join(out) + "\n")
print("ok")
