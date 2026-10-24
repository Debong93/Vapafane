# shared – gemeinsame Datentypen und Sprachen

Datentypen, die das Programm (`src-tauri`) und die Oberfläche (`frontend`) austauschen, z. B. der Server-Status. Beide Seiten nutzen dieselben Strukturen; passt ein Feld nicht, fällt das schon beim Bauen auf.

Außerdem die Übersetzungen (`sprache.rs`, [Fluent](https://projectfluent.org/)): Die Texte liegen in [`../locales/`](../locales), eine `.ftl`-Datei je Sprache, und werden beim Bauen eingebettet. Deutsch und Englisch sind von Hand gepflegt, die übrigen Sprachen maschinell übersetzt. Neue Sprache: Datei `locales/<code>.ftl` anlegen und in `sprache.rs` bei `SPRACHEN` und `QUELLEN` eintragen – die Tests prüfen, dass alle Sprachen dieselben Schlüssel haben und fehlerfrei lesbar sind.
