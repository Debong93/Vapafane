"use strict";
// Oberfläche von Redcraftserver. Spricht über Tauri-Befehle mit dem Rust-Teil.

const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const $ = (id) => document.getElementById(id);
const esc = (s) => String(s ?? "").replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));

// ------------------------------------------------------------------ Formatierung

function fmtBytes(n) {
  if (n == null) return "–";
  const units = ["B", "KB", "MB", "GB", "TB"];
  let i = 0;
  while (n >= 1024 && i < units.length - 1) { n /= 1024; i++; }
  return i >= 3 ? `${n.toFixed(1).replace(".", ",")} ${units[i]}` : `${Math.round(n)} ${units[i]}`;
}

function fmtDuration(sec) {
  sec = Math.max(0, Math.floor(sec));
  const d = Math.floor(sec / 86400), h = Math.floor((sec % 86400) / 3600), m = Math.floor((sec % 3600) / 60), s = sec % 60;
  if (d) return `${d} T ${h} Std`;
  if (h) return `${h} Std ${m} Min`;
  if (m) return `${m} Min ${s} Sek`;
  return `${s} Sek`;
}

const num = (x, digits = 1) => Number(x).toFixed(digits).replace(".", ",");
const clock = () => new Date().toLocaleTimeString("de-DE", { hour: "2-digit", minute: "2-digit" });

// ------------------------------------------------------------------ Dialoge

let modalResolve = null;

/** Zeigt einen Dialog. buttons: [{label, value, kind}] – liefert value des Knopfs. */
function ask(title, body, buttons, opts = {}) {
  return new Promise((resolve) => {
    $("modal-title").textContent = title;
    const b = $("modal-body");
    if (opts.html) b.innerHTML = body; else b.textContent = body || "";
    const bar = $("modal-buttons");
    bar.innerHTML = "";
    for (const btn of buttons) {
      const el = document.createElement("button");
      el.textContent = btn.label;
      if (btn.kind) el.className = btn.kind;
      if (btn.id) el.id = btn.id;
      el.onclick = () => close(btn.value);
      bar.appendChild(el);
    }
    const close = (v) => {
      $("modal-back").classList.add("hidden");
      modalResolve = null;
      resolve(v);
    };
    modalResolve = close;
    document.querySelector("#modal-back .modal").classList.toggle("wide", !!opts.wide);
    $("modal-back").classList.remove("hidden");
    if (opts.onOpen) opts.onOpen();
    const def = bar.querySelector("button.primary") || bar.lastElementChild;
    if (def && !opts.noFocus) def.focus();
  });
}

const info = (title, text) => ask(title, text, [{ label: "OK", value: true, kind: "primary" }]);
const error = (title, text) => ask(title, String(text), [{ label: "Schließen", value: true, kind: "primary" }]);
const confirmAsk = (title, text, yes = "Ja") =>
  ask(title, text, [{ label: "Abbrechen", value: false }, { label: yes, value: true, kind: "primary" }]);

document.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && modalResolve) modalResolve(false);
});

/** Fragt nach der Minecraft-EULA. */
async function askEula(name) {
  const body = `<p>Ohne Zustimmung zur Minecraft-EULA startet kein Server.</p>
    <label class="check"><input type="checkbox" id="eula-check"> Ich akzeptiere die
    <a href="#" id="eula-link">Minecraft-EULA</a> für „${esc(name)}“.</label>`;
  const ok = await ask("EULA bestätigen", body,
    [{ label: "Abbrechen", value: false }, { label: "Weiter", value: true, kind: "primary", id: "eula-ok" }],
    {
      html: true, noFocus: true, onOpen: () => {
        $("eula-ok").disabled = true;
        $("eula-check").onchange = (e) => ($("eula-ok").disabled = !e.target.checked);
        $("eula-link").onclick = (e) => { e.preventDefault(); invoke("open_eula").catch(() => {}); };
      },
    });
  return ok === true;
}

// ------------------------------------------------------------------ Tabs und Design

for (const btn of document.querySelectorAll("#tabs button")) {
  btn.onclick = () => showTab(btn.dataset.tab);
}

function showTab(name) {
  for (const b of document.querySelectorAll("#tabs button")) b.classList.toggle("active", b.dataset.tab === name);
  for (const s of document.querySelectorAll(".tab")) s.classList.toggle("active", s.id === `tab-${name}`);
  if (name === "console") scrollConsole(true);
  if (name === "backups") refreshBackups();
  if (name === "players") refreshPlayersTab();
  if (name === "mods") refreshMods();
  if (name === "settings") loadSettings();
  if (name === "updates") refreshUpdateTab();
}

function applyTheme() {
  let t = null;
  try { t = localStorage.getItem("theme"); } catch (_) { /* egal */ }
  if (t) document.documentElement.dataset.theme = t; else delete document.documentElement.dataset.theme;
}
$("btn-theme").onclick = () => {
  const dark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const cur = document.documentElement.dataset.theme || (dark ? "dark" : "light");
  try { localStorage.setItem("theme", cur === "dark" ? "light" : "dark"); } catch (_) { /* egal */ }
  applyTheme();
};
applyTheme();

// ------------------------------------------------------------------ Ereignisse

function addEvent(text) {
  const ul = $("events");
  if (ul.firstElementChild && ul.firstElementChild.classList.contains("dim")) ul.innerHTML = "";
  const li = document.createElement("li");
  li.innerHTML = `<span class="t">${clock()}</span>${esc(text)}`;
  ul.prepend(li);
  while (ul.children.length > 50) ul.lastElementChild.remove();
}

const notify = (title, body = "") => invoke("notify", { title, body }).catch(() => {});

// ------------------------------------------------------------------ Live-Kurven

class Spark {
  constructor(id, color, max = null, len = 150) {
    this.el = $(id); this.color = color; this.max = max; this.len = len; this.values = [];
  }
  push(v) { this.values.push(v); if (this.values.length > this.len) this.values.shift(); this.draw(); }
  clear() { this.values = []; this.draw(); }
  draw() {
    const vals = this.values;
    const color = getComputedStyle(document.documentElement).getPropertyValue(this.color).trim();
    if (vals.length < 2) { this.el.innerHTML = ""; return; }
    const top = this.max || Math.max(...vals, 1e-9);
    const step = 150 / (this.len - 1);
    const x0 = 150 - (vals.length - 1) * step;
    const pts = vals.map((v, i) => `${(x0 + i * step).toFixed(1)},${(32 - (Math.min(v, top) / top) * 30).toFixed(1)}`);
    this.el.innerHTML = `<polygon fill="${color}" points="${x0},34 ${pts.join(" ")} 150,34"/>` +
      `<polyline stroke="${color}" points="${pts.join(" ")}"/>`;
  }
}

const sparks = { tps: new Spark("s-tps", "--green", 20), cpu: new Spark("s-cpu", "--blue", 100), ram: new Spark("s-ram", "--amber") };

// ------------------------------------------------------------------ Status

const STATUS = {
  running: ["Läuft", "--green"],
  starting: ["Startet …", "--amber"],
  stopping: ["Stoppt …", "--amber"],
  restarting: ["Neustart nach Absturz …", "--amber"],
  stopped: ["Gestoppt", "--gray"],
  failed: ["Abgestürzt", "--red"],
  external: ["Läuft außerhalb von Redcraftserver", "--amber"],
  other: ["Ein anderer Server läuft", "--amber"],
  none: ["Kein Server ausgewählt", "--gray"],
};

let status = { state: "stopped" };
let lastState = null;
let live = { players: [] };
let serverPath = null;
let busy = null;

