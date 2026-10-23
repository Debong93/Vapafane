//! Pixel-Texturen der Oberfläche (eigene Pixel-Art im 16×16-Stil, nichts von Mojang):
//! Erde, Grasblock-Kante (hell/dunkel) und Crying Obsidian – als SVG in CSS-Variablen.

use crate::pyrandom::PyRandom;

const N: usize = 16;
const SCALE: usize = 4;

type Grid = Vec<Vec<&'static str>>;

/// Wie Pythons `urllib.parse.quote(s, safe=" =:/'")`.
fn quote(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        let c = b as char;
        if c.is_ascii_alphanumeric() || "_.-~ =:/'".contains(c) {
            out.push(c);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn svg(grid: &Grid) -> String {
    let (h, w) = (grid.len(), grid.first().map(|r| r.len()).unwrap_or(0));
    let mut rects = String::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if !c.is_empty() {
                rects += &format!(
                    "<rect x='{}' y='{}' width='{SCALE}' height='{SCALE}' fill='{c}'/>",
                    x * SCALE,
                    y * SCALE
                );
            }
        }
    }
    let s = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' shape-rendering='crispEdges'>{rects}</svg>",
        w * SCALE,
        h * SCALE
    );
    format!("url(\"data:image/svg+xml,{}\")", quote(&s))
}

fn pick(r: &mut PyRandom, list: &[&'static str]) -> &'static str {
    r.choice(list).copied().unwrap_or("")
}

/// Kleine unregelmäßige Pixel-Gruppen (1–max_size Pixel) setzen.
fn clusters(
    r: &mut PyRandom,
    grid: &mut Grid,
    colors: &[&'static str],
    count: usize,
    max_size: i64,
) {
    let rows = grid.len() as i64;
    for _ in 0..count {
        let mut x = r.randrange(0, N as i64);
        let mut y = r.randrange(0, rows);
        let c = pick(r, colors);
        for _ in 0..r.randint(1, max_size) {
            let (yy, xx) = (y.rem_euclid(rows) as usize, x.rem_euclid(N as i64) as usize);
            if let Some(cell) = grid.get_mut(yy).and_then(|row| row.get_mut(xx)) {
                *cell = c;
            }
            x += [-1i64, 0, 1].get(r.randbelow_pub(3)).copied().unwrap_or(0);
            y += [0i64, 1].get(r.randbelow_pub(2)).copied().unwrap_or(0);
        }
    }
}

fn dirt(
    seed: u32,
    base: &'static str,
    darks: &[&'static str],
    lights: &[&'static str],
    rows: usize,
) -> Grid {
    let mut r = PyRandom::new(seed);
    let mut g = vec![vec![base; N]; rows];
    clusters(&mut r, &mut g, darks, 26, 3);
    clusters(&mut r, &mut g, lights, 16, 2);
    g
}

/// Obere Kante eines Grasblocks: Gras mit gezackten „Tropfen“ über Erde.
fn grass_side(seed: u32, greens: &[&'static str], dirt_rows: Grid) -> Grid {
    let mut r = PyRandom::new(seed);
    let mut g = dirt_rows;
    let choices = [3usize, 3, 4, 4, 5, 6];
    let depth: Vec<usize> = (0..N)
        .map(|_| r.choice(&choices).copied().unwrap_or(3))
        .collect();
    let last = greens.last().copied().unwrap_or("");
    for (x, d) in depth.iter().enumerate() {
        for y in 0..*d {
            let c = pick(&mut r, greens);
            if let Some(cell) = g.get_mut(y).and_then(|row| row.get_mut(x)) {
                *cell = c;
            }
        }
        // einzelne Gras-Pixel darunter für eine unruhige Kante
        if r.random() < 0.3
            && d + 1 < g.len()
            && let Some(cell) = g.get_mut(d + 1).and_then(|row| row.get_mut(x))
        {
            *cell = last;
        }
    }
    g
}

fn crying_obsidian(seed: u32) -> Grid {
    let mut r = PyRandom::new(seed);
    let base = ["#0d0717", "#110a1e", "#160c27", "#0a0512"];
    let mut g: Grid = (0..N)
        .map(|_| (0..N).map(|_| pick(&mut r, &base)).collect())
        .collect();
    // Obsidian-Schimmer: hellere violette Splitter
    clusters(&mut r, &mut g, &["#2a1747", "#22123a", "#3a2063"], 14, 3);
    // Tränen: senkrechte, leuchtende Spuren mit hellem Kern und Glimmen drumherum
    let n = N as i64;
    let set = |g: &mut Grid, y: i64, x: i64, c: &'static str| {
        if let Some(cell) = g
            .get_mut(y.rem_euclid(n) as usize)
            .and_then(|row| row.get_mut(x.rem_euclid(n) as usize))
        {
            *cell = c;
        }
    };
    for _ in 0..4 {
        let x = r.randrange(1, n - 1);
        let y = r.randrange(0, n - 6);
        let length = r.randint(3, 6);
        for i in 0..length {
            set(
                &mut g,
                y + i,
                x,
                if i < length - 1 { "#b45cff" } else { "#e3b8ff" },
            );
            for dx in [-1i64, 1] {
                if r.random() < 0.6 {
                    set(&mut g, y + i, x + dx, "#4a1f80");
                }
            }
        }
        set(&mut g, y + length, x, "#7a2fd0");
    }
    g
}

/// Inhalt von textures.css.
pub fn css() -> String {
    let dirt_l = dirt(
        11,
        "#7d573a",
        &["#5b3d27", "#644430", "#6c4a33"],
        &["#9a7150", "#a77d58"],
        N,
    );
    let dirt_d = dirt(
        11,
        "#4a3322",
        &["#33231a", "#3b291d", "#402c1f"],
        &["#5d412b", "#664a31"],
        N,
    );
    let greens = ["#5d9e37", "#6db342", "#4f8a2e", "#78c04c", "#457a28"];
    let greens_d = ["#3f7a24", "#4a8c2c", "#356a1e", "#559a33", "#2f5c1a"];
    let grass_l = grass_side(
        5,
        &greens,
        dirt(23, "#7d573a", &["#5b3d27", "#644430"], &["#9a7150"], 8),
    );
    let grass_d = grass_side(
        5,
        &greens_d,
        dirt(23, "#4a3322", &["#33231a", "#3b291d"], &["#5d412b"], 8),
    );
    let obsidian = crying_obsidian(42);
    [
        "/* Erzeugt von werkzeuge (cargo run --bin texturen) – eigene Pixel-Art im 16×16-Stil */"
            .to_string(),
        ":root {".into(),
        format!("  --tex-dirt: {};", svg(&dirt_l)),
        format!("  --tex-dirt-dark: {};", svg(&dirt_d)),
        format!("  --tex-grass: {};", svg(&grass_l)),
        format!("  --tex-grass-dark: {};", svg(&grass_d)),
        format!("  --tex-obsidian: {};", svg(&obsidian)),
        "}".into(),
    ]
    .join("\n")
        + "\n"
}

#[cfg(test)]
mod tests {
    /// Die eingecheckte textures.css muss genau dem entsprechen, was das Werkzeug erzeugt.
    #[test]
    fn textures_css_aktuell() {
        let file = include_str!("../../frontend/style/textures.css");
        assert_eq!(
            super::css(),
            file,
            "textures.css neu erzeugen: cargo run --bin texturen"
        );
    }
}
