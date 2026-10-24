//! Minecrafts Server-Liste (servers.dat): unkomprimiertes NBT lesen, ergänzen, schreiben.
//!
//! Texte bleiben als Rohbytes erhalten (Minecraft nutzt „Modified UTF-8“) – so wird eine
//! vorhandene Liste unverändert zurückgeschrieben, auch mit Emojis in den Namen.

use crate::util::{Result, err};

const MAX_DEPTH: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(u32),
    Double(u64),
    ByteArray(Vec<u8>),
    Str(Vec<u8>),
    /// Typ der Einträge und die Einträge
    List(u8, Vec<Tag>),
    Compound(Vec<(Vec<u8>, Tag)>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Tag {
    fn id(&self) -> u8 {
        match self {
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::Str(_) => 8,
            Tag::List(..) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
            Tag::LongArray(_) => 12,
        }
    }

    fn get(&self, key: &str) -> Option<&Tag> {
        match self {
            Tag::Compound(c) => c.iter().find(|(k, _)| k == key.as_bytes()).map(|(_, v)| v),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------- Lesen

struct Reader<'a> {
    b: &'a [u8],
    pos: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8]> {
        let end = self.pos.checked_add(n).filter(|e| *e <= self.b.len());
        let Some(end) = end else {
            return err("servers.dat ist unvollständig.");
        };
        let s = &self.b[self.pos..end];
        self.pos = end;
        Ok(s)
    }
    fn arr<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut a = [0u8; N];
        a.copy_from_slice(self.take(N)?);
        Ok(a)
    }
    fn u8(&mut self) -> Result<u8> {
        Ok(self.arr::<1>()?[0])
    }
    fn len(&mut self) -> Result<usize> {
        let n = i32::from_be_bytes(self.arr()?);
        usize::try_from(n).or_else(|_| err("servers.dat enthält eine ungültige Länge."))
    }
    fn string(&mut self) -> Result<Vec<u8>> {
        let n = u16::from_be_bytes(self.arr()?) as usize;
        Ok(self.take(n)?.to_vec())
    }
    fn payload(&mut self, id: u8, depth: usize) -> Result<Tag> {
        if depth > MAX_DEPTH {
            return err("servers.dat ist zu tief verschachtelt.");
        }
        Ok(match id {
            1 => Tag::Byte(i8::from_be_bytes(self.arr()?)),
            2 => Tag::Short(i16::from_be_bytes(self.arr()?)),
            3 => Tag::Int(i32::from_be_bytes(self.arr()?)),
            4 => Tag::Long(i64::from_be_bytes(self.arr()?)),
            5 => Tag::Float(u32::from_be_bytes(self.arr()?)),
            6 => Tag::Double(u64::from_be_bytes(self.arr()?)),
            7 => {
                let n = self.len()?;
                Tag::ByteArray(self.take(n)?.to_vec())
            }
            8 => Tag::Str(self.string()?),
            9 => {
                let t = self.u8()?;
                let n = self.len()?;
                if n > self.b.len() {
                    return err("servers.dat enthält eine ungültige Länge.");
                }
                let mut v = Vec::with_capacity(n.min(1024));
                for _ in 0..n {
                    v.push(self.payload(t, depth + 1)?);
                }
                Tag::List(t, v)
            }
            10 => {
                let mut v = vec![];
                loop {
                    let t = self.u8()?;
                    if t == 0 {
                        break;
                    }
                    let name = self.string()?;
                    v.push((name, self.payload(t, depth + 1)?));
                }
                Tag::Compound(v)
            }
            11 => {
                let n = self.len()?;
                let raw = self.take(n.saturating_mul(4))?;
                Tag::IntArray(raw.as_chunks::<4>().0.iter().map(|c| i32::from_be_bytes(*c)).collect())
            }
            12 => {
                let n = self.len()?;
                let raw = self.take(n.saturating_mul(8))?;
                Tag::LongArray(raw.as_chunks::<8>().0.iter().map(|c| i64::from_be_bytes(*c)).collect())
            }
            _ => return err("servers.dat enthält einen unbekannten Eintrag."),
        })
    }
}

/// Wurzel lesen (immer ein Compound). Liefert Name und Inhalt.
pub fn read(b: &[u8]) -> Result<(Vec<u8>, Tag)> {
    let mut r = Reader { b, pos: 0 };
    if r.u8()? != 10 {
        return err("servers.dat hat ein unbekanntes Format.");
    }
    let name = r.string()?;
    let root = r.payload(10, 0)?;
    Ok((name, root))
}

// ---------------------------------------------------------------- Schreiben

fn put_string(out: &mut Vec<u8>, s: &[u8]) {
    let n = u16::try_from(s.len()).unwrap_or(u16::MAX);
    out.extend_from_slice(&n.to_be_bytes());
    out.extend_from_slice(&s[..n as usize]);
}

fn put_len(out: &mut Vec<u8>, n: usize) {
    out.extend_from_slice(&i32::try_from(n).unwrap_or(i32::MAX).to_be_bytes());
}

fn put_payload(out: &mut Vec<u8>, t: &Tag) {
    match t {
        Tag::Byte(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Short(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Int(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Long(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Float(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::Double(v) => out.extend_from_slice(&v.to_be_bytes()),
        Tag::ByteArray(v) => {
            put_len(out, v.len());
            out.extend_from_slice(v);
        }
        Tag::Str(s) => put_string(out, s),
        Tag::List(id, v) => {
            out.push(if v.is_empty() { 0 } else { *id });
            put_len(out, v.len());
            for e in v {
                put_payload(out, e);
            }
        }
        Tag::Compound(c) => {
            for (k, v) in c {
                out.push(v.id());
                put_string(out, k);
                put_payload(out, v);
            }
            out.push(0);
        }
        Tag::IntArray(v) => {
            put_len(out, v.len());
            for x in v {
                out.extend_from_slice(&x.to_be_bytes());
            }
        }
        Tag::LongArray(v) => {
            put_len(out, v.len());
            for x in v {
                out.extend_from_slice(&x.to_be_bytes());
            }
        }
    }
}

pub fn write(name: &[u8], root: &Tag) -> Vec<u8> {
    let mut out = vec![10u8];
    put_string(&mut out, name);
    put_payload(&mut out, root);
    out
}

/// Text in Minecrafts „Modified UTF-8“: NUL als zwei Bytes, Zeichen außerhalb der
/// Grundebene (Emojis) als Ersatzpaar.
pub fn mutf8(s: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    let mut put = |u: u32| {
        if u != 0 && u < 0x80 {
            out.push(u as u8);
        } else if u < 0x800 {
            out.push(0xC0 | (u >> 6) as u8);
            out.push(0x80 | (u & 0x3F) as u8);
        } else {
            out.push(0xE0 | (u >> 12) as u8);
            out.push(0x80 | ((u >> 6) & 0x3F) as u8);
            out.push(0x80 | (u & 0x3F) as u8);
        }
    };
    for c in s.chars() {
        let mut buf = [0u16; 2];
        for unit in c.encode_utf16(&mut buf) {
            put(u32::from(*unit));
        }
    }
    out
}

// ---------------------------------------------------------------- Server-Liste

fn same_address(a: &[u8], b: &str) -> bool {
    String::from_utf8_lossy(a).trim().eq_ignore_ascii_case(b.trim())
}

fn server_entry(name: &str, address: &str) -> Tag {
    Tag::Compound(vec![
        (b"name".to_vec(), Tag::Str(mutf8(name))),
        (b"ip".to_vec(), Tag::Str(mutf8(address))),
    ])
}

fn entries(root: &Tag) -> Vec<Tag> {
    match root.get("servers") {
        Some(Tag::List(10, v)) => v.clone(),
        _ => vec![],
    }
}

fn ip_of(entry: &Tag) -> Option<&[u8]> {
    match entry.get("ip") {
        Some(Tag::Str(s)) => Some(s),
        _ => None,
    }
}

/// Server-Listen zusammenführen und den eigenen Server sicher eintragen.
///
/// Die erste lesbare Liste ist die Grundlage (ihre übrigen Felder bleiben); Einträge der
/// weiteren Listen kommen dazu, wenn ihre Adresse noch fehlt. Fehlt der eigene Server,
/// steht er danach ganz oben. Unlesbare Listen werden übergangen.
pub fn servers_with(lists: &[&[u8]], name: &str, address: &str) -> Vec<u8> {
    let parsed: Vec<(Vec<u8>, Tag)> = lists
        .iter()
        .filter_map(|b| read(b).ok())
        .filter(|(_, r)| matches!(r, Tag::Compound(_)))
        .collect();
    let (root_name, mut root) = parsed
        .first()
        .cloned()
        .unwrap_or_else(|| (vec![], Tag::Compound(vec![])));
    let mut list = entries(&root);
    for (_, other) in parsed.iter().skip(1) {
        for e in entries(other) {
            let known = ip_of(&e).is_some_and(|ip| {
                list.iter()
                    .any(|x| ip_of(x).is_some_and(|y| same_address(y, &String::from_utf8_lossy(ip))))
            });
            if !known {
                list.push(e);
            }
        }
    }
    if !list.iter().any(|e| ip_of(e).is_some_and(|ip| same_address(ip, address))) {
        list.insert(0, server_entry(name, address));
    }
    if let Tag::Compound(c) = &mut root {
        c.retain(|(k, _)| k != b"servers");
        c.push((b"servers".to_vec(), Tag::List(10, list)));
    }
    write(&root_name, &root)
}

/// Adressen einer Server-Liste (für Tests).
#[cfg(test)]
pub fn addresses(b: &[u8]) -> Vec<String> {
    read(b)
        .map(|(_, r)| {
            entries(&r)
                .iter()
                .filter_map(ip_of)
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_of(entries: &[(&str, &str)]) -> Vec<u8> {
        let list = entries.iter().map(|(n, ip)| server_entry(n, ip)).collect();
        write(b"", &Tag::Compound(vec![(b"servers".to_vec(), Tag::List(10, list))]))
    }

    #[test]
    fn neue_liste_nur_mit_eigenem_server() {
        let b = servers_with(&[], "Welt", "a.b:1");
        assert_eq!(&b[..3], &[10, 0, 0]);
        assert_eq!(addresses(&b), vec!["a.b:1"]);
    }

    #[test]
    fn vorhandene_server_bleiben_und_eigener_kommt_nach_oben() {
        let old = list_of(&[("Hypixel", "mc.hypixel.net"), ("Freund", "1.2.3.4")]);
        let b = servers_with(&[&old], "E2E", "5.6.7.8:25599");
        assert_eq!(addresses(&b), vec!["5.6.7.8:25599", "mc.hypixel.net", "1.2.3.4"]);
        // schon eingetragen → nichts doppelt
        let again = servers_with(&[&b], "E2E", "5.6.7.8:25599");
        assert_eq!(addresses(&again).len(), 3);
    }

    #[test]
    fn listen_zusammenfuehren_ohne_doppelte() {
        let a = list_of(&[("A", "a.de")]);
        let b = list_of(&[("A2", "A.de"), ("B", "b.de")]);
        let m = servers_with(&[&a, &b], "S", "s.de");
        assert_eq!(addresses(&m), vec!["s.de", "a.de", "b.de"]);
    }

    #[test]
    fn unlesbare_liste_wird_uebergangen() {
        let m = servers_with(&[b"kaputt"], "S", "s.de");
        assert_eq!(addresses(&m), vec!["s.de"]);
    }

    #[test]
    fn rohbytes_und_weitere_felder_bleiben_erhalten() {
        // Eintrag mit Symbol (Modified UTF-8), Icon und Schalter wie von Minecraft geschrieben
        let entry = Tag::Compound(vec![
            (b"name".to_vec(), Tag::Str(mutf8("Burg 🏰"))),
            (b"ip".to_vec(), Tag::Str(b"burg.de".to_vec())),
            (b"icon".to_vec(), Tag::Str(b"iVBORw0".to_vec())),
            (b"acceptTextures".to_vec(), Tag::Byte(1)),
        ]);
        let old = write(b"", &Tag::Compound(vec![(b"servers".to_vec(), Tag::List(10, vec![entry.clone()]))]));
        let m = servers_with(&[&old], "S", "s.de");
        let (_, root) = read(&m).unwrap();
        assert_eq!(entries(&root)[1], entry);
    }

    #[test]
    fn modified_utf8() {
        assert_eq!(mutf8("abc"), b"abc");
        assert_eq!(mutf8("\0"), vec![0xC0, 0x80]);
        assert_eq!(mutf8("ä"), "ä".as_bytes());
        // 🏰 = U+1F3F0 → Ersatzpaar D83C DFF0, je drei Bytes
        assert_eq!(mutf8("🏰"), vec![0xED, 0xA0, 0xBC, 0xED, 0xBF, 0xB0]);
    }

    #[test]
    fn alle_typen_hin_und_zurueck() {
        let root = Tag::Compound(vec![
            (b"b".to_vec(), Tag::Byte(-1)),
            (b"s".to_vec(), Tag::Short(300)),
            (b"i".to_vec(), Tag::Int(-70000)),
            (b"l".to_vec(), Tag::Long(1 << 40)),
            (b"f".to_vec(), Tag::Float(1.5f32.to_bits())),
            (b"d".to_vec(), Tag::Double(2.5f64.to_bits())),
            (b"ba".to_vec(), Tag::ByteArray(vec![1, 2, 3])),
            (b"ia".to_vec(), Tag::IntArray(vec![1, -2])),
            (b"la".to_vec(), Tag::LongArray(vec![3, -4])),
            (b"leer".to_vec(), Tag::List(0, vec![])),
        ]);
        let b = write(b"wurzel", &root);
        assert_eq!(read(&b).unwrap(), (b"wurzel".to_vec(), root));
    }

    #[test]
    fn abgeschnittene_datei_ist_ein_fehler() {
        let b = list_of(&[("A", "a.de")]);
        assert!(read(&b[..b.len() - 3]).is_err());
        assert!(read(&[]).is_err());
    }
}
