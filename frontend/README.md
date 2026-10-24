# frontend – die Vapafane-Oberfläche

Bedienoberfläche in Rust mit [Leptos](https://leptos.dev), übersetzt zu WebAssembly und im Tauri-Fenster angezeigt. Sie zeigt nur an und ruft Befehle des Programms (`src-tauri`) auf; die eigentliche Arbeit macht das Programm.

- `src/pages/` – die Seiten (Übersicht, Konsole, Spieler, Mods, Updates, Backups, Erreichbarkeit, Einstellungen, Client) und die Anleitung „Shader & Extras“ (`extras.rs`)
- `src/dialog.rs`, `src/tasks.rs` – Dialoge und Fortschrittsleiste
- `style/` – Aussehen (CSS); `textures.css` wird von `werkzeuge` erzeugt

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk build --release        # Ergebnis in dist/ – Tauri bettet es beim Bauen ein
```