async function refreshStatus() {
  let st;
  try { st = await invoke("get_status"); } catch (e) { console.error(e); return; }
  status = st;
  busy = st.busy;
  const state = st.state;
  const [label, color] = STATUS[state] || [state, "--gray"];

  // Server-Auswahl
  const sel = $("server-select");
  const key = JSON.stringify(st.servers.map((s) => [s.name, s.path, s.active]));
  if (sel.dataset.key !== key) {
    sel.dataset.key = key;
    sel.innerHTML = st.servers.length ? "" : `<option value="">(kein Server)</option>`;
    for (const s of st.servers) {
      const o = document.createElement("option");
      o.value = s.path; o.textContent = s.name; o.selected = s.active;
      sel.appendChild(o);
    }
  }
  const newPath = st.server ? st.server.path : null;
  if (newPath !== serverPath) {
    serverPath = newPath;
    onServerChanged();
  }

  // Übergänge melden
  if (lastState && state !== lastState) {
    if ((state === "failed" || state === "restarting") && !["failed", "restarting"].includes(lastState)) {
      addEvent("Server ist abgestürzt" + (state === "restarting" ? " – wird automatisch neu gestartet" : ""));
      notify("Minecraft-Server abgestürzt", state === "restarting" ? "Er wird automatisch neu gestartet." : "");
    } else if (state === "running" && lastState !== "running") {
      addEvent("Server ist bereit");
      refreshLive();
    } else if (state === "stopped" && lastState !== "none") {
      addEvent("Server gestoppt");
    }
  }
  lastState = state;

  const dot = $("status-dot");
  dot.style.background = `var(${color})`;
  dot.classList.toggle("pulse", ["starting", "stopping", "restarting"].includes(state));
  $("status-label").textContent = label;
  document.title = st.server ? `${st.server.name} – ${label} · Redcraftserver` : "Redcraftserver";

  const sub = [];
  if (st.uptime && ["running", "starting", "stopping", "external"].includes(state)) sub.push(`seit ${fmtDuration(st.uptime)}`);
  if (st.restarts) sub.push(`${st.restarts}× automatisch neu gestartet`);
  if (st.countdown) sub.push(st.countdown);
  if (busy) sub.push(`${busy} …`);
  else if (st.job) sub.push(`${st.job} …`);
  $("status-sub").textContent = sub.join("  ·  ");

  const up = ["running", "starting", "stopping", "restarting"].includes(state);
  const free = !busy && !st.job;
  $("btn-start").disabled = !(["stopped", "failed"].includes(state) && free);
  $("btn-stop").disabled = !((up && state !== "stopping" && !st.countdown) || state === "external") || !free;
  $("btn-restart").disabled = !(state === "running" && !st.countdown && free);
  $("btn-backup").disabled = $("btn-backup2").disabled = !st.server || !free;
  $("cmd-input").disabled = $("btn-send").disabled = !["running", "external"].includes(state);
  sel.disabled = !free;

  // CPU / RAM
  if (st.pid) {
    if (st.cpu != null) {
      $("c-cpu").textContent = `${Math.round(st.cpu)} %`;
      $("c-cpu-sub").textContent = `von ${st.ncpu} Kernen`;
      sparks.cpu.push(st.cpu);
    }
    if (st.ram != null) {
      $("c-ram").textContent = fmtBytes(st.ram);
      $("c-ram-sub").textContent = st.ram_limit_gb ? `belegt (Limit ${st.ram_limit_gb} GB)` : "belegt";
      sparks.ram.max = st.ram_limit_gb || null;
      sparks.ram.push(st.ram / 1024 ** 3);
    }
  } else {
    $("c-cpu").textContent = "–"; $("c-cpu-sub").textContent = "";
    $("c-ram").textContent = "–"; $("c-ram-sub").textContent = st.ram_limit_gb ? `Limit ${st.ram_limit_gb} GB` : "";
  }

  // Hinweise
  const hint = $("overview-hint");
  let h = "", isErr = false;
  if (state === "external") h = "Der Server wurde nicht über Redcraftserver gestartet (z. B. altes Bedienfenster oder Terminal). Konsole und Spieler gehen trotzdem; „Stoppen“ schickt ihm den stop-Befehl.";
  else if (state === "other") h = "Der Hintergrund-Dienst betreibt gerade einen anderen Server. Bitte den zuerst stoppen.";
  else if (state === "failed") { h = "Der Server ist abgestürzt und wurde nicht mehr neu gestartet. Schau in der Konsole nach der letzten Fehlermeldung." + (st.last_exit ? `\n\n${st.last_exit}` : ""); isErr = true; }
  else if (state === "none") h = "Füge oben über „＋“ einen Server hinzu.";
  else if (state === "restarting" && st.last_exit) { h = st.last_exit; isErr = true; }
  hint.textContent = h;
  hint.classList.toggle("hidden", !h);
  hint.classList.toggle("error", isErr);
}

async function refreshLive() {
  if (!["running", "external", "starting"].includes(status.state)) {
    if (live.players.length) { live = { players: [] }; renderOnline(); }
    $("c-players").textContent = "–";
    $("c-players-sub").textContent = status.state === "stopped" ? "Server aus" : "";
    $("c-tps").textContent = "–"; $("c-tps-sub").textContent = "";
    return;
  }
  try {
    const l = await invoke("get_live");
    const changed = JSON.stringify(l.players) !== JSON.stringify(live.players) || JSON.stringify(l.ops) !== JSON.stringify(live.ops);
    live = { ...l, players: l.players || [] };
    if (l.online != null) {
      $("c-players").textContent = `${l.online} / ${l.max}`;
      $("c-players-sub").textContent = live.players.join(", ") || "niemand";
    }
    if (l.tps != null) {
      $("c-tps").textContent = num(l.tps);
      $("c-tps-sub").textContent = `${num(l.mspt)} ms pro Tick  ·  ${l.tps < 18 ? "ruckelt" : "flüssig"}`;
      sparks.tps.push(l.tps);
    }
    if (changed) renderOnline();
  } catch (_) {
    $("c-players").textContent = "–";
    $("c-players-sub").textContent = status.state === "starting" ? "Server startet …" : "keine Verbindung";
  }
}

async function refreshDetails() {
  let d;
  try { d = await invoke("get_details"); } catch (_) { return; }
  if (!d) {
    for (const id of ["c-size", "c-addr", "c-backup", "c-version"]) { $(id).textContent = "–"; $(`${id}-sub`).textContent = ""; }
    return;
  }
  $("c-size").textContent = fmtBytes(d.size);
  $("c-size-sub").textContent = `${fmtBytes(d.free)} frei`;
  $("c-addr").textContent = d.ip;
  $("c-addr-sub").textContent = `Port ${d.port}`;
  if (d.last_backup) {
    $("c-backup").textContent = `vor ${fmtDuration(d.last_backup.age)}`;
    $("c-backup-sub").textContent = `${fmtBytes(d.last_backup.size)} · ${d.backups} gespeichert`;
  } else {
    $("c-backup").textContent = "noch keins";
    $("c-backup-sub").textContent = "Knopf „Backup“ drücken";
  }
  const i = d.info;
  $("c-version").textContent = i.version || "?";
  $("c-version-sub").textContent = (i.loader ? `Fabric ${i.loader}` : i.type) + (d.mods ? ` · ${d.mods} Mods` : "");
  details = d;
}
let details = null;

function onServerChanged() {
  for (const s of Object.values(sparks)) s.clear();
  live = { players: [] };
  renderOnline();
  invoke("log_reset");
  refreshDetails();
  refreshBackups();
  const active = document.querySelector("#tabs button.active").dataset.tab;
  if (active === "settings") loadSettings();
  if (active === "players") refreshPlayersTab();
  if (active === "mods") refreshMods();
  if (active === "updates") refreshUpdateTab();
}

// ------------------------------------------------------------------ Start / Stopp

$("btn-start").onclick = startServer;

async function startServer() {
  try {
    await invoke("start_server");
    addEvent("Server wird gestartet");
  } catch (e) {
    if (String(e) === "EULA") {
      if (!(await askEula(status.server?.name || ""))) return;
      try { await invoke("accept_eula"); } catch (e2) { return error("EULA", e2); }
      return startServer();
    }
    return error("Start fehlgeschlagen", e);
  }
  setTimeout(refreshStatus, 400);
}

