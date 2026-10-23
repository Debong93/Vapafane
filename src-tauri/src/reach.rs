//! Erreichbarkeit des Servers von außen – die Ampel im Reiter „Erreichbarkeit“.
//!
//! Geprüft wird getrennt über IPv4 und IPv6: Ein Prüfdienst im Internet (ifconfig.co)
//! versucht, den Server-Port unter der Adresse zu erreichen, von der die Anfrage kommt.
//! Für IPv6 geht die Anfrage bewusst von der festen Adresse des PCs aus (nicht von der
//! täglich wechselnden „temporären“), denn die steht auch im Einladungs-Code.
//!
//! DS-Lite (geteilte IPv4-Adresse) erkennt Vapafane, indem es den Router per UPnP nach
//! seiner Internet-Adresse fragt – nur lesend, es wird nichts freigegeben oder verändert.
//!
//! Läuft der Minecraft-Server nicht, antwortet Vapafane für den Test kurz selbst auf dem
//! Port (nicht unter Windows: dort würde die Firewall dafür extra nachfragen).

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::config;
use crate::server;
use crate::util::Result;

const CHECK_HOST: &str = "ifconfig.co";
/// Englische Anleitung zur Portfreigabe für alle Router-Marken.
pub const GUIDE_URL: &str = "https://portforward.com/minecraft-java-edition-server/";

/// Ergebnis für die Anzeige (alle Texte kommen fertig aus Rust).
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Report {
    /// "green", "yellow", "red" oder "gray"
    pub light: String,
    pub title: String,
    pub text: String,
    /// Zeilen „IPv4“ / „IPv6“: Text und Farbe ("ok", "warn", "bad", "none")
    pub v4: String,
    pub v4_state: String,
    pub v6: String,
    pub v6_state: String,
    /// Server nur über IPv6 erreichbar (kommt in den Einladungs-Code)
    pub v6_only: bool,
    pub ds_lite: bool,
    /// Link zur Anleitung zeigen
    pub guide: bool,
    pub port: u16,
    /// Feste IPv6-Adresse dieses PCs
    #[serde(default)]
    pub ipv6: Option<String>,
    /// Server lief nicht und konnte nicht geprüft werden (nur Windows)
    #[serde(default)]
    pub server_off: bool,
    /// Zeitpunkt der Prüfung (Unix-Sekunden)
    pub checked: i64,
}

/// Was die Prüfung herausgefunden hat (ohne Texte – die macht [`classify`]).
#[derive(Clone, Debug, Default)]
pub struct Findings {
    pub port: u16,
    /// Port über IPv4 von außen offen? `None` = Prüfung nicht möglich
    pub v4: Option<bool>,
    /// Öffentliche IPv4, wie der Prüfdienst sie sieht
    pub v4_public: Option<Ipv4Addr>,
    /// Internet-Adresse laut Router (UPnP), falls er sie verrät
    pub router_v4: Option<Ipv4Addr>,
    /// Feste globale IPv6-Adresse dieses PCs
    pub ipv6: Option<Ipv6Addr>,
    /// Port über IPv6 von außen offen? `None` = keine IPv6 oder Prüfung nicht möglich
    pub v6: Option<bool>,
}

// ---------------------------------------------------------------- Auswerten

/// Adresse, die nicht direkt im Internet steht (Router hat dann keine eigene IPv4).
fn shared_v4(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    ip.is_private()
        || ip.is_unspecified()
        || ip.is_loopback()
        || (o[0] == 100 && (o[1] & 0xc0) == 64) // 100.64.0.0/10 (Carrier-NAT)
        || (o[0] == 192 && o[1] == 0 && o[2] == 0) // 192.0.0.0/24 (DS-Lite)
}

fn ds_lite(f: &Findings) -> bool {
    match f.router_v4 {
        Some(r) if shared_v4(r) => true,
        Some(r) => f.v4_public.is_some_and(|p| p != r),
        None => false,
    }
}

