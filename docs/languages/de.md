# Potato Recorder — Deutsch

**Benutzerhandbuch**

Ein kleines natives Windows-Studio für die Gameplay-Aufzeichnung.

## Erste Schritte

Installieren Sie FFmpeg 8.1 oder neuer und die passende ffprobe. Platzieren Sie sie auf PATH oder neben der Anwendung. Die App lädt keine Tools herunter und lädt keine Aufzeichnungen hoch.

Öffnen Sie die Einstellungen, um Ihre Sprache auszuwählen. Wählen Sie unter „Aufzeichnung“ eine Anzeige oder ein Spielfenster aus. Windows Graphics Capture unterstützt Fenster und Anzeigen; DXGI unterstützt Anzeigen.

## Video und Performance

Beginnen Sie mit 1280 × 720 bei 60 FPS auf integrierter Grafik und messen Sie dann die Frametimes des Spiels. Die Standardauflösung bleibt die Quellauflösung. Dies ist eine Empfehlung, keine Leistungsgarantie.

Automatische Tests echter Hardware-Encoder. Qualitätsänderungen Quantisierung; Geschwindigkeit ändert Encoderaufwand. Exakte Teilbildraten werden bis zu 120 FPS unterstützt. Die tatsächliche Quelllieferung kann niedriger sein.

Die standardmäßige Kompatibilitätsübertragung lädt Frames in den RAM herunter. Es handelt sich nicht um eine Nullkopie. Zusätzliche Arbeitskräfte können den Konflikt verstärken. Profile ändern niemals Spieleinstellungen oder verringern stillschweigend die Auflösung oder FPS.

## Audio

Das Mikrofon ist standardmäßig ausgeschaltet. Aktivieren Sie es explizit in Audio und wählen Sie sein Gerät aus. Desktop-Sound erfasst den ausgewählten Ausgabeendpunkt, nicht nur ein Spiel. Benutzen Sie Kopfhörer.

Aktivieren Sie nach Bedarf Mixed Audio und Independent Track Routing. Die resultierende Datei enthält echte separate Tracks. RNNoise, Gate, Expander, Kompressor, Limiter und Gain können aktiviert und neu angeordnet werden. Möglicherweise bleiben Tastaturgeräusche bestehen.

Verwenden Sie den acht Sekunden dauernden Mikrofontest und spielen Sie den gespeicherten Test ab, um die Klarheit zu beurteilen. Live-Kopfhörerüberwachung und prozessspezifisches Spielaudio sind nicht implementiert.

## Aufnahmesteuerung

F9 startet oder stoppt die Aufnahme. F10 pausiert oder setzt fort. F11 schaltet das Mikrofon stumm. Mit F7 wird die Anwendung angezeigt oder ausgeblendet. F8 speichert den explizit gestarteten Nur-Video-Wiedergabepuffer. Widersprüchliche Tastenkombinationen können in Hotkeys geändert werden.

Ziehen Sie die benutzerdefinierte Titelleiste, um das Fenster zu verschieben. Doppelklicken Sie darauf, um es zu maximieren. Das Schließen während einer Aufnahme stoppt und schließt diese sicher ab. Bei der Quellillustration handelt es sich nicht um eine Live-Vorschau.

## Dateien und Wiederherstellung

MKV ist die Standardeinstellung. MP4 und fragmentiertes MP4 werden nach dem Stoppen finalisiert. Audio wird in PCM-Sidecars erfasst und nach dem Stopp hinzugefügt. Bei langen Sitzungen kann es einige Zeit dauern, bis sie abgeschlossen sind.

Jede aktivierte Stereo-Audioquelle verbraucht zusätzlich zu Video und Zwischenkopien etwa 1,38 GB pro Stunde im temporären PCM. In Wiederherstellungsordnern werden die Originale aufbewahrt. Überprüfen Sie die abgeschlossene Aufzeichnung, bevor Sie den Sitzungsordner manuell entfernen.

Verwenden Sie Aufnahmen, um Dateien zu öffnen, MKV in MP4 umzuwandeln, ohne das Video neu zu kodieren, oder nicht abgeschlossene Sitzungen wiederherzustellen. Originalaufnahmen werden nicht automatisch gelöscht.

## Grenzen und Überprüfung

Replay speichert derzeit nur Videos. HDR, adaptive Optimierung, Prozess-Audio-Isolation, Live-Überwachung und automatische GPU-Reset-Wiederherstellung sind noch nicht abgeschlossen. Die Audio-/Videoausrichtung beim Start wird geschätzt; Überprüfen Sie vor wichtigen Aufnahmen die Lippensynchronisation.

Geschützte Inhalte, einige exklusive Vollbildspiele und minimierte Fenster liefern möglicherweise keine Frames mehr. Alt+Tab hält die Sitzung geöffnet, kann jedoch Betriebssystemeinschränkungen nicht aufheben.

Es wurde kein Leistungsvorteil gegenüber OBS oder der Xbox Game Bar festgestellt. Informationen zu reproduzierbaren Tests und verbleibenden Arbeiten finden Sie im Benchmark-Protokoll und im Implementierungsstatus.

## Übersetzungshinweis

Dieses Sprachpaket wurde maschinell unterstützt und hat noch keine vollständige Überprüfung durch Muttersprachler erhalten. Technische Windows- und FFmpeg-Diagnosen behalten ihren ursprünglichen Wortlaut.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