async function askStop(restart) {
  const verb = restart ? "neu starten" : "stoppen";
  const secs = (await invoke("get_config")).stop_warning || 30;
  if (status.state === "external") {
    if (!(await confirmAsk("Server stoppen?", "Der Server läuft außerhalb von Redcraftserver. Ich schicke ihm den Befehl „stop“ – er speichert die Welt und beendet sich.", "Stoppen"))) return;
    return doStop(false, 0);
  }
  const players = live.players || [];
  let choice;
  if (players.length) {
    choice = await ask(`Server ${verb}?`, `${players.length} Spieler online: ${players.join(", ")}`, [
      { label: "Abbrechen", value: null },
      { label: "Sofort", value: 0 },
      { label: `In ${secs} Sek. mit Warnung`, value: secs, kind: "primary" },
    ]);
  } else {
    choice = await ask(`Server ${verb}?`, "Gerade ist niemand online.", [
      { label: "Abbrechen", value: null },
      { label: verb[0].toUpperCase() + verb.slice(1), value: 0, kind: "primary" },
    ]);
  }
  if (choice === null || choice === false || choice === undefined) return;
  doStop(restart, choice);
}

async function doStop(restart, warn) {
  addEvent(`Server ${restart ? "startet neu" : "stoppt"}${warn ? ` in ${warn} Sek.` : ""}`);
  try { await invoke("stop_server", { warn, restart }); } catch (e) { error("Stoppen fehlgeschlagen", e); }
  setTimeout(refreshStatus, 400);
}

$("btn-stop").onclick = () => askStop(false);
$("btn-restart").onclick = () => askStop(true);

// ------------------------------------------------------------------ Konsole

const consoleEl = $("console");
const JOIN_RE = /\]: (\w{3,16}) (joined|left) the game/;
const CHAT_RE = /\]: <(\w{3,16})> /;
const DONE_RE = /\]: Done \(([\d.,]+)s\)!/;

function atBottom() {
  return consoleEl.scrollTop + consoleEl.clientHeight >= consoleEl.scrollHeight - 30;
}
function scrollConsole(force) {
  if (force) consoleEl.scrollTop = consoleEl.scrollHeight;
}

function appendConsole(lines, cls) {
  const stick = atBottom();
  const frag = document.createDocumentFragment();
  for (const line of lines) {
    const div = document.createElement("div");
    div.textContent = line;
    const c = cls || classify(line);
    if (c) div.className = c;
    frag.appendChild(div);
  }
  consoleEl.appendChild(frag);
  while (consoleEl.childElementCount > 3000) consoleEl.firstElementChild.remove();
  if (stick) consoleEl.scrollTop = consoleEl.scrollHeight;
}

function classify(line) {
  if (line.includes("/ERROR]") || line.includes("Exception")) return "error";
  if (JOIN_RE.test(line)) return "join";
  if (CHAT_RE.test(line)) return "chat";
  if (line.includes("/WARN]")) return "warn";
  return null;
}

function logEvents(lines) {
  for (const line of lines) {
    const m = JOIN_RE.exec(line);
    if (m) {
      const text = m[2] === "joined" ? `${m[1]} ist beigetreten` : `${m[1]} hat das Spiel verlassen`;
      addEvent(text);
      notify(text);
      setTimeout(refreshLive, 800);
      continue;
    }
    const d = DONE_RE.exec(line);
    if (d) {
      addEvent(`Server bereit (Start in ${d[1].replace(".", ",")} s)`);
      setTimeout(refreshLive, 500);
    }
  }
}

listen("log", (ev) => {
  const { reset, initial, lines } = ev.payload;
  if (reset && initial) consoleEl.innerHTML = "";
  if (reset && !initial) appendConsole(["––– neue Log-Datei (Server wurde neu gestartet) –––"], "sys");
  appendConsole(lines);
  if (!initial) logEvents(lines);
  if (initial) scrollConsole(true);
});

const history = [];
let histPos = 0;

async function runCommand(cmd, quiet = false) {
  if (!quiet) appendConsole([`> ${cmd}`], "cmd");
  try {
    const resp = await invoke("send_command", { cmd });
    if (!quiet && resp) appendConsole(resp.split("\n"), "resp");
    return resp;
  } catch (e) {
    if (!quiet) appendConsole([`Fehler: ${e}`], "error");
    throw e;
  }
}

function sendInput() {
  const cmd = $("cmd-input").value.trim();
  if (!cmd) return;
  history.push(cmd);
  histPos = history.length;
  $("cmd-input").value = "";
  runCommand(cmd).catch(() => {});
  scrollConsole(true);
}

$("btn-send").onclick = sendInput;
$("cmd-input").addEventListener("keydown", (e) => {
  if (e.key === "Enter") { sendInput(); return; }
  if (e.key === "ArrowUp" && history.length) histPos = Math.max(0, histPos - 1);
  else if (e.key === "ArrowDown" && history.length) histPos = Math.min(history.length, histPos + 1);
  else return;
  e.preventDefault();
  $("cmd-input").value = history[histPos] ?? "";
});
for (const b of document.querySelectorAll(".quick button[data-cmd]")) {
  b.onclick = () => runCommand(b.dataset.cmd).catch(() => {});
}
$("btn-open-logs").onclick = () => invoke("open_folder", { which: "logs" }).catch((e) => error("Ordner", e));

// ------------------------------------------------------------------ Spieler

function renderOnline() {
  const box = $("online-list");
  const players = live.players || [];
  if (!players.length) { box.innerHTML = `<div class="empty">Niemand online</div>`; return; }
  const ops = live.ops || [];
  box.innerHTML = "";
  for (const name of players) {
    const isOp = ops.includes(name.toLowerCase());
    const row = document.createElement("div");
    row.className = "item";
    row.innerHTML = `<div class="grow">${esc(name)}${isOp ? '<span class="badge">OP</span>' : ""}</div>`;
    for (const [label, cmd] of [
      ["Kreativ", `gamemode creative ${name}`],
      ["Überleben", `gamemode survival ${name}`],
      [isOp ? "OP entziehen" : "OP geben", `${isOp ? "deop" : "op"} ${name}`],
      ["Rauswerfen", `kick ${name}`],
    ]) {
      const b = document.createElement("button");
      b.textContent = label;
      b.onclick = async () => {
        if (label === "Rauswerfen" && !(await confirmAsk(`${name} rauswerfen?`, "Er kann gleich wieder beitreten.", "Rauswerfen"))) return;
        try { await runCommand(cmd); } catch (e) { error("Befehl fehlgeschlagen", e); }
        setTimeout(refreshLive, 500);
      };
      row.appendChild(b);
    }
    box.appendChild(row);
  }
}

async function refreshPlayersTab() {
  renderOnline();
  let names = [];
  try { names = await invoke("get_whitelist"); } catch (_) { /* noch nicht da */ }
  const box = $("wl-list");
  box.innerHTML = names.length ? "" : `<div class="empty">Die Whitelist ist leer.</div>`;
  for (const name of names) {
    const row = document.createElement("div");
    row.className = "item";
    row.innerHTML = `<div class="grow">${esc(name)}</div>`;
    const b = document.createElement("button");
    b.textContent = "Entfernen";
    b.onclick = () => whitelistRemove(name);
    row.appendChild(b);
    box.appendChild(row);
  }
}

async function whitelistAdd() {
  const name = $("wl-input").value.trim();
  if (!/^\w{3,16}$/.test(name)) { $("wl-hint").textContent = "Bitte einen gültigen Minecraft-Namen eingeben (3–16 Zeichen)."; return; }
  if (!["running", "external"].includes(status.state)) {
    $("wl-hint").textContent = "Hinzufügen geht nur, während der Server läuft (er prüft den Namen bei Mojang).";
    return;
  }
  $("wl-input").value = "";
  try { $("wl-hint").textContent = await runCommand(`whitelist add ${name}`, true); } catch (e) { $("wl-hint").textContent = String(e); }
  setTimeout(refreshPlayersTab, 700);
}

async function whitelistRemove(name) {
  try {
    if (["running", "external"].includes(status.state)) $("wl-hint").textContent = await runCommand(`whitelist remove ${name}`, true);
    else { await invoke("whitelist_remove_offline", { name }); $("wl-hint").textContent = `${name} entfernt.`; }
  } catch (e) { $("wl-hint").textContent = `Fehler: ${e}`; }
  setTimeout(refreshPlayersTab, 700);
}

$("btn-wl-add").onclick = whitelistAdd;
$("wl-input").addEventListener("keydown", (e) => { if (e.key === "Enter") whitelistAdd(); });

// ------------------------------------------------------------------ Aufgaben mit Fortschritt

const taskWaiters = [];