/// Ampel und Texte aus den Befunden.
pub fn classify(f: &Findings) -> Report {
    let ds = ds_lite(f);
    let port = f.port;
    let mut r = Report {
        port,
        ds_lite: ds,
        ipv6: f.ipv6.map(|a| a.to_string()),
        checked: chrono::Local::now().timestamp(),
        ..Default::default()
    };
    (r.v4, r.v4_state) = match f.v4 {
        Some(true) => ("erreichbar".into(), "ok".into()),
        Some(false) if ds => ("nicht erreichbar (DS-Lite)".into(), "warn".into()),
        Some(false) => ("nicht erreichbar".into(), "bad".into()),
        None => ("nicht geprüft".into(), "none".into()),
    };
    (r.v6, r.v6_state) = match (f.ipv6, f.v6) {
        (None, _) => ("nicht vorhanden".into(), "none".into()),
        (Some(_), Some(true)) => ("erreichbar".into(), "ok".into()),
        (Some(_), Some(false)) => ("nicht erreichbar".into(), "bad".into()),
        (Some(_), None) => ("nicht geprüft".into(), "none".into()),
    };
    let v6_ok = f.v6 == Some(true);
    if f.v4.is_none() && f.v6.is_none() {
        r.light = "gray".into();
        r.title = "Prüfung gerade nicht möglich".into();
        r.text = "Der Prüfdienst im Internet antwortet nicht. Später erneut prüfen.".into();
    } else if f.v4 == Some(true) {
        r.light = "green".into();
        r.title = if v6_ok {
            "Über IPv4 und IPv6 erreichbar"
        } else {
            "Über IPv4 erreichbar"
        }
        .into();
        r.text = "Alle Freunde kommen rein.".into();
    } else if v6_ok {
        r.light = "yellow".into();
        r.v6_only = true;
        r.guide = !ds;
        if ds {
            r.title = "DS-Lite: nur über IPv6 erreichbar".into();
            r.text =
                "Dein Anbieter teilt die IPv4-Adresse. Freunde mit IPv6 kommen trotzdem rein, das sind die meisten."
                    .into();
        } else {
            r.title = "Nur über IPv6 erreichbar".into();
            r.text = format!(
                "Über IPv4 kommt niemand rein: Entweder fehlt die Portfreigabe für IPv4 (Port {port}), oder dein \
                 Anbieter teilt die IPv4-Adresse (DS-Lite). Freunde mit IPv6 kommen trotzdem rein."
            );
        }
    } else {
        r.light = "red".into();
        r.guide = true;
        r.title = "Von außen nicht erreichbar".into();
        r.text = if ds && f.ipv6.is_some() {
            format!(
                "Dein Anbieter teilt die IPv4-Adresse (DS-Lite), es geht also nur über IPv6. Gib im Router die \
                 IPv6-Freigabe für Port {port} frei, und prüf die Firewall am PC."
            )
        } else if ds {
            "Dein Anbieter teilt die IPv4-Adresse (DS-Lite), und dein PC hat keine IPv6-Adresse. So kommt von \
             außen niemand an den Server."
                .into()
        } else {
            format!("Meist fehlt noch die Portfreigabe im Router, oder die Firewall am PC blockt Port {port}.")
        };
    }
    r
}

/// Anzeige, wenn der Server aus ist und nicht geprüft werden kann (Windows).
fn server_off_report(port: u16) -> Report {
    Report {
        light: "gray".into(),
        title: "Server läuft nicht".into(),
        text: "Geprüft wird, sobald der Server läuft.".into(),
        v4: "nicht geprüft".into(),
        v4_state: "none".into(),
        v6: "nicht geprüft".into(),
        v6_state: "none".into(),
        port,
        server_off: true,
        checked: chrono::Local::now().timestamp(),
        ..Default::default()
    }
}

// ---------------------------------------------------------------- Prüfen

pub fn server_port(dir: &Path) -> u16 {
    server::read_properties(dir)
        .get("server-port")
        .and_then(|p| p.trim().parse().ok())
        .unwrap_or(25565)
}

fn state_file() -> PathBuf {
    config::state_dir().join("reach.json")
}

/// Letztes Ergebnis (für die Anzeige beim Öffnen und den Einladungs-Code).
pub fn last() -> Option<Report> {
    serde_json::from_str(&fs::read_to_string(state_file()).ok()?).ok()
}

