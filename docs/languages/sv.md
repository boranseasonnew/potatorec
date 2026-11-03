# Potato Recorder — Svenska

**Användarhandbok**

En liten inbyggd Windows-studio för spelinspelning.

## Komma igång

Installera FFmpeg 8.1 eller nyare och den matchande ffprobe. Lägg dem på PATH eller bredvid applikationen. Appen laddar inte ner verktyg eller laddar upp inspelningar.

Öppna Inställningar för att välja ditt språk. I Inspelning väljer du ett skärm- eller spelfönster. Windows Graphics Capture stöder fönster och bildskärmar; DXGI stöder skärmar.

## Video och prestanda

Börja med 1280 × 720 vid 60 FPS på integrerad grafik, mät sedan spelets tider. Standardinställningen förblir källupplösningen. Detta är en rekommendation, inte en prestandagaranti.

Autotestar riktiga hårdvarukodare. Kvalitetsförändringar kvantisering; hastighet ändrar kodaransträngning. Exakta delbildhastigheter stöds upp till 120 FPS. Faktisk källleverans kan vara lägre.

Standardkompatibilitetsöverföringen laddar ner ramar till RAM. Det är inte nollkopia. Extraarbetare kan öka konflikten. Profiler ändrar aldrig spelinställningar och sänker aldrig upplösningen eller FPS.

## Ljud

Mikrofonen är avstängd som standard. Aktivera det uttryckligen i Audio och välj dess enhet. Skrivbordsljud fångar den valda slutpunkten, inte bara ett spel. Använd hörlurar.

Aktivera blandat ljud och oberoende spårdirigering efter behov. Den resulterande filen innehåller riktiga separata spår. RNNoise, gate, expander, kompressor, limiter och gain kan aktiveras och omordnas. Tangentbordsljud kan finnas kvar.

Använd det åtta sekunder långa mikrofontestet och spela det sparade testet för att bedöma klarheten. Live hörlursövervakning och processspecifikt spelljud implementeras inte.

## Inspelningskontroller

F9 startar eller stoppar inspelningen. F10 pausar eller fortsätter. F11 tystar mikrofonen. F7 visar eller döljer applikationen. F8 sparar den explicit startade uppspelningsbufferten för endast video. Motstridiga genvägar kan ändras i snabbtangenter.

Dra den anpassade titelraden för att flytta fönstret. Dubbelklicka på den för att maximera. Stängning under en inspelning stoppar och avslutar den på ett säkert sätt. Källillustrationen är inte en liveförhandsvisning.

## Filer och återställning

MKV är standard. MP4 och fragmenterad MP4 slutförs efter stopp. Ljud fångas in i PCM sidovagnar och läggs till efter stopp. Långa sessioner kan ta tid att slutföra.

Varje aktiverad stereoljudkälla använder cirka 1,38 GB per timme i temporär PCM, förutom video och mellankopior. Återställningsmappar behåller originalen. Kontrollera den slutförda inspelningen innan du manuellt tar bort dess sessionsmapp.

Använd Recordings för att öppna filer, remuxa MKV till MP4 utan videoomkodning eller återställa oavslutade sessioner. Originalinspelningar raderas inte automatiskt.

## Begränsningar och verifiering

Omspelning sparar för närvarande endast video. HDR, adaptiv optimering, processljudisolering, liveövervakning och automatisk GPU-återställning är oavslutade. Startljud/videojustering är uppskattad; kontrollera läppsynk före viktiga inspelningar.

Skyddat innehåll, vissa exklusiva helskärmsspel och minimerade fönster kan sluta leverera ramar. Alt+Tab håller sessionen öppen, men kan inte ta bort operativsystembegränsningar.

Ingen prestandafördel jämfört med OBS eller Xbox Game Bar har fastställts. Se benchmarkprotokollet och implementeringsstatus för reproducerbara tester och återstående arbete.

## Översättningsanteckning

Det här språkpaketet var maskinassisterat och har ännu inte fått en fullständig recension som modersmål. Teknisk Windows- och FFmpeg-diagnostik behåller sin ursprungliga formulering.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