/** Startet eine Aufgabe im Rust-Teil und wartet auf ihr Ende. */
async function task(cmd, args = {}) {
  await invoke(cmd, args);
  $("taskbar").classList.remove("hidden");
  $("task-bar").style.width = "0";
  $("task-detail").textContent = "";
  refreshStatus();
  return new Promise((resolve, reject) => taskWaiters.push({ resolve, reject }));
}

listen("task", (ev) => {
  const t = ev.payload;
  $("taskbar").classList.remove("hidden");
  $("task-title").textContent = t.title;
  if (t.frac != null) $("task-bar").style.width = `${Math.round(t.frac * 100)}%`;
  if (t.text) $("task-detail").textContent = t.text;
  if (t.done) {
    $("taskbar").classList.add("hidden");
    const w = taskWaiters.shift();
    if (w) { if (t.error) w.reject(t.error); else w.resolve(t.result); }
    refreshStatus(); refreshDetails(); refreshBackups();
  }
});

// ------------------------------------------------------------------ Backups

async function refreshBackups() {
  let b;
  try { b = await invoke("list_backups"); } catch (_) { $("backup-list").innerHTML = ""; $("backup-info").textContent = ""; return; }
  $("backup-info").textContent = `Jedes Backup ist der komplette Server-Ordner als ZIP, mit Datum im Namen. Gespeichert in ${b.dir}. Die neuesten ${b.keep} bleiben, ältere werden gelöscht.`;
  if (document.activeElement !== $("backup-keep")) $("backup-keep").value = b.keep;
  const box = $("backup-list");
  box.innerHTML = b.items.length ? "" : `<div class="empty">Noch keine Backups für diesen Server.</div>`;
  for (const it of b.items) {
    const row = document.createElement("div");
    row.className = "item";
    row.innerHTML = `<div class="grow"><div>${esc(it.label)}</div><div class="sub">${esc(it.file)} · ${fmtBytes(it.size)}</div></div>`;
    const btn = document.createElement("button");
    btn.textContent = "Wiederherstellen";
    btn.onclick = () => restore(it.path, it.file);
    row.appendChild(btn);
    box.appendChild(row);
  }
}

async function backupNow() {
  addEvent("Backup gestartet");
  try {
    const r = await task("backup_now");
    addEvent(`Backup fertig: ${r.path.split(/[\\/]/).pop()} (${fmtBytes(r.size)})`);
  } catch (e) { error("Backup fehlgeschlagen", e); }
}

async function restore(path, file) {
  const name = status.server?.name || "";
  if (!(await confirmAsk(`„${name}“ auf ${file} zurücksetzen?`,
    "Der Server wird dafür gestoppt. Der jetzige Stand wird nicht gelöscht, sondern bleibt als Ordner „…_vor_Wiederherstellung_<Datum>“ daneben liegen.", "Wiederherstellen"))) return;
  try {
    const r = await task("restore_backup", { path });
    addEvent("Backup wiederhergestellt");
    info("Wiederhergestellt", `Der Server ist auf dem Stand von ${r.label}. Du kannst ihn jetzt starten.` +
      (r.aside ? `\n\nDer vorherige Stand liegt in:\n${r.aside}\n(kann gelöscht werden, wenn alles passt)` : ""));
  } catch (e) { error("Wiederherstellen fehlgeschlagen", e); }
  invoke("log_reset");
}

$("btn-backup").onclick = backupNow;
$("btn-backup2").onclick = backupNow;
$("btn-backup-folder").onclick = () => invoke("open_folder", { which: "backups" }).catch((e) => error("Ordner", e));
$("btn-restore-file").onclick = async () => {
  const b = await invoke("list_backups").catch(() => null);
  const path = await invoke("pick_archive", { title: "Backup wählen", start: b ? b.dir : null });
  if (path) restore(path, path.split(/[\\/]/).pop());
};
$("backup-keep").onchange = async (e) => {
  const keep = Math.max(1, parseInt(e.target.value, 10) || 5);
  await invoke("set_backup_keep", { keep }).catch((er) => error("Speichern", er));
  refreshBackups();
};

// ------------------------------------------------------------------ Server-Auswahl

$("server-select").onchange = async (e) => {
  const path = e.target.value;
  if (!path || (status.server && path === status.server.path)) return;
  try {
    await invoke("select_server", { path });
    addEvent(`Server gewechselt: ${e.target.selectedOptions[0].textContent}`);
  } catch (er) {
    error("Umschalten nicht möglich", er);
  }
  e.target.dataset.key = "";
  refreshStatus();
};

const addMenu = $("add-menu");
$("btn-add-server").onclick = (e) => { e.stopPropagation(); addMenu.classList.toggle("hidden"); };
document.addEventListener("click", () => addMenu.classList.add("hidden"));
for (const b of addMenu.querySelectorAll("button")) {
  b.onclick = () => {
    addMenu.classList.add("hidden");
    ({ folder: addFolder, archive: addArchive, new: addNew, remove: removeServer })[b.dataset.add]();
  };
}

/** Dialog: Name (und evtl. Version), EULA-Häkchen. Liefert {name, version, eula} oder null. */
const KINDS = [["fabric", "Fabric (Mods)"], ["paper", "Paper (Plugins)"], ["neoforge", "NeoForge (Mods)"], ["vanilla", "Vanilla (ohne Mods)"]];

/** Dialog: Name, evtl. Server-Typ + Version, evtl. Ordner, EULA-Häkchen.
 *  Liefert {name, kind, version, target, eula} oder null. */
async function askServerDetails(title, name, { needEula = false, kinds = false, folder = false } = {}) {
  let html = `<label for="f-name">Name in der Server-Liste</label><input id="f-name" value="${esc(name)}">`;
  if (kinds) {
    html += `<label for="f-kind">Server-Typ</label><select id="f-kind">` +
      KINDS.map(([k, l]) => `<option value="${k}">${esc(l)}</option>`).join("") + `</select>` +
      `<label for="f-version">Minecraft-Version</label><select id="f-version"><option value="">lädt …</option></select>`;
  }
  if (folder) {
    html += `<label>Ordner</label><div class="row" style="margin:0"><code id="f-dir" class="grow" style="word-break:break-all"></code>` +
      `<button id="f-dir-btn" type="button">Anderer Ordner …</button></div><div id="f-dir-hint" class="dim small"></div>`;
  }
  if (needEula) {
    html += `<label class="check"><input type="checkbox" id="f-eula"> Ich akzeptiere die
      <a href="#" id="f-eula-link">Minecraft-EULA</a> (ohne startet kein Server)</label>`;
  }
  let customDir = null; // vom Nutzer gewählter Ordner (sonst Vorschlag aus dem Namen)
  const ok = await ask(title, html, [{ label: "Abbrechen", value: false }, { label: "Weiter", value: true, kind: "primary", id: "f-ok" }], {
    html: true, noFocus: true, onOpen: () => {
      const okBtn = $("f-ok");
      const check = () => {
        okBtn.disabled = !$("f-name").value.trim() || (needEula && !$("f-eula").checked) ||
          (kinds && !$("f-version").value) || (folder && !$("f-dir").textContent);
      };
      const suggest = async () => {
        if (!folder || customDir) return;
        $("f-dir").textContent = await invoke("suggest_dir", { name: $("f-name").value.trim() || "Server" }).catch(() => "");
        check();
      };
      $("f-name").oninput = () => { check(); suggest(); };
      if (kinds) {
        const loadVersions = async () => {
          const sel = $("f-version");
          sel.innerHTML = `<option value="">lädt …</option>`;
          check();
          try {
            const v = await invoke("server_versions", { kind: $("f-kind").value });
            sel.innerHTML = v.slice(0, 40).map((x) => `<option>${esc(x)}</option>`).join("");
          } catch (e) { sel.innerHTML = `<option value="">Keine Verbindung: ${esc(e)}</option>`; }
          check();
        };
        $("f-kind").onchange = loadVersions;
        $("f-version").onchange = check;
        loadVersions();
      }
      if (folder) {
        suggest();
        $("f-dir-btn").onclick = async () => {
          const picked = await invoke("pick_folder", { title: "Ordner für den Server wählen" });
          if (!picked) return;
          const c = await invoke("check_dir", { path: picked, name: $("f-name").value.trim() || "Server" });
          if (c.state === "nonempty") {
            customDir = c.suggestion;
            $("f-dir-hint").textContent = "Der gewählte Ordner ist nicht leer – der Server kommt in einen neuen Unterordner.";
          } else if (c.state === "invalid") {
            $("f-dir-hint").textContent = "Dieser Ordner lässt sich nicht verwenden.";
            return;
          } else {
            customDir = picked;
            $("f-dir-hint").textContent = "";
          }
          $("f-dir").textContent = customDir;
          check();
        };
      }
      if (needEula) { $("f-eula").onchange = check; $("f-eula-link").onclick = (e) => { e.preventDefault(); invoke("open_eula").catch(() => {}); }; }
      $("f-name").onkeydown = (e) => { if (e.key === "Enter" && !okBtn.disabled) okBtn.click(); };
      check();
      $("f-name").focus();
    },
  });
  if (ok !== true) return null;
  // Der Dialog ist nur versteckt, die Felder sind noch da
  return {
    name: $("f-name").value.trim(),
    kind: kinds ? $("f-kind").value : null,
    version: kinds ? $("f-version").value : null,
    target: folder ? $("f-dir").textContent : null,
    eula: needEula ? $("f-eula").checked : false,
  };
}