/// Erreichbarkeit des Servers in `dir` prüfen und das Ergebnis merken.
pub fn check(dir: &Path) -> Result<Report> {
    let port = server_port(dir);
    let report = match test_override(port) {
        Some(r) => r,
        None => check_now(port),
    };
    let _ = fs::create_dir_all(config::state_dir());
    crate::util::write_atomic(&state_file(), serde_json::to_string_pretty(&report)?.as_bytes())?;
    Ok(report)
}

/// Im Testmodus lässt sich die Ampel mit `VAPAFANE_TEST_REACH=green|yellow|red|gray` vorgeben.
fn test_override(port: u16) -> Option<Report> {
    config::test_home()?;
    let want = std::env::var("VAPAFANE_TEST_REACH").ok()?;
    let v6: Ipv6Addr = "2001:db8::1".parse().ok()?;
    let mut f = Findings {
        port,
        v4_public: Some(Ipv4Addr::new(203, 0, 113, 7)),
        router_v4: Some(Ipv4Addr::new(203, 0, 113, 7)),
        ipv6: Some(v6),
        ..Default::default()
    };
    match want.as_str() {
        "green" => (f.v4, f.v6) = (Some(true), Some(true)),
        "yellow" => (f.v4, f.v6, f.router_v4) = (Some(false), Some(true), Some(Ipv4Addr::new(192, 0, 0, 2))),
        "red" => (f.v4, f.v6) = (Some(false), Some(false)),
        "off" => return Some(server_off_report(port)),
        _ => {}
    }
    Some(classify(&f))
}

fn check_now(port: u16) -> Report {
    // Antwortet schon jemand auf dem Port (der Minecraft-Server)? Sonst kurz selbst antworten.
    let listening = TcpStream::connect_timeout(&SocketAddr::from((Ipv4Addr::LOCALHOST, port)), Duration::from_secs(1))
        .is_ok()
        || TcpStream::connect_timeout(&SocketAddr::from((Ipv6Addr::LOCALHOST, port)), Duration::from_secs(1)).is_ok();
    let _responder = if listening {
        None
    } else if cfg!(windows) {
        return server_off_report(port);
    } else {
        Responder::start(port)
    };

    let ipv6 = stable_ipv6();
    let t4 = std::thread::spawn(move || port_check(None, port));
    let t6 = std::thread::spawn(move || ipv6.and_then(|a| port_check(Some(IpAddr::V6(a)), port)));
    let router_v4 = router_external_ipv4();
    let v4 = t4.join().ok().flatten();
    let v6 = t6.join().ok().flatten();
    classify(&Findings {
        port,
        v4: v4.map(|(open, _)| open),
        v4_public: v4.and_then(|(_, ip)| match ip {
            Some(IpAddr::V4(a)) => Some(a),
            _ => None,
        }),
        router_v4,
        ipv6,
        v6: v6.map(|(open, _)| open),
    })
}

/// Nimmt für die Dauer der Prüfung Verbindungen auf dem Port an (und schließt sie gleich wieder).
struct Responder {
    stop: Arc<AtomicBool>,
}

impl Responder {
    fn start(port: u16) -> Option<Responder> {
        // [::] nimmt unter Linux auch IPv4 an; falls nicht, zusätzlich 0.0.0.0
        let listeners: Vec<TcpListener> = [
            SocketAddr::from((Ipv6Addr::UNSPECIFIED, port)),
            SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)),
        ]
        .into_iter()
        .filter_map(|a| TcpListener::bind(a).ok())
        .collect();
        if listeners.is_empty() {
            return None;
        }
        let stop = Arc::new(AtomicBool::new(false));
        for l in listeners {
            let stop = stop.clone();
            let _ = l.set_nonblocking(true);
            std::thread::spawn(move || {
                // Spätestens nach einer Minute aufhören, falls die Prüfung hängt
                for _ in 0..600 {
                    if stop.load(Ordering::Relaxed) {
                        break;
                    }
                    while let Ok((s, _)) = l.accept() {
                        drop(s);
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            });
        }
        Some(Responder { stop })
    }
}

