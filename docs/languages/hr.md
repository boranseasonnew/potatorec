# Potato Recorder — Hrvatski

**Upute za upotrebu**

Mali nativni Windows studio za snimanje gameplaya.

## Početak rada

Instalirajte FFmpeg 8.1 ili noviji i odgovarajući ffprobe. Stavite ih na PATH ili pored aplikacije. Aplikacija ne preuzima alate niti prenosi snimke.

Otvorite Postavke za odabir jezika. U Snimanju odaberite zaslon ili prozor igre. Windows Graphics Capture podržava prozore i zaslone; DXGI podržava zaslone.

## Video i izvedba

Počnite s 1280 × 720 pri 60 FPS na integriranoj grafici, a zatim izmjerite vrijeme okvira igre. Zadana je rezolucija izvora. Ovo je preporuka, a ne jamstvo izvedbe.

Automatski testira prave hardverske kodere. Kvantizacija promjena kvalitete; brzina se mijenja koder napor. Točne frakcijske brzine sličica u sekundi podržane su do 120 FPS. Stvarna isporuka izvora može biti manja.

Zadani prijenos kompatibilnosti preuzima okvire u RAM. Nije nulta kopija. Dodatni radnici mogu povećati sukobe. Profili nikada ne mijenjaju postavke igre niti tiho smanjuju rezoluciju ili FPS.

## Audio

Mikrofon je prema zadanim postavkama isključen. Omogućite ga eksplicitno u Audio i odaberite njegov uređaj. Zvuk stolnog računala snima odabranu krajnju točku izlaza, a ne samo jednu igru. Koristite slušalice.

Po potrebi omogućite usmjeravanje miješanog zvuka i neovisnog zapisa. Rezultirajuća datoteka sadrži prave zasebne pjesme. RNNoise, gate, expander, kompresor, limiter i gain mogu se omogućiti i promijeniti redoslijed. Može ostati šum tipkovnice.

Koristite test mikrofona od osam sekundi i reproducirajte spremljeni test da biste procijenili jasnoću. Praćenje uživo putem slušalica i zvuk igre specifičan za proces nisu implementirani.

## Kontrole snimanja

F9 pokreće ili zaustavlja snimanje. F10 pauzira ili nastavlja. F11 isključuje mikrofon. F7 prikazuje ili skriva aplikaciju. F8 sprema eksplicitno pokrenuti međuspremnik za ponovnu reprodukciju samo videa. Konfliktni prečaci mogu se promijeniti u Hotkeys.

Povucite prilagođenu naslovnu traku za pomicanje prozora. Kliknite dvaput za maksimiziranje. Zatvaranje tijekom snimanja zaustavlja i sigurno ga finalizira. Izvorna ilustracija nije pregled uživo.

## Datoteke i oporavak

MKV je zadani. MP4 i fragmentirani MP4 finaliziraju se nakon zaustavljanja. Audio se snima u PCM bočne prikolice i dodaje nakon zaustavljanja. Završetak dugih sesija može potrajati.

Svaki omogućeni izvor stereo zvuka koristi oko 1,38 GB po satu u privremenom PCM-u, uz video i međukopije. Mape za oporavak čuvaju izvornike. Prije ručnog uklanjanja mape sesije provjerite dovršeno snimanje.

Koristite snimke za otvaranje datoteka, remux MKV u MP4 bez ponovnog kodiranja videa ili oporavak nedovršenih sesija. Izvorne snimke ne brišu se automatski.

## Ograničenja i provjera

Replay trenutno sprema samo video. HDR, adaptivna optimizacija, audio izolacija procesa, praćenje uživo i automatski oporavak GPU-a poništavanjem nisu dovršeni. Procjenjuje se audio/video usklađivanje pri pokretanju; provjerite sinkronizaciju usana prije važnih snimanja.

Zaštićeni sadržaj, neke ekskluzivne igre preko cijelog zaslona i minimizirani prozori mogu prestati isporučivati ​​okvire. Alt+Tab drži sesiju otvorenom, ali ne može ukloniti ograničenja operativnog sustava.

Nije utvrđena prednost performansi u odnosu na OBS ili Xbox Game Bar. Pogledajte referentni protokol i status implementacije za ponovljive testove i preostali rad.

## Napomena o prijevodu

Ovaj je jezični paket potpomognut strojno i još nije dobio potpuni pregled izvornih govornika. Tehnička dijagnostika sustava Windows i FFmpeg zadržava svoj izvorni tekst.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