async function afterAdded(r, name) {
  if (r.notes && r.notes.length) await info("Hinweis", r.notes.join("\n"));
  addEvent(`Server hinzugefügt: ${name}`);
  $("server-select").dataset.key = "";
  await refreshStatus();
  if (!r.activated) info("Server hinzugefügt", `„${name}“ ist in der Liste. Stoppe den laufenden Server, um umzuschalten.`);
}

async function addFolder() {
  const path = await invoke("pick_folder", { title: "Server-Ordner wählen" });
  if (!path) return;
  let insp;
  try { insp = await invoke("inspect_folder", { path }); } catch (e) { return error("Kein Server gefunden", e); }
  const d = await askServerDetails("Server hinzufügen", insp.name, { needEula: !insp.eula });
  if (!d) return;
  try { afterAdded(await invoke("add_server", { path: insp.root, name: d.name, eula: d.eula }), d.name); }
  catch (e) { error("Hinzufügen fehlgeschlagen", e); }
}

async function addArchive() {
  const archive = await invoke("pick_archive", { title: "Server-Backup (ZIP oder tar) wählen", start: null });
  if (!archive) return;
  let base = archive.split(/[\\/]/).pop().replace(/(\.tar)?\.(zip|gz|tgz|zst|tar)$/i, "");
  base = base.replace(/_\d{4}-\d{2}-\d{2}_\d{2}-\d{2}(-\d{2})?$/, "");
  const d = await askServerDetails("Server importieren", base, { folder: true });
  if (!d) return;
  let r;
  try { r = await task("import_server", { archive, name: d.name, target: d.target }); } catch (e) { return error("Import fehlgeschlagen", e); }
  let eula = r.eula;
  if (!eula) eula = await askEula(d.name);
  try { afterAdded(await invoke("add_server", { path: r.path, name: d.name, eula }), d.name); }
  catch (e) { error("Hinzufügen fehlgeschlagen", e); }
}

async function addNew() {
  const d = await askServerDetails("Neuen Server erstellen", "Neuer_Server", { needEula: true, kinds: true, folder: true });
  if (!d) return;
  let r;
  try { r = await task("create_server", { kind: d.kind, name: d.name, version: d.version, target: d.target }); }
  catch (e) { return error("Erstellen fehlgeschlagen", e); }
  try { afterAdded(await invoke("add_server", { path: r.path, name: d.name, eula: true }), d.name); }
  catch (e) { error("Hinzufügen fehlgeschlagen", e); }
}

async function removeServer() {
  const sel = $("server-select");
  const path = sel.value;
  if (!path) return;
  const name = sel.selectedOptions[0].textContent;
  if (!(await confirmAsk(`„${name}“ aus der Liste entfernen?`, `Die Dateien in ${path} bleiben erhalten, nur der Eintrag verschwindet.`, "Entfernen"))) return;
  try { await invoke("remove_server", { path }); } catch (e) { return error("Entfernen fehlgeschlagen", e); }
  sel.dataset.key = "";
  refreshStatus();
}

// ------------------------------------------------------------------ Einstellungen

let settings = null;

async function loadSettings() {
  try { settings = await invoke("get_settings"); } catch (e) { return; }
  const s = settings;
  for (const el of document.querySelectorAll("[data-prop]")) {
    const v = s.props ? s.props[el.dataset.prop] ?? "" : "";
    if (el.type === "checkbox") el.checked = v === "true"; else el.value = v;
    el.disabled = !s.props;
  }
  const totalGb = Math.round(s.total_memory / 1024 ** 3);
  $("set-ram").max = Math.max(2, totalGb);
  $("set-ram").value = s.ram_gb ?? 4;
  $("ram-hint").textContent = `Der PC hat ${totalGb} GB. Lass etwa 3–4 GB für das System übrig.`;
  $("save-hint").textContent = "";

  $("sw-autostart").checked = s.autostart;
  $("sw-window").checked = s.window;
  $("sw-daily").checked = s.daily;
  $("sw-notify").checked = s.notify;
  const autoOk = s.systemd || s.windows;
  $("sw-autostart").disabled = !autoOk;
  let sub = "";
  if (!autoOk) sub = s.test_mode ? "Im Testmodus abgeschaltet." : "Auf diesem System nicht verfügbar (kein systemd).";
  else if (s.windows) sub = "Startet beim PC-Start, auch ohne Anmeldung (Windows fragt beim Einschalten nach Administratorrechten).";
  else if (s.autostart && !s.linger) sub = "Achtung: startet erst, wenn du dich anmeldest. Klick hier, um den Start ohne Anmeldung zu erlauben.";
  else sub = "Startet auch ohne Anmeldung und nach einem Absturz von selbst neu.";
  const subEl = $("autostart-sub");
  subEl.textContent = sub;
  subEl.style.cursor = s.autostart && !s.linger && s.systemd ? "pointer" : "";
  subEl.onclick = s.autostart && !s.linger && s.systemd ? askLinger : null;

  const box = $("old-panel-box");
  if (s.old_panel) {
    const o = s.old_panel;
    box.innerHTML = `<b>Altes Bedienfenster noch eingerichtet.</b> Dienst „minecraft.service“: ${o.active ? "läuft" : "aus"}, ` +
      `beim Start ${o.enabled ? "an" : "aus"}; tägliches Backup-Zeitprogramm ${o.timer ? "an" : "aus"}. ` +
      `Solange es läuft, kann Redcraftserver den Server nicht selbst starten (gleicher Port), und Backups würden doppelt gemacht. ` +
      `<div class="row"><button id="btn-old-off">Altes Panel abschalten …</button></div>`;
    box.classList.remove("hidden");
    $("btn-old-off").onclick = disableOldPanel;
  } else box.classList.add("hidden");
}

async function askLinger() {
  if (!(await confirmAsk("Start ohne Anmeldung erlauben?",
    "Dafür muss systemd deine Benutzer-Dienste schon beim Hochfahren starten dürfen („loginctl enable-linger“). Danach läuft der Server auch, wenn sich niemand anmeldet.", "Erlauben"))) return;
  try { await invoke("enable_linger"); } catch (e) { error("Nicht erlaubt", e); }
  loadSettings();
}

async function disableOldPanel() {
  if (!(await confirmAsk("Altes Panel abschalten?",
    "Ich stoppe den Dienst „minecraft.service“ (der Server speichert dabei sauber und beendet sich) und schalte ihn und sein tägliches Backup beim PC-Start aus.\n\nDas alte Panel selbst und seine Dateien bleiben erhalten – man kann es jederzeit wieder einschalten. Danach startest du den Server hier mit „Starten“.", "Abschalten"))) return;
  try { await invoke("disable_old_panel"); addEvent("Altes Panel abgeschaltet"); } catch (e) { error("Abschalten fehlgeschlagen", e); }
  loadSettings();
  refreshStatus();
}

