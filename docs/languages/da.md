# Potato Recorder — Dansk

**Brugervejledning**

Et lille indbygget Windows-studie til gameplay-optagelse.

## Kom godt i gang

Installer FFmpeg 8.1 eller nyere og den matchende ffprobe. Sæt dem på PATH eller ved siden af ​​applikationen. Appen downloader ikke værktøjer eller uploader optagelser.

Åbn Indstillinger for at vælge dit sprog. I Optagelse skal du vælge et display eller et spilvindue. Windows Graphics Capture understøtter vinduer og skærme; DXGI understøtter skærme.

## Video og performance

Start med 1280 × 720 ved 60 FPS på integreret grafik, og mål derefter spilletider. Standarden forbliver kildeopløsning. Dette er en anbefaling, ikke en ydeevnegaranti.

Autotester rigtige hardwarekodere. Kvalitetsændringer kvantisering; hastighed ændrer encoderindsats. Nøjagtige billedhastigheder i brøkdel understøttes op til 120 FPS. Faktisk kildelevering kan være lavere.

Standard kompatibilitetsoverførsel downloader rammer til RAM. Det er ikke nul-kopi. Ekstra arbejdskraft kan øge striden. Profiler ændrer aldrig spilindstillinger eller sænker lydløst opløsning eller FPS.

## Lyd

Mikrofonen er som standard slukket. Aktiver det eksplicit i Audio, og vælg dets enhed. Skrivebordslyd fanger det valgte output-endepunkt, ikke kun ét spil. Brug høretelefoner.

Aktiver Mixed Audio og Independent Track routing efter behov. Den resulterende fil indeholder rigtige separate spor. RNNoise, gate, expander, kompressor, limiter og gain kan aktiveres og omarrangeres. Tastaturstøj kan forblive.

Brug otte sekunders mikrofontest og afspil den gemte test for at bedømme klarheden. Live hovedtelefonovervågning og processpecifik spillyd er ikke implementeret.

## Optagelseskontroller

F9 starter eller stopper optagelsen. F10 holder pause eller genoptager. F11 slår mikrofonen fra. F7 viser eller skjuler applikationen. F8 gemmer den eksplicit startede video-only replay buffer. Modstridende genveje kan ændres i genvejstaster.

Træk den tilpassede titellinje for at flytte vinduet. Dobbeltklik på det for at maksimere. Lukning under en optagelse stopper og afslutter den sikkert. Kildeillustrationen er ikke en live forhåndsvisning.

## Filer og gendannelse

MKV er standard. MP4 og fragmenteret MP4 afsluttes efter stop. Lyden optages i PCM sidevogne og tilføjes efter stop. Lange sessioner kan tage tid at afslutte.

Hver aktiveret stereolydkilde bruger omkring 1,38 GB i timen i midlertidig PCM, foruden video og mellemliggende kopier. Gendannelsesmapper beholder originaler. Kontroller den afsluttede optagelse, før du manuelt fjerner dens sessionsmappe.

Brug Optagelser til at åbne filer, remux MKV til MP4 uden videoomkodning eller gendanne ufærdige sessioner. Originale optagelser slettes ikke automatisk.

## Grænser og verifikation

Genafspilning gemmer i øjeblikket kun video. HDR, adaptiv optimering, proceslydisolering, liveovervågning og automatisk gendannelse af GPU-nulstilling er ufærdige. Opstart lyd/video justering er estimeret; tjek læbesynkronisering før vigtige optagelser.

Beskyttet indhold, nogle eksklusive fuldskærmsspil og minimerede vinduer kan stoppe med at levere rammer. Alt+Tab holder sessionen åben, men kan ikke fjerne operativsystemets begrænsninger.

Der er ikke etableret nogen præstationsfordel i forhold til OBS eller Xbox Game Bar. Se benchmark-protokollen og implementeringsstatus for reproducerbare test og resterende arbejde.

## Oversættelsesnote

Denne sprogpakke blev maskinassisteret og har endnu ikke modtaget en komplet anmeldelse som modersmål. Tekniske Windows- og FFmpeg-diagnostik bevarer deres oprindelige ordlyd.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
