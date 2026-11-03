# Potato Recorder — Čeština

**Uživatelská příručka**

Malé nativní Windows studio pro nahrávání hry.

## Začínáme

Nainstalujte FFmpeg 8.1 nebo novější a odpovídající ffprobe. Umístěte je na PATH nebo vedle aplikace. Aplikace nestahuje nástroje ani nenahrává nahrávky.

Otevřete Nastavení a vyberte jazyk. V Nahrávání vyberte displej nebo okno hry. Windows Graphics Capture podporuje okna a displeje; DXGI podporuje displeje.

## Video a výkon

Začněte s rozlišením 1280 × 720 při 60 FPS na integrované grafice a poté změřte časy snímků hry. Výchozí rozlišení zdroje zůstává. Toto je doporučení, nikoli záruka výkonu.

Auto testuje skutečné hardwarové kodéry. Kvantování změn kvality; rychlost změny úsilí kodéru. Přesné zlomkové snímkové frekvence jsou podporovány až do 120 FPS. Skutečná dodávka zdroje může být nižší.

Výchozí přenos kompatibility stahuje snímky do paměti RAM. Není to nulová kopie. Další pracovníci mohou zvýšit spor. Profily nikdy nemění nastavení hry ani tiše nesnižují rozlišení nebo FPS.

## Zvuk

Mikrofon je ve výchozím nastavení vypnutý. Povolte jej explicitně v Audio a vyberte jeho zařízení. Zvuk plochy zachycuje vybraný výstupní koncový bod, nejen jednu hru. Používejte sluchátka.

Podle potřeby povolte směrování smíšeného zvuku a nezávislých stop. Výsledný soubor obsahuje skutečné samostatné stopy. RNNoise, gate, expander, kompresor, limiter a gain lze povolit a změnit jejich pořadí. Hluk klávesnice může zůstat.

Použijte osmisekundový test mikrofonu a přehrajte uložený test, abyste posoudili jasnost. Není implementováno živé monitorování sluchátek a herní zvuk specifický pro proces.

## Ovládání nahrávání

F9 spustí nebo zastaví nahrávání. F10 pozastaví nebo obnoví. F11 ztlumí mikrofon. F7 zobrazí nebo skryje aplikaci. F8 uloží explicitně spuštěnou vyrovnávací paměť pro přehrávání pouze videa. Konfliktní zkratky lze změnit v Klávesové zkratky.

Přetažením vlastního záhlaví okna přesunete. Pro maximalizaci na něj dvakrát klikněte. Zavření během nahrávání zastaví a bezpečně dokončí. Zdrojová ilustrace není živý náhled.

## Soubory a obnova

MKV je výchozí. MP4 a fragmentované MP4 jsou finalizovány po zastavení. Zvuk je zachycen do postranních vozíků PCM a přidán po zastavení. Dokončení dlouhých sezení může chvíli trvat.

Každý povolený zdroj stereo zvuku využívá přibližně 1,38 GB za hodinu v dočasném PCM, navíc k videu a přechodným kopiím. Složky pro obnovení uchovávají originály. Před ručním odstraněním složky relace zkontrolujte dokončený záznam.

Pomocí Nahrávek můžete otevřít soubory, remuxovat MKV na MP4 bez překódování videa nebo obnovit nedokončené relace. Původní nahrávky nejsou automaticky vymazány.

## Limity a ověřování

Replay aktuálně ukládá pouze video. HDR, adaptivní optimalizace, procesní zvuková izolace, živé monitorování a automatická obnova GPU-reset jsou nedokončené. Odhaduje se zarovnání zvuku/videa při spuštění; před důležitými nahrávkami zkontrolujte synchronizaci rtů.

Chráněný obsah, některé exkluzivní celoobrazovkové hry a minimalizovaná okna mohou přestat zobrazovat snímky. Alt+Tab ponechá relaci otevřenou, ale nemůže odstranit omezení operačního systému.

Nebyla stanovena žádná výkonnostní výhoda oproti OBS nebo Xbox Game Bar. Pro reprodukovatelné testy a zbývající práci viz protokol benchmarku a stav implementace.

## Poznámka k překladu

Tento jazykový balíček byl podporován strojově a dosud neobdržel kompletní recenzi rodilým mluvčím. Technická diagnostika Windows a FFmpeg si zachovávají původní znění.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