$("btn-save-settings").onclick = async () => {
  const props = {};
  for (const el of document.querySelectorAll("[data-prop]")) props[el.dataset.prop] = el.type === "checkbox" ? String(el.checked) : el.value.trim();
  try {
    await invoke("save_settings", { props, ramGb: parseInt($("set-ram").value, 10) || 4 });
    $("save-hint").textContent = ["running", "starting"].includes(status.state) ? "Gespeichert. Wird beim nächsten Neustart aktiv." : "Gespeichert.";
  } catch (e) { error("Speichern fehlgeschlagen", e); }
  refreshDetails();
};

function bindSwitch(id, cmd, after) {
  $(id).onchange = async (e) => {
    const on = e.target.checked;
    e.target.disabled = true;
    try { await invoke(cmd, { on }); if (after) await after(on); } catch (er) { error("Einstellung", er); }
    e.target.disabled = false;
    loadSettings();
  };
}
bindSwitch("sw-autostart", "set_autostart", async (on) => { if (on && settings && settings.systemd && !settings.linger) await askLinger(); });
bindSwitch("sw-window", "set_window_autostart");
bindSwitch("sw-daily", "set_daily_backup");
bindSwitch("sw-notify", "set_notify");
for (const b of document.querySelectorAll("[data-open]")) {
  b.onclick = () => invoke("open_folder", { which: b.dataset.open }).catch((e) => error("Ordner", e));
}

// ------------------------------------------------------------------ Mods

function modRow(m) {
  const row = document.createElement("div");
  row.className = "item" + (m.enabled ? "" : " off");
  const sub = [m.version ? `Version ${esc(m.version)}` : "", esc(m.file)].filter(Boolean).join(" · ");
  row.innerHTML = `<div class="grow"><div>${esc(m.title)}${m.client_only ? '<span class="badge">nur Client</span>' : ""}` +
    `${m.folder ? '<span class="badge">Ordner</span>' : ""}</div><div class="sub">${sub}</div>` +
    (m.reason && !m.enabled ? `<div class="reason">Aus: ${esc(m.reason)}</div>` : "") + `</div>`;
  if (!m.folder) {
    const sw = document.createElement("label");
    sw.className = "switch";
    sw.title = m.enabled ? "Eingeschaltet – klicken zum Ausschalten" : "Ausgeschaltet – klicken zum Einschalten";
    sw.innerHTML = `<input type="checkbox" ${m.enabled ? "checked" : ""}><span></span>`;
    const input = sw.querySelector("input");
    input.onchange = async () => {
      input.disabled = true;
      try {
        const r = await invoke("mod_set_enabled", { rel: m.rel, on: input.checked });
        addEvent(`${m.title} ${input.checked ? "eingeschaltet" : "ausgeschaltet"}`);
        const hint = $("mods-hint");
        hint.textContent = r.restart_needed ? "Der Server läuft – die Änderung wirkt nach dem nächsten Neustart." : "";
        hint.classList.toggle("hidden", !r.restart_needed);
      } catch (e) { error("Umschalten nicht möglich", e); }
      refreshMods();
    };
    row.appendChild(sw);
  }
  return row;
}

async function refreshMods() {
  let l;
  try { l = await invoke("mods_list"); } catch (_) { $("mods-list").innerHTML = ""; $("packs-list").innerHTML = ""; return; }
  const word = l.mod_dir === "plugins" ? "Plugins" : "Mods";
  $("mods-heading").textContent = word;
  modsInfo = l;
  $("btn-add-mods").textContent = (details?.info?.type === "vanilla" ? "Datapacks" : word) + " hinzufügen …";
  $("mods-count").textContent = `${l.mods.filter((m) => m.enabled).length} von ${l.mods.length} ${word} eingeschaltet` + (l.datapacks.length ? ` · ${l.datapacks.length} Datapacks` : "");
  for (const [id, items, empty] of [["mods-list", l.mods, `Keine ${word} im Ordner „${l.mod_dir}“.`], ["packs-list", l.datapacks, "Keine Datapacks in der Welt."]]) {
    const box = $(id);
    box.innerHTML = items.length ? "" : `<div class="empty">${empty}</div>`;
    for (const m of items) box.appendChild(modRow(m));
  }
}

let modsInfo = null;

// ------------------------------------------------------------------ Mods hinzufügen (Modrinth)

const fmtCount = (n) => n >= 1e6 ? `${num(n / 1e6)} Mio.` : n >= 1e3 ? `${Math.round(n / 1e3)} Tsd.` : String(n);

async function openCatalog() {
  const vanilla = details?.info?.type === "vanilla";
  const word = vanilla ? "Datapacks" : modsInfo?.mod_dir === "plugins" ? "Plugins" : "Mods";
  const body = `<div class="catalog-bar">
      <input type="search" id="cat-q" placeholder="Suchen auf Modrinth … (leer = die beliebtesten)" autocomplete="off" spellcheck="false">
      ${vanilla ? "" : `<select id="cat-what"><option value="mods">${word}</option><option value="datapacks">Datapacks (für die Welt)</option></select>`}
    </div>
    <div id="cat-info" class="dim small"></div>
    <p id="catalog-msg" class="hint"></p>
    <div id="cat-list" class="catalog-list"></div>
    <div class="row"><button id="cat-more" class="hidden">Mehr laden</button></div>`;
  let installed = new Set();
  let offset = 0;
  let total = 0;
  let timer = null;
  const datapacks = () => vanilla || ($("cat-what") && $("cat-what").value === "datapacks");
  const msg = (html) => { $("catalog-msg").innerHTML = html; };

  const load = async (reset) => {
    if (reset) { offset = 0; $("cat-list").innerHTML = `<div class="empty">Suche …</div>`; }
    let r;
    try { r = await invoke("catalog_search", { query: $("cat-q").value, offset, datapacks: datapacks() }); }
    catch (e) { $("cat-list").innerHTML = `<div class="empty">Fehler: ${esc(e)}</div>`; return; }
    if (reset) $("cat-list").innerHTML = "";
    total = r.total || 0;
    $("cat-info").textContent = `${total.toLocaleString("de-DE")} passende ${datapacks() ? "Datapacks" : word} für Minecraft ${r.mc}` +
      ($("cat-q").value.trim() ? "" : " – sortiert nach Beliebtheit");
    for (const h of r.hits) $("cat-list").appendChild(hitRow(h));
    offset += r.hits.length;
    if (!r.hits.length && reset) $("cat-list").innerHTML = `<div class="empty">Nichts gefunden.</div>`;
    $("cat-more").classList.toggle("hidden", offset >= total);
  };

  const hitRow = (h) => {
    const row = document.createElement("div");
    row.className = "hit";
    const icon = h.icon ? `<img src="${esc(h.icon)}" alt="" loading="lazy">` : `<div class="noicon"></div>`;
    const side = h.client_required
      ? `<span class="badge both" title="Die Spieler brauchen das auch selbst – z. B. über „Mods für Freunde exportieren (.mrpack)“">Server + Client</span>`
      : `<span class="badge">Nur Server</span>`;
    row.innerHTML = `${icon}<div class="txt"><div class="t">${esc(h.title)} ${side}</div><div class="d">${esc(h.description)}</div>` +
      `<div class="meta">${fmtCount(h.downloads || 0)} Downloads · von ${esc(h.author || "?")}</div></div>`;
    const btn = document.createElement("button");
    const isIn = installed.has(h.id);
    btn.textContent = isIn ? "Installiert" : "Installieren";
    btn.disabled = isIn;
    if (isIn) btn.className = "badge ok";
    else btn.className = "primary";
    btn.onclick = async () => {
      btn.disabled = true;
      btn.textContent = "Lädt …";
      try {
        const r = await task("catalog_install", { project: h.id, title: h.title, datapacks: datapacks() });
        installed = new Set(await invoke("catalog_installed").catch(() => [...installed]));
        btn.textContent = "Installiert";
        btn.className = "badge ok";
        const extra = r.installed.slice(1).map((x) => x.title);
        let text = `<b>${esc(r.title)}</b> ist installiert (in „${esc(r.folder)}“)` +
          (extra.length ? `, dazu als Abhängigkeit: ${extra.map(esc).join(", ")}` : "") + ".";
        if (r.skipped.length) text += ` Schon vorhanden: ${r.skipped.map(esc).join(", ")}.`;
        if (r.missing.length) text += `<br>Achtung: Diese Abhängigkeit gibt es nicht für diese Version: ${r.missing.map(esc).join(", ")} – der Server startet so evtl. nicht.`;
        if (h.client_required) text += `<br>Die Spieler brauchen diese Mod auch – am einfachsten über „Mods für Freunde exportieren (.mrpack)“ im Tab Updates.`;
        if (r.restart_needed) text += `<br>Der Server läuft – das wirkt erst nach einem Neustart. <button id="cat-restart">Jetzt neu starten</button>`;
        msg(text);
        if (r.restart_needed) $("cat-restart").onclick = () => { if (modalResolve) modalResolve(false); askStop(true); };
        addEvent(`Installiert: ${r.title}`);
      } catch (e) {
        btn.disabled = false;
        btn.textContent = "Installieren";
        msg(`Installieren hat nicht geklappt: ${esc(e)}`);
      }
    };
    row.appendChild(btn);
    return row;
  };

  await ask(`${word} hinzufügen`, body, [{ label: "Schließen", value: false, kind: "primary" }], {
    html: true, wide: true, noFocus: true, onOpen: async () => {
      $("cat-q").oninput = () => { clearTimeout(timer); timer = setTimeout(() => load(true), 350); };
      $("cat-q").onkeydown = (e) => { if (e.key === "Enter") { clearTimeout(timer); load(true); } };
      if ($("cat-what")) $("cat-what").onchange = () => load(true);
      $("cat-more").onclick = () => load(false);
      $("cat-q").focus();
      installed = new Set(await invoke("catalog_installed").catch(() => []));
      load(true);
    },
  });
  refreshMods();
}

