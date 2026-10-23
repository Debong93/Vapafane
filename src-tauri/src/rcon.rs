//! Kleiner RCON-Client. Port und Passwort kommen aus server.properties.

use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::server;
use crate::util::{Error, Result, err};

const TYPE_COMMAND: i32 = 2;
const TYPE_LOGIN: i32 = 3;

pub struct Rcon {
    dir: PathBuf,
    timeout: Duration,
    sock: Option<TcpStream>,
    next_id: i32,
}

impl Rcon {
    pub fn new(dir: &Path) -> Rcon {
        Rcon {
            dir: dir.to_path_buf(),
            timeout: Duration::from_secs(5),
            sock: None,
            next_id: 1,
        }
    }

    pub fn with_timeout(mut self, t: Duration) -> Rcon {
        self.timeout = t;
        self
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn credentials(&self) -> Result<(u16, String)> {
        let props = server::read_properties(&self.dir);
        if props.get("enable-rcon").map(String::as_str) != Some("true") {
            return err("RCON ist in server.properties ausgeschaltet");
        }
        let port = props.get("rcon.port").and_then(|p| p.parse().ok()).unwrap_or(25575);
        Ok((port, props.get("rcon.password").cloned().unwrap_or_default()))
    }

    fn send(&mut self, kind: i32, body: &str) -> Result<i32> {
        let id = self.next_id;
        self.next_id = if self.next_id >= i32::MAX - 1 {
            1
        } else {
            self.next_id + 1
        };
        let mut payload = Vec::with_capacity(body.len() + 10);
        payload.extend_from_slice(&id.to_le_bytes());
        payload.extend_from_slice(&kind.to_le_bytes());
        payload.extend_from_slice(body.as_bytes());
        payload.extend_from_slice(&[0, 0]);
        let mut packet = (payload.len() as i32).to_le_bytes().to_vec();
        packet.extend_from_slice(&payload);
        let sock = self.sock.as_mut().ok_or_else(|| Error("nicht verbunden".into()))?;
        sock.write_all(&packet)?;
        Ok(id)
    }

    fn recv(&mut self) -> Result<(i32, String)> {
        let sock = self.sock.as_mut().ok_or_else(|| Error("nicht verbunden".into()))?;
        let mut len = [0u8; 4];
        sock.read_exact(&mut len)?;
        let len = i32::from_le_bytes(len);
        if !(10..=1_100_000).contains(&len) {
            return err("Ungültige RCON-Antwort");
        }
        let mut data = vec![0u8; len as usize];
        sock.read_exact(&mut data)?;
        let id = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let body = String::from_utf8_lossy(&data[8..data.len() - 2]).into_owned();
        Ok((id, body))
    }

    pub fn connect(&mut self) -> Result<()> {
        self.close();
        let (port, password) = self.credentials()?;
        let addr = ("127.0.0.1", port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| Error("Adresse ungültig".into()))?;
        let sock = TcpStream::connect_timeout(&addr, self.timeout)?;
        sock.set_read_timeout(Some(self.timeout))?;
        sock.set_write_timeout(Some(self.timeout))?;
        self.sock = Some(sock);
        let login = self.send(TYPE_LOGIN, &password)?;
        match self.recv() {
            Ok((id, _)) if id == login => Ok(()),
            Ok(_) => {
                self.close();
                err("RCON-Passwort falsch")
            }
            Err(e) => {
                self.close();
                Err(e)
            }
        }
    }

    pub fn close(&mut self) {
        self.sock = None;
    }

    /// Befehl schicken und Antwort zurückgeben (ohne Farbcodes).
    pub fn command(&mut self, cmd: &str) -> Result<String> {
        let cmd = cmd.trim().trim_start_matches('/');
        let mut last = Error("RCON nicht erreichbar".into());
        for _ in 0..2 {
            if self.sock.is_none()
                && let Err(e) = self.connect()
            {
                if e.0.contains("Passwort") || e.0.contains("ausgeschaltet") {
                    return Err(e);
                }
                last = e;
                continue;
            }
            match self.send(TYPE_COMMAND, cmd).and_then(|id| {
                loop {
                    let (rid, body) = self.recv()?;
                    if rid == id {
                        return Ok(body);
                    }
                }
            }) {
                Ok(body) => return Ok(strip_formatting(&body)),
                Err(e) => {
                    self.close();
                    last = e;
                }
            }
        }
        Err(last)
    }
}

/// Entfernt Minecraft-Farbcodes (§x).
pub fn strip_formatting(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut skip = false;
    for ch in text.chars() {
        if skip {
            skip = false;
        } else if ch == '§' {
            skip = true;
        } else {
            out.push(ch);
        }
    }
    out
}

/// Einmaliger Befehl ohne offene Verbindung.
pub fn once(dir: &Path, cmd: &str) -> Result<String> {
    Rcon::new(dir).command(cmd)
}