impl Drop for Responder {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// Fragt den Prüfdienst, ob `port` unter der eigenen Adresse von außen erreichbar ist.
/// `src` = Absender-Adresse (IPv6: die feste Adresse des PCs); `None` = über IPv4.
/// Ergebnis: (offen?, eigene Adresse laut Prüfdienst) – `None`, wenn es nicht ging.
fn port_check(src: Option<IpAddr>, port: u16) -> Option<(bool, Option<IpAddr>)> {
    use socket2::{Domain, Protocol, Socket, Type};
    let v6 = src.is_some_and(|s| s.is_ipv6());
    let target = (CHECK_HOST, 80).to_socket_addrs().ok()?.find(|a| a.is_ipv6() == v6)?;
    let sock = Socket::new(Domain::for_address(target), Type::STREAM, Some(Protocol::TCP)).ok()?;
    if let Some(s) = src {
        sock.bind(&SocketAddr::new(s, 0).into()).ok()?;
    }
    sock.connect_timeout(&target.into(), Duration::from_secs(8)).ok()?;
    let mut s: TcpStream = sock.into();
    s.set_read_timeout(Some(Duration::from_secs(25))).ok()?;
    s.set_write_timeout(Some(Duration::from_secs(8))).ok()?;
    let req = format!(
        "GET /port/{port} HTTP/1.1\r\nHost: {CHECK_HOST}\r\nUser-Agent: {}\r\nAccept: application/json\r\n\
         Connection: close\r\n\r\n",
        crate::net::USER_AGENT
    );
    s.write_all(req.as_bytes()).ok()?;
    let mut buf = Vec::new();
    let _ = s.take(64 * 1024).read_to_end(&mut buf);
    parse_port_answer(&String::from_utf8_lossy(&buf))
}

fn parse_port_answer(resp: &str) -> Option<(bool, Option<IpAddr>)> {
    if !resp.starts_with("HTTP/1.1 200") && !resp.starts_with("HTTP/1.0 200") {
        return None;
    }
    let body = resp.split_once("\r\n\r\n")?.1;
    let json = &body[body.find('{')?..=body.rfind('}')?];
    let v: serde_json::Value = serde_json::from_str(json).ok()?;
    let open = v.get("reachable")?.as_bool()?;
    let ip = v.get("ip").and_then(|i| i.as_str()).and_then(|i| i.parse().ok());
    Some((open, ip))
}

// ---------------------------------------------------------------- Eigene IPv6-Adresse

/// Öffentliche IPv6-Adresse (2000::/3), keine Dokumentations-Adresse.
fn global_v6(a: &Ipv6Addr) -> bool {
    let s = a.segments();
    (s[0] & 0xe000) == 0x2000 && !(s[0] == 0x2001 && s[1] == 0x0db8)
}

/// Feste (nicht temporäre) öffentliche IPv6-Adresse dieses PCs.
pub fn stable_ipv6() -> Option<Ipv6Addr> {
    #[cfg(target_os = "linux")]
    {
        let text = fs::read_to_string("/proc/net/if_inet6").ok()?;
        parse_if_inet6(&text)
    }
    #[cfg(windows)]
    {
        stable_ipv6_windows()
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        outgoing_ipv6()
    }
}

/// Aus /proc/net/if_inet6: globale Adressen ohne die Merkmale temporär (0x01),
/// DAD fehlgeschlagen (0x08), veraltet (0x20) und vorläufig (0x40).
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn parse_if_inet6(text: &str) -> Option<Ipv6Addr> {
    text.lines().find_map(|line| {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 6 || f[5] == "lo" {
            return None;
        }
        let flags = u32::from_str_radix(f[4], 16).ok()?;
        if flags & (0x01 | 0x08 | 0x20 | 0x40) != 0 {
            return None;
        }
        let bits = u128::from_str_radix(f[0], 16).ok()?;
        let a = Ipv6Addr::from(bits);
        global_v6(&a).then_some(a)
    })
}

/// Adresse, mit der der PC ins IPv6-Internet geht (`None` = kein IPv6).
/// Schickt nichts – fragt nur das System nach dem Weg.
pub fn outgoing_ipv6() -> Option<Ipv6Addr> {
    let s = UdpSocket::bind((Ipv6Addr::UNSPECIFIED, 0)).ok()?;
    s.connect((Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888), 53))
        .ok()?;
    match s.local_addr().ok()?.ip() {
        IpAddr::V6(a) if global_v6(&a) => Some(a),
        _ => None,
    }
}