$("btn-add-mods").onclick = openCatalog;
$("btn-open-mods").onclick = () => invoke("open_folder", { which: "mods" }).catch((e) => error("Ordner", e));

$("btn-cleanup").onclick = async () => {
  let p;
  try { p = await invoke("cleanup_preview"); } catch (e) { return error("Aufräumen", e); }
  if (!p.items.length) return info("Aufräumen", "Alles sauber – es gibt keine Reste alter Versionen.");
  const stopped = ["stopped", "failed", "none"].includes(status.state);
  const list = p.items.map((c) => `<div><span class="sz">${fmtBytes(c.size)}</span><span><code>${esc(c.rel)}</code><br><span class="dim small">${esc(c.what)}</span></span></div>`).join("");
  const body = `<p>Diese Reste alter Versionen würden gelöscht (zusammen <b>${fmtBytes(p.total)}</b>). Welt, Einstellungen und ausgeschaltete Mods bleiben.` +
    ` Was der Server doch noch braucht, lädt er beim nächsten Start selbst nach.</p><div class="cleanup-list">${list}</div>` +
    (stopped ? "" : `<p class="hint">Zum Löschen bitte zuerst den Server stoppen.</p>`);
  const ok = await ask("Aufräumen", body, stopped
    ? [{ label: "Abbrechen", value: false }, { label: `${fmtBytes(p.total)} löschen`, value: true, kind: "primary" }]
    : [{ label: "Schließen", value: false, kind: "primary" }], { html: true });
  if (ok !== true) return;
  try {
    const r = await task("cleanup_run");
    addEvent(`Aufgeräumt: ${fmtBytes(r.freed)} frei`);
    info("Aufgeräumt", `${r.count} Einträge gelöscht, ${fmtBytes(r.freed)} frei.` + (r.failed.length ? `\n\nNicht löschbar:\n${r.failed.join("\n")}` : ""));
  } catch (e) { error("Aufräumen fehlgeschlagen", e); }
  refreshMods();
};

// ------------------------------------------------------------------ Updates

const ST_TEXT = {
  update: "✓ Update", current: "= aktuell", missing: "✗ fehlt", unknown: "? unbekannt",
  new: "+ neu (wird gebraucht)", reenable: "✓ passt wieder – wird eingeschaltet", duplicate: "⇄ doppelt (ist schon als Mod da)",
};
let updInfo = null;
let plan = null;

async function refreshUpdateTab() {
  try { updInfo = await invoke("upd_info"); } catch (e) { $("upd-current").textContent = String(e); return; }
  const i = updInfo.info;
  const loaderText = !i.loader ? "" : i.type === "paper" ? ` · Paper Build ${esc(i.loader)}` : i.type === "neoforge" ? ` · NeoForge ${esc(i.loader)}` : i.type === "fabric" ? ` · Fabric ${esc(i.loader)}` : "";
  $("upd-current").innerHTML = `<b>Jetzt:</b> Minecraft ${esc(i.version || "?")}` + loaderText +
    (updInfo.mods ? ` · ${updInfo.mods} ${updInfo.mod_dir === "plugins" ? "Plugins" : "Mods"}` : "") + (updInfo.datapacks ? ` · ${updInfo.datapacks} Datapacks` : "") + `  <span class="dim">(${esc(i.type)})</span>`;
  const supported = ["fabric", "paper", "neoforge", "vanilla"].includes(i.type);
  for (const id of ["upd-target", "btn-upd-versions", "btn-upd-check"]) $(id).disabled = !supported;
  $("btn-mrpack").disabled = !["fabric", "neoforge"].includes(i.type);
  $("btn-mrpack").title = $("btn-mrpack").disabled ? "Nur für Fabric- und NeoForge-Server (Mods, die auch die Freunde brauchen)" : "";
  if (!supported) $("upd-summary").textContent = "Den Update-Assistenten gibt es für Fabric, Paper, NeoForge und Vanilla.";
  $("btn-upd-rollback").classList.toggle("hidden", !updInfo.rollback);
  const sel = $("upd-target");
  if (!sel.options.length || sel.dataset.server !== serverPath) {
    sel.dataset.server = serverPath;
    sel.innerHTML = i.version ? `<option value="${esc(i.version)}">${esc(i.version)} (nur Mods/Plugins aktualisieren)</option>` : "";
    clearPlan();
  }
}

function clearPlan() {
  plan = null;
  $("upd-table").querySelector("tbody").innerHTML = "";
  $("upd-summary").textContent = "";
  $("btn-upd-run").disabled = true;
  $("btn-upd-trial").disabled = true;
}

$("btn-upd-versions").onclick = async () => {
  let versions;
  try { versions = await invoke("upd_versions"); } catch (e) { return error("Keine Verbindung", e); }
  const cur = updInfo.info.version;
  const sel = $("upd-target");
  sel.innerHTML = versions.map((v) => `<option value="${esc(v)}">${esc(v)}</option>`).join("") +
    (cur ? `<option value="${esc(cur)}">${esc(cur)} (nur Mods/Plugins aktualisieren)</option>` : "");
  if (!versions.length) $("upd-summary").textContent = "Es gibt keine neuere stabile Minecraft-Version mit Fabric.";
};
$("upd-target").onchange = clearPlan;

$("btn-upd-check").onclick = async () => {
  const target = $("upd-target").value;
  if (!target) return;
  clearPlan();
  let r;
  try { r = await task("upd_check", { target }); } catch (e) { return error("Prüfen fehlgeschlagen", e); }
  plan = r.plan;
  showPlan(r.has_work);
};

