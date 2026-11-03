# Potato Recorder — Magyar

**Felhasználói útmutató**

Egy kis natív Windows stúdió játékmenet rögzítéséhez.

## Kezdő lépések

Telepítse az FFmpeg 8.1 vagy újabb verzióját és a megfelelő ffprobe-ot. Helyezze őket a PATH-ra vagy az alkalmazás mellé. Az alkalmazás nem tölt le eszközöket és nem tölt fel felvételeket.

Nyissa meg a Beállításokat a nyelv kiválasztásához. A Felvétel alatt válasszon ki egy kijelzőt vagy játékablakot. A Windows Graphics Capture támogatja az ablakokat és a kijelzőket; A DXGI támogatja a kijelzőket.

## Videó és előadás

Kezdje 1280 × 720 felbontással 60 FPS-sel integrált grafikán, majd mérje meg a játék képkockaidejét. Az alapértelmezett marad a forrásfelbontás. Ez egy ajánlás, nem pedig teljesítménygarancia.

Automatikusan teszteli a valódi hardveres kódolókat. A minőség megváltoztatja a kvantálást; sebesség megváltoztatja a kódoló erőfeszítéseit. A pontos tört képkockasebesség 120 FPS-ig támogatott. A tényleges forráskiadás alacsonyabb lehet.

Az alapértelmezett kompatibilitási átvitel a kereteket a RAM-ba tölti le. Ez nem nulla másolat. A többletmunkások növelhetik a vitát. A profilok soha nem változtatják meg a játékbeállításokat, vagy halkan csökkentik a felbontást vagy az FPS-t.

## Hang

A mikrofon alapértelmezés szerint ki van kapcsolva. Engedélyezze kifejezetten az Audio menüben, és válassza ki az eszközt. Az asztali hang a kiválasztott kimeneti végpontot rögzíti, nem csak egy játékot. Használj fejhallgatót.

Szükség esetén engedélyezze a vegyes hang és a független sávok útválasztását. Az eredményül kapott fájl valódi külön sávokat tartalmaz. Az RNNzaj, a kapu, a bővítő, a kompresszor, a korlátozó és az erősítés engedélyezhető és átrendezhető. A billentyűzet zaja megmaradhat.

Használja a nyolc másodperces mikrofontesztet, és játssza le a mentett tesztet a tisztaság megítéléséhez. Élő fejhallgató-felügyelet és folyamatspecifikus játékhang nincs megvalósítva.

## Felvételvezérlők

F9 elindítja vagy leállítja a felvételt. F10 szünetel vagy folytatja. Az F11 elnémítja a mikrofont. Az F7 megjeleníti vagy elrejti az alkalmazást. Az F8 elmenti az explicit módon elindított, csak videó lejátszási puffert. Az ütköző billentyűparancsok a Gyorsbillentyűkben módosíthatók.

Húzza az egyéni címsort az ablak mozgatásához. Kattintson duplán a maximalizáláshoz. A rögzítés közbeni bezárás leáll, és biztonságosan lezárja. A forrásillusztráció nem élő előnézet.

## Fájlok és helyreállítás

Az MKV az alapértelmezett. Az MP4 és a töredezett MP4 a leállítás után véglegesítésre kerül. A hangot a PCM oldalkocsik rögzítik, és leállítás után hozzáadják. A hosszú ülések befejezése időt vehet igénybe.

Mindegyik engedélyezett sztereó hangforrás körülbelül 1,38 GB-ot használ óránként ideiglenes PCM-ben, a videó és köztes másolatok mellett. A helyreállítási mappák megőrzik az eredetiket. Ellenőrizze az elkészült felvételt, mielőtt manuálisan eltávolítaná a munkamenet mappáját.

Használja a Recordings alkalmazást a fájlok megnyitásához, az MKV MP4 formátumba való újrakeveréséhez video-újrakódolás nélkül, vagy a befejezetlen munkamenetek helyreállításához. Az eredeti felvételek nem törlődnek automatikusan.

## Korlátok és ellenőrzés

A Replay jelenleg csak a videót menti. A HDR, az adaptív optimalizálás, a folyamat hangszigetelése, az élő monitorozás és az automatikus GPU-visszaállítás helyreállítása befejezetlen. Az indítási audio/video összehangolás becsült; fontos felvételek előtt ellenőrizze az ajak szinkronizálását.

A védett tartalom, néhány exkluzív teljes képernyős játék és a kicsinyített ablakok leállíthatják a keretek megjelenítését. Az Alt+Tab billentyűkombináció nyitva tartja a munkamenetet, de nem tudja eltávolítani az operációs rendszer korlátozásait.

Nem állapítottak meg teljesítményelőnyt az OBS-hez vagy az Xbox Game Barhoz képest. Tekintse meg a benchmark protokollt és a megvalósítás állapotát a reprodukálható tesztekhez és a hátralévő munkákhoz.

## Fordítási megjegyzés

Ez a nyelvi csomag gépi támogatással készült, és még nem kapott teljes anyanyelvi felülvizsgálatot. A műszaki Windows és az FFmpeg diagnosztika megtartja eredeti szövegét.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