/// Windows: feste IPv6-Adresse über PowerShell abfragen (ohne unsicheren Systemaufruf).
/// „Random“ sind die wechselnden temporären Adressen – die werden übersprungen.
#[cfg(windows)]
fn stable_ipv6_windows() -> Option<Ipv6Addr> {
    let out = crate::util::command("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Get-NetIPAddress -AddressFamily IPv6 -AddressState Preferred | Where-Object { $_.SuffixOrigin -ne 'Random' } | ForEach-Object { $_.IPAddress }",
        ])
        .output()
        .ok()?;
    parse_ip_lines(&String::from_utf8_lossy(&out.stdout))
}

/// Erste öffentliche IPv6 aus einer Liste (eine Adresse pro Zeile, evtl. mit %Zone).
#[cfg_attr(not(windows), allow(dead_code))]
fn parse_ip_lines(text: &str) -> Option<Ipv6Addr> {
    text.lines()
        .filter_map(|l| l.trim().split('%').next()?.parse::<Ipv6Addr>().ok())
        .find(global_v6)
}

// ---------------------------------------------------------------- Router fragen (UPnP, nur lesend)

/// Internet-Adresse des Routers per UPnP (GetExternalIPAddress). Ändert nichts am Router.
pub fn router_external_ipv4() -> Option<Ipv4Addr> {
    let location = ssdp_location()?;
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(4)))
        .http_status_as_error(false)
        .build()
        .into();
    let desc = agent.get(&location).call().ok()?.body_mut().read_to_string().ok()?;
    let (service, control) = wan_service(&desc)?;
    let base = desc
        .split("<URLBase>")
        .nth(1)
        .and_then(|s| s.split('<').next())
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| origin(&location));
    let url = if control.starts_with("http") {
        control
    } else {
        format!("{base}/{}", control.trim_start_matches('/'))
    };
    let body = format!(
        "<?xml version=\"1.0\"?><s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" \
         s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\"><s:Body>\
         <u:GetExternalIPAddress xmlns:u=\"{service}\"></u:GetExternalIPAddress></s:Body></s:Envelope>"
    );
    let answer = agent
        .post(&url)
        .header("Content-Type", "text/xml; charset=\"utf-8\"")
        .header("SOAPAction", format!("\"{service}#GetExternalIPAddress\""))
        .send(body)
        .ok()?
        .body_mut()
        .read_to_string()
        .ok()?;
    answer
        .split("<NewExternalIPAddress>")
        .nth(1)?
        .split('<')
        .next()?
        .trim()
        .parse()
        .ok()
}

/// Router im Heimnetz suchen (SSDP) und die Adresse seiner Beschreibung liefern.
fn ssdp_location() -> Option<String> {
    let s = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok()?;
    s.set_read_timeout(Some(Duration::from_millis(2500))).ok()?;
    let msg = "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 2\r\n\
               ST: urn:schemas-upnp-org:device:InternetGatewayDevice:1\r\n\r\n";
    s.send_to(msg.as_bytes(), (Ipv4Addr::new(239, 255, 255, 250), 1900))
        .ok()?;
    let mut buf = [0u8; 2048];
    let (n, from) = s.recv_from(&mut buf).ok()?;
    let text = String::from_utf8_lossy(&buf[..n]);
    let loc = text
        .lines()
        .find_map(|l| {
            l.split_once(':')
                .filter(|(k, _)| k.trim().eq_ignore_ascii_case("location"))
        })?
        .1
        .trim()
        .to_string();
    // Nur dem Gerät glauben, das geantwortet hat (kein Umleiten woandershin)
    let host = loc.strip_prefix("http://")?.split(['/', ':']).next()?;
    (host == from.ip().to_string()).then_some(loc)
}

fn origin(url: &str) -> String {
    let rest = url.split_once("://").map(|(_, r)| r).unwrap_or(url);
    let host = rest.split('/').next().unwrap_or(rest);
    format!("http://{host}")
}

