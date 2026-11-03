# Potato Recorder — Nederlands

**Gebruikershandleiding**

Een kleine native Windows-studio voor het opnemen van gameplay.

## Aan de slag

Installeer FFmpeg 8.1 of nieuwer en de bijpassende ffprobe. Zet ze op PATH of naast de applicatie. De app downloadt geen tools en uploadt geen opnames.

Open Instellingen om uw taal te selecteren. Selecteer in Opname een weergave- of gamevenster. Windows Graphics Capture ondersteunt vensters en beeldschermen; DXGI ondersteunt beeldschermen.

## Video en prestaties

Begin met 1280 × 720 bij 60 FPS op geïntegreerde graphics en meet vervolgens de gameframetijden. De standaardwaarde blijft de bronresolutie. Dit is een aanbeveling, geen prestatiegarantie.

Test automatisch echte hardware-encoders. Kwantisering van kwaliteitsveranderingen; snelheidsveranderingen encoderinspanning. Exacte fractionele framesnelheden worden ondersteund tot 120 FPS. De werkelijke bronlevering kan lager zijn.

De standaard compatibiliteitsoverdracht downloadt frames naar RAM. Het is geen zero-copy. Extra werknemers kunnen de strijd vergroten. Profielen veranderen nooit de spelinstellingen en verlagen nooit stilletjes de resolutie of FPS.

## Audio

De microfoon is standaard uitgeschakeld. Schakel het expliciet in Audio in en kies het apparaat ervan. Desktopgeluid legt het geselecteerde uitvoereindpunt vast, niet slechts één game. Gebruik een koptelefoon.

Schakel indien nodig gemengde audio en onafhankelijke trackroutering in. Het resulterende bestand bevat echte afzonderlijke tracks. RNNoise, gate, expander, compressor, limiter en gain kunnen worden ingeschakeld en opnieuw worden geordend. Het toetsenbordgeluid kan aanhouden.

Gebruik de microfoontest van acht seconden en speel de opgeslagen test af om de helderheid te beoordelen. Live hoofdtelefoonmonitoring en processpecifieke gameaudio zijn niet geïmplementeerd.

## Opnamebedieningen

F9 start of stopt de opname. F10 pauzeert of hervat. F11 dempt de microfoon. F7 toont of verbergt de applicatie. F8 slaat de expliciet gestarte afspeelbuffer voor alleen video op. Conflicterende sneltoetsen kunnen worden gewijzigd in Sneltoetsen.

Versleep de aangepaste titelbalk om het venster te verplaatsen. Dubbelklik erop om te maximaliseren. Het sluiten tijdens een opname stopt en rondt deze veilig af. De bronillustratie is geen live preview.

## Bestanden en herstel

MKV is de standaard. MP4 en gefragmenteerde MP4 worden na het stoppen gefinaliseerd. Audio wordt vastgelegd in PCM-zijspannen en na de stop toegevoegd. Het kan enige tijd duren voordat lange sessies zijn afgerond.

Elke ingeschakelde stereo-audiobron gebruikt ongeveer 1,38 GB per uur aan tijdelijke PCM, naast video en tussentijdse kopieën. Herstelmappen bewaren originelen. Controleer de voltooide opname voordat u de sessiemap handmatig verwijdert.

Gebruik Opnames om bestanden te openen, MKV naar MP4 te remuxen zonder video opnieuw te coderen, of onvoltooide sessies te herstellen. Originele opnamen worden niet automatisch verwijderd.

## Limieten en verificatie

Bij opnieuw afspelen wordt momenteel alleen video opgeslagen. HDR, adaptieve optimalisatie, procesaudio-isolatie, live monitoring en automatisch GPU-resetherstel zijn nog niet voltooid. Opstartaudio/video-uitlijning wordt geschat; controleer lipsynchronisatie vóór belangrijke opnames.

Het is mogelijk dat beveiligde inhoud, sommige exclusieve games op volledig scherm en geminimaliseerde vensters geen frames meer leveren. Alt+Tab houdt de sessie open, maar kan de beperkingen van het besturingssysteem niet verwijderen.

Er is geen prestatievoordeel ten opzichte van OBS of Xbox Game Bar vastgesteld. Bekijk het benchmarkprotocol en de implementatiestatus voor reproduceerbare tests en resterend werk.

## Vertaling nota

Dit taalpakket is machinaal ondersteund en heeft nog geen volledige beoordeling door de moedertaalspreker gekregen. Technische Windows- en FFmpeg-diagnostiek behouden hun oorspronkelijke bewoording.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
