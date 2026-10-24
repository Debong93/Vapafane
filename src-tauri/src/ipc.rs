//! Verbindung zwischen Fenster und Hintergrund-Dienst.
//!
//! Der Dienst lauscht nur auf 127.0.0.1 an einem zufälligen Port. Port und ein
//! Zufalls-Schlüssel stehen in `service.json` im Konfigurationsordner, die nur
//! der eigene Benutzer lesen kann. Jede Anfrage ist eine JSON-Zeile.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use crate::config;
use crate::util::{self, Error, Result, err};

#[derive(Serialize, Deserialize)]
pub struct ServiceFile {
    pub port: u16,
    pub token: String,
    pub pid: u32,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub token: String,
    pub cmd: String,
    #[serde(default)]
    pub arg: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub data: Value,
}

pub fn service_file() -> std::path::PathBuf {
    config::state_dir().join("service.json")
}

pub fn write_service_file(sf: &ServiceFile) -> Result<()> {
    let path = service_file();
    util::write_atomic(&path, serde_json::to_string(sf)?.as_bytes())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

/// Anfrage an den Dienst. Fehler, wenn er nicht läuft.
pub fn call(cmd: &str, arg: Value) -> Result<Value> {
    call_at(&service_file(), cmd, arg)
}

/// Wie [`call`], aber über eine bestimmte service.json (z. B. die der Vorgängerversion).
pub fn call_at(file: &std::path::Path, cmd: &str, arg: Value) -> Result<Value> {
    let Some(sf) = std::fs::read_to_string(file)
        .ok()
        .and_then(|t| serde_json::from_str::<ServiceFile>(&t).ok())
    else {
        return err(crate::sprache::t!("err-dienst-aus"));
    };
    let addr = SocketAddr::from(([127, 0, 0, 1], sf.port));
    let mut sock = TcpStream::connect_timeout(&addr, Duration::from_millis(800))
        .map_err(|_| Error(crate::sprache::t!("err-dienst-aus")))?;
    sock.set_read_timeout(Some(Duration::from_secs(15)))?;
    let req = Request {
        token: sf.token,
        cmd: cmd.into(),
        arg,
    };
    let mut line = serde_json::to_string(&req)?;
    line.push('\n');
    sock.write_all(line.as_bytes())?;
    let mut resp = String::new();
    BufReader::new(sock).read_line(&mut resp)?;
    let resp: Response = serde_json::from_str(&resp).map_err(|_| Error(crate::sprache::t!("err-dienst-antwort")))?;
    if resp.ok {
        Ok(resp.data)
    } else {
        Err(Error(resp.error.unwrap_or_else(|| crate::sprache::t!("err-dienst-fehler"))))
    }
}

pub fn running() -> bool {
    call("ping", Value::Null).is_ok()
}