/// (serviceType, controlURL) des Internet-Anschlusses aus der Router-Beschreibung.
fn wan_service(desc: &str) -> Option<(String, String)> {
    desc.split("<service>").skip(1).find_map(|block| {
        let ty = block.split("<serviceType>").nth(1)?.split('<').next()?.trim();
        if !ty.contains(":WANIPConnection:") && !ty.contains(":WANPPPConnection:") {
            return None;
        }
        let ctl = block.split("<controlURL>").nth(1)?.split('<').next()?.trim();
        Some((ty.to_string(), ctl.to_string()))
    })
}

// ---------------------------------------------------------------- Beim Freund

/// Hinweis für den Freund, wenn er den Server nicht erreicht.
pub fn client_hint(only_v6: bool, has_ipv6: bool) -> Option<String> {
    (only_v6 && !has_ipv6).then(|| {
        "Der Server ist nur über IPv6 erreichbar, dein Internet hat aber kein IPv6. Tipp: Verbinde deinen PC über \
         den Hotspot deines Handys – Mobilfunk hat meistens IPv6."
            .to_string()
    })
}

/// Hat dieser PC IPv6-Internet? (Testmodus: `VAPAFANE_TEST_NO_IPV6` spielt „nein“.)
pub fn has_ipv6() -> bool {
    if config::test_home().is_some() && std::env::var_os("VAPAFANE_TEST_NO_IPV6").is_some() {
        return false;
    }
    outgoing_ipv6().is_some()
}

/// IPv6-Adresse mit Port für den Einladungs-Code.
pub fn invite_v6(dir: &Path) -> Option<String> {
    if config::test_home().is_some() && std::env::var("VAPAFANE_TEST_REACH").is_ok() {
        return Some(format!("[2001:db8::1]:{}", server_port(dir)));
    }
    stable_ipv6().map(|a| format!("[{a}]:{}", server_port(dir)))
}