function showPlan(hasWork) {
  const tb = $("upd-table").querySelector("tbody");
  tb.innerHTML = "";
  const rows = [...plan.mods, ...plan.extra, ...plan.datapacks];
  for (const m of rows) {
    const tr = document.createElement("tr");
    const title = (m.kind === "datapack" ? "Datapack: " : plan.mod_dir === "plugins" ? "Plugin: " : "") + (m.title || m.file || m.project_id || "?");
    const st = m.disabled && m.status === "update" ? "✓ passt wieder – wird eingeschaltet" : ST_TEXT[m.status] || m.status;
    tr.innerHTML = `<td class="st-${m.status}">${st}</td><td>${esc(title)}</td>` +
      `<td>${esc(m.current_version || (m.status === "new" ? "–" : m.file))}</td><td>${esc(m.new_version || "")}</td>`;
    tb.appendChild(tr);
  }
  const count = (st) => [...plan.mods, ...plan.datapacks].filter((m) => m.status === st).length;
  const newDeps = plan.extra.filter((e) => e.status === "new").length;
  const lines = [`${count("update")} Updates, ${count("current")} schon aktuell, ${count("missing")} ohne passende Version, ${count("unknown")} unbekannt` +
    (newDeps ? `, ${newDeps} werden neu gebraucht` : "") + (count("reenable") ? `, ${count("reenable")} passen wieder` : "") + (count("duplicate") ? `, ${count("duplicate")} doppelt (wird beiseitegelegt)` : "")];
  if (plan.server && !plan.change_version) lines.push(`Neuer Server-Stand für Minecraft ${plan.target}: ${plan.server.label}.`);
  if (plan.server && plan.server.beta) lines.push("Hinweis: Dafür gibt es bisher nur eine Test-/Beta-Version.");
  if (plan.change_version) {
    lines.push(`Server: Minecraft ${plan.current} → ${plan.target} (${plan.server ? plan.server.label : "?"}).`);
    lines.push("Achtung: Eine Welt, die einmal mit der neuen Version lief, kann man nicht zurückstufen. Zurück geht nur über das Backup, das automatisch gemacht wird.");
  }
  if (plan.java_required) {
    lines.push(plan.java_ok ? `Java ${plan.java_required} nötig – vorhanden (${plan.java_path}).`
      : `Java ${plan.java_required} nötig, aber nicht installiert. ${javaHint(plan.java_required)}`);
  }
  if (!plan.space_ok) lines.push(`Zu wenig Speicherplatz für das Backup (${fmtBytes(plan.free_space)} frei, Server ${fmtBytes(plan.server_size)}).`);
  if (!hasWork) lines.splice(0, lines.length, "Alles ist auf dem neuesten Stand.");
  $("upd-summary").textContent = lines.join("\n");
  $("upd-unknown-row").classList.toggle("hidden", count("unknown") === 0);
  const ok = plan.java_ok && plan.space_ok && hasWork;
  $("btn-upd-run").disabled = !ok;
  $("btn-upd-trial").disabled = !ok;
}

function javaHint(n) {
  return navigator.userAgent.includes("Windows") ? `Installieren z. B. von adoptium.net (Temurin ${n}).`
    : `Installieren mit: sudo apt install openjdk-${n}-jre-headless`;
}

const updOptions = () => ({ disableMissing: $("upd-disable-missing").checked, disableUnknown: $("upd-disable-unknown").checked });

$("btn-upd-run").onclick = async () => {
  if (!plan) return;
  const what = plan.change_version ? `Minecraft ${plan.current} → ${plan.target}` : "Mods aktualisieren";
  const n = plan.mods.filter((m) => m.status === "update").length + plan.extra.filter((e) => e.status === "new").length +
    plan.datapacks.filter((d) => d.status === "update").length;
  let msg = `Ablauf: Server stoppen (mit Warnung, falls jemand online ist) → komplettes Backup → ${n} Dateien laden und prüfen → tauschen → Server starten und prüfen.`;
  const missing = [...plan.mods, ...plan.datapacks].filter((m) => m.status === "missing").length;
  if (missing && $("upd-disable-missing").checked) msg += `\n\n${missing} Mod(s)/Datapack(s) ohne passende Version werden deaktiviert.`;
  msg += "\n\nTipp: Mit „Probelauf“ kannst du das vorher gefahrlos an einer Kopie testen.";
  if (!(await confirmAsk(`Update starten? (${what})`, msg, "Update starten"))) return;
  addEvent(`Update gestartet (${what})`);
  let r;
  try { r = await task("upd_run", updOptions()); } catch (e) { error("Update fehlgeschlagen", e); refreshUpdateTab(); return; }
  clearPlan();
  await refreshUpdateTab();
  invoke("log_reset");
  if (r.ok) {
    addEvent(`Update fertig (${r.what})`);
    const c = r.cleaned;
    info("Update fertig", `Der Server läuft wieder (${r.what}).\n\nBackup vom alten Stand:\n${r.backup}` +
      (c ? `\n\nAufgeräumt: ${c.count} alte Dateien/Ordner gelöscht (${fmtBytes(c.freed)}).` : "") +
      "\nMods ohne passende Version sind ausgeschaltet und im Tab „Mods“ zu sehen.");
  } else if (await confirmAsk("Update hat nicht geklappt",
    `${r.error}\n\nSchau in der Konsole nach der Fehlermeldung. Soll ich den Stand vor dem Update aus dem Backup wiederherstellen?`, "Wiederherstellen")) {
    rollback(false);
  }
};

async function rollback(ask = true) {
  if (ask && !(await confirmAsk("Zurück zum Stand vor dem letzten Update?",
    "Der Server wird gestoppt und aus dem Backup wiederhergestellt, das beim Update gemacht wurde. Der jetzige Stand bleibt als Ordner „…_vor_Wiederherstellung_<Datum>“ daneben liegen.", "Zurücksetzen"))) return;
  try {
    const r = await task("upd_rollback");
    addEvent("Stand vor dem Update wiederhergestellt");
    info("Zurückgesetzt", `Der Server ist wieder auf dem Stand von ${r.label}. Du kannst ihn jetzt starten.` +
      (r.aside ? `\n\nDer Stand nach dem Update liegt in:\n${r.aside}` : ""));
  } catch (e) { error("Zurücksetzen fehlgeschlagen", e); }
  invoke("log_reset");
  refreshUpdateTab();
}
$("btn-upd-rollback").onclick = () => rollback(true);

$("btn-upd-trial").onclick = async () => {
  if (!plan) return;
  if (!(await confirmAsk("Probelauf starten?",
    "Ich kopiere den Server in den Temp-Ordner, aktualisiere die Kopie, starte sie auf einem anderen Port und schaue, ob sie fertig hochfährt. Danach wird die Kopie gestoppt und gelöscht.\n\nDein echter Server bleibt dabei unverändert und kann weiterlaufen. Bei großen Welten dauert das Kopieren ein paar Minuten.", "Probelauf starten"))) return;
  let r;
  try { r = await task("upd_trial", updOptions()); } catch (e) { return error("Probelauf fehlgeschlagen", e); }
  const parts = [];
  if (r.ok) parts.push(`✓ Die aktualisierte Kopie ist in ${r.seconds} Sekunden fertig gestartet (Port ${r.port}).`);
  else if (r.timeout) parts.push("✗ Die Kopie war nach 15 Minuten noch nicht fertig gestartet.");
  else parts.push(`✗ Die Kopie hat sich beim Start beendet (Exit-Code ${r.exit_code ?? "?"}).`);
  const lines = [r.version_line, r.mods_line].filter(Boolean).map((l) => l.replace(/^.*\]: /, ""));
  if (lines.length) parts.push(lines.join("\n"));
  if (r.problems.length) parts.push("Auffällige Meldungen:\n" + r.problems.map((l) => "• " + l.replace(/^\[[^\]]*\] /, "")).join("\n"));
  if (r.tail.length) parts.push("Letzte Zeilen:\n" + r.tail.join("\n"));
  parts.push(r.copy_deleted ? "Die Kopie wurde wieder gelöscht." : `Die Kopie konnte nicht ganz gelöscht werden: ${r.copy}`);
  addEvent(`Probelauf ${r.ok ? "erfolgreich" : "fehlgeschlagen"}`);
  ask(r.ok ? "Probelauf erfolgreich" : "Probelauf fehlgeschlagen", parts.join("\n\n"), [{ label: "OK", value: true, kind: "primary" }]);
};

$("btn-mrpack").onclick = async () => {
  let r;
  try { r = await invoke("export_mrpack"); } catch (e) { return error("Export fehlgeschlagen", e); }
  if (!r) return;
  info("Modpack gespeichert", `${r.path}\n\n${r.files} Mods von Modrinth` + (r.overrides ? `, ${r.overrides} direkt eingepackt` : "") +
    ".\nDeine Freunde importieren die Datei in der Modrinth-App („Datei hinzufügen“). Reine Server-Mods werden dort nicht installiert.");
};

// ------------------------------------------------------------------ Start

async function init() {
  const cfg = await invoke("get_config");
  $("about").textContent = `Redcraftserver ${cfg.version}` + (cfg.test_mode ? " · TESTMODUS" : "");
  await refreshStatus();
  refreshDetails();
  setInterval(refreshStatus, 2000);
  setInterval(refreshLive, 5000);
  setInterval(refreshDetails, 60000);
  refreshLive();
}

init();
