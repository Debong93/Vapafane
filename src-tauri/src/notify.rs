//! Stille Benachrichtigungen (ohne Ton) – für Server-Ereignisse und den Client-Wächter.

/// Meldung zeigen, ohne dass ein Ton abgespielt wird. Fehler werden ignoriert.
pub fn silent(title: &str, body: &str) {
    let mut n = notify_rust::Notification::new();
    n.summary(title).body(body).appname("Vapafane");
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        n.icon("vapafane").hint(notify_rust::Hint::SuppressSound(true));
    }
    #[cfg(windows)]
    {
        // Ohne sound_name ist die Meldung unter Windows stumm; installierte App per Kennung zuordnen
        if !std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.ends_with("debug") || d.ends_with("release")))
            .unwrap_or(false)
        {
            n.app_id("de.debong93.vapafane");
        }
    }
    let _ = n.show();
}