/// Port beim Start prüfen? Nur im Server-Bereich, wenn ein Server angelegt ist.
pub fn should_check_on_start(cfg: &config::Config) -> bool {
    cfg.reach_check && cfg.active_path().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(v4: Option<bool>, v6: Option<bool>, router: Option<[u8; 4]>) -> Findings {
        Findings {
            port: 25565,
            v4,
            v4_public: Some(Ipv4Addr::new(95, 1, 2, 3)),
            router_v4: router.map(Ipv4Addr::from),
            ipv6: Some("2a02:8071::1".parse().unwrap()),
            v6,
        }
    }

    #[test]
    fn ampel() {
        let g = classify(&f(Some(true), Some(true), Some([95, 1, 2, 3])));
        assert_eq!(
            (g.light.as_str(), g.title.as_str()),
            ("green", "Über IPv4 und IPv6 erreichbar")
        );
        assert!(!g.guide && !g.v6_only);

        let y = classify(&f(Some(false), Some(true), Some([192, 0, 0, 2])));
        assert_eq!(y.light, "yellow");
        assert!(y.ds_lite && y.v6_only && !y.guide);
        assert_eq!(y.v4, "nicht erreichbar (DS-Lite)");
        assert_eq!(y.title, "DS-Lite: nur über IPv6 erreichbar");

        // IPv4 zu, aber kein DS-Lite erkennbar: Portfreigabe für IPv4 fehlt vielleicht
        let y2 = classify(&f(Some(false), Some(true), None));
        assert_eq!(y2.light, "yellow");
        assert!(!y2.ds_lite && y2.guide);

        // Carrier-NAT: Router-Adresse ist eine andere als die öffentliche
        assert!(classify(&f(Some(false), Some(false), Some([80, 1, 2, 3]))).ds_lite);
        assert!(classify(&f(Some(false), Some(false), Some([100, 70, 0, 1]))).ds_lite);

        let r = classify(&f(Some(false), Some(false), Some([95, 1, 2, 3])));
        assert_eq!(r.light, "red");
        assert!(r.guide && !r.ds_lite);

        let gray = classify(&f(None, None, None));
        assert_eq!(gray.light, "gray");

        let mut no6 = f(Some(true), None, None);
        no6.ipv6 = None;
        let n = classify(&no6);
        assert_eq!((n.light.as_str(), n.v6.as_str()), ("green", "nicht vorhanden"));
        assert_eq!(n.title, "Über IPv4 erreichbar");
    }

    #[test]
    fn feste_ipv6_aus_proc() {
        let text = "fd03b8a504b900003a0525fffe34c61d 02 40 00 00   enp2s0\n\
                    00000000000000000000000000000001 01 80 10 80       lo\n\
                    fe800000000000003a0525fffe34c61d 02 40 20 80   enp2s0\n\
                    2a0280716c74b0201fa9313414b7c37a 02 40 00 01   enp2s0\n\
                    2a0280716c74b0203a0525fffe34c61d 02 40 00 00   enp2s0\n";
        assert_eq!(
            parse_if_inet6(text),
            Some("2a02:8071:6c74:b020:3a05:25ff:fe34:c61d".parse().unwrap())
        );
        // nur temporäre Adresse → keine feste
        assert_eq!(
            parse_if_inet6("2a0280716c74b0201fa9313414b7c37a 02 40 00 01 enp2s0"),
            None
        );
    }

    #[test]
    fn ipv6_aus_powershell() {
        let text = "fe80::1%12\r\nfd03::5\r\n2a02:8071:6c74:b020:3a05:25ff:fe34:c61d\r\n";
        assert_eq!(
            parse_ip_lines(text),
            Some("2a02:8071:6c74:b020:3a05:25ff:fe34:c61d".parse().unwrap())
        );
        assert_eq!(parse_ip_lines("fe80::1\n"), None);
    }

    #[test]
    fn antwort_des_pruefdienstes() {
        let ok = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\n  \"ip\": \"95.223.160.235\",\n  \
                  \"port\": 25565,\n  \"reachable\": true\n}";
        assert_eq!(
            parse_port_answer(ok),
            Some((true, Some("95.223.160.235".parse().unwrap())))
        );
        // „chunked“: Längenangaben um das JSON herum stören nicht
        let chunked = "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5c\r\n{\"ip\":\"::1\",\"port\":1,\
                       \"reachable\":false}\r\n0\r\n\r\n";
        assert_eq!(parse_port_answer(chunked), Some((false, Some("::1".parse().unwrap()))));
        assert_eq!(parse_port_answer("HTTP/1.1 429 Too Many Requests\r\n\r\n{}"), None);
    }

    #[test]
    fn router_beschreibung() {
        let desc = "<root><URLBase>http://192.168.0.1:49000</URLBase><service>\
            <serviceType>urn:schemas-upnp-org:service:WANCommonInterfaceConfig:1</serviceType>\
            <controlURL>/igdupnp/control/WANCommonIFC1</controlURL></service><service>\
            <serviceType>urn:schemas-upnp-org:service:WANIPConnection:1</serviceType>\
            <controlURL>/igdupnp/control/WANIPConn1</controlURL></service></root>";
        assert_eq!(
            wan_service(desc),
            Some((
                "urn:schemas-upnp-org:service:WANIPConnection:1".into(),
                "/igdupnp/control/WANIPConn1".into()
            ))
        );
        assert_eq!(
            origin("http://192.168.0.1:49000/igddesc.xml"),
            "http://192.168.0.1:49000"
        );
    }

    #[test]
    fn hinweis_beim_freund() {
        assert!(client_hint(true, false).unwrap().contains("Hotspot"));
        assert!(client_hint(true, true).is_none());
        assert!(client_hint(false, false).is_none());
    }
}

#[cfg(test)]
mod live {
    /// Nur von Hand: `cargo test live_einzeln -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn live_einzeln() {
        let v6 = super::stable_ipv6();
        println!("feste IPv6: {v6:?}, ausgehend: {:?}", super::outgoing_ipv6());
        println!("Router: {:?}", super::router_external_ipv4());
        let r = super::Responder::start(25598);
        println!("Responder: {}", r.is_some());
        println!("v4: {:?}", super::port_check(None, 25598));
        println!(
            "v6: {:?}",
            v6.and_then(|a| super::port_check(Some(std::net::IpAddr::V6(a)), 25598))
        );
        println!(
            "lokal: {:?}",
            std::net::TcpStream::connect(("127.0.0.1", 25598)).is_ok()
        );
    }
}
