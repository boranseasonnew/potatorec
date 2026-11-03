# Potato Recorder — Slovenčina

**Používateľská príručka**

Malé natívne Windows štúdio na nahrávanie hier.

## Začíname

Nainštalujte FFmpeg 8.1 alebo novší a zodpovedajúci ffprobe. Umiestnite ich na PATH alebo vedľa aplikácie. Aplikácia nesťahuje nástroje ani nenahráva nahrávky.

Otvorte Nastavenia a vyberte jazyk. V časti Nahrávanie vyberte zobrazenie alebo okno hry. Windows Graphics Capture podporuje okná a displeje; DXGI podporuje displeje.

## Video a výkon

Začnite s rozlíšením 1 280 × 720 pri 60 FPS na integrovanej grafike a potom zmerajte časy snímok hry. Predvolené zostáva rozlíšenie zdroja. Toto je odporúčanie, nie záruka výkonu.

Automatické testovanie skutočných hardvérových kódovačov. Kvantovanie zmien kvality; rýchlosť zmeny úsilia kodéra. Presné zlomkové obnovovacie frekvencie sú podporované až do 120 FPS. Skutočné dodanie zdroja môže byť nižšie.

Predvolený prenos kompatibility sťahuje snímky do pamäte RAM. Nie je to nulová kópia. Ďalší pracovníci môžu zvýšiť spor. Profily nikdy nemenia nastavenia hry ani neznižujú rozlíšenie alebo FPS.

## Zvuk

Mikrofón je predvolene vypnutý. Povoľte ho explicitne v časti Zvuk a vyberte jeho zariadenie. Zvuk pracovnej plochy zachytáva vybraný výstupný koncový bod, nielen jednu hru. Používajte slúchadlá.

Podľa potreby povoľte smerovanie zmiešaného zvuku a nezávislých skladieb. Výsledný súbor obsahuje skutočné samostatné stopy. RNNoise, gate, expander, kompresor, limiter a gain je možné povoliť a zmeniť ich poradie. Hluk klávesnice môže zostať.

Použite osemsekundový test mikrofónu a prehrajte uložený test, aby ste posúdili jasnosť. Živé monitorovanie slúchadiel a zvuk hry špecifický pre proces nie sú implementované.

## Ovládacie prvky nahrávania

F9 spustí alebo zastaví nahrávanie. F10 pozastaví alebo obnoví. F11 stlmí mikrofón. F7 zobrazí alebo skryje aplikáciu. F8 uloží explicitne spustenú vyrovnávaciu pamäť prehrávania iba videa. Konfliktné skratky je možné zmeniť v Klávesových skratkách.

Potiahnutím vlastného záhlaví okna presuniete. Dvojitým kliknutím ho maximalizujete. Zatvorenie počas nahrávania sa zastaví a bezpečne dokončí. Zdrojová ilustrácia nie je živý náhľad.

## Súbory a obnovenie

MKV je predvolená hodnota. MP4 a fragmentované MP4 sa finalizujú po zastavení. Zvuk sa zachytáva do postranných vozíkov PCM a pridáva sa po zastavení. Dokončenie dlhých sedení môže chvíľu trvať.

Každý povolený zdroj stereo zvuku využíva približne 1,38 GB za hodinu v dočasnom PCM, okrem videa a prechodných kópií. Priečinky na obnovenie uchovávajú originály. Pred ručným odstránením priečinka relácie skontrolujte dokončený záznam.

Pomocou Nahrávok môžete otvárať súbory, remuxovať MKV na MP4 bez prekódovania videa alebo obnovovať nedokončené relácie. Pôvodné nahrávky sa automaticky nevymažú.

## Limity a overovanie

Replay momentálne ukladá iba video. HDR, adaptívna optimalizácia, izolácia procesného zvuku, živé monitorovanie a automatická obnova GPU-reset sú nedokončené. Odhaduje sa zarovnanie zvuku/videa pri spustení; pred dôležitými nahrávkami skontrolujte synchronizáciu pier.

Chránený obsah, niektoré exkluzívne hry na celú obrazovku a minimalizované okná môžu prestať zobrazovať snímky. Alt+Tab ponechá reláciu otvorenú, ale nedokáže odstrániť obmedzenia operačného systému.

Nebola stanovená žiadna výkonnostná výhoda oproti OBS alebo Xbox Game Bar. Pozrite si protokol benchmarku a stav implementácie pre reprodukovateľné testy a zostávajúcu prácu.

## Poznámka k prekladu

Tento jazykový balík bol podporovaný strojom a zatiaľ nezískal úplnú kontrolu rodeným hovorcom. Technická diagnostika Windows a FFmpeg si zachováva svoje pôvodné znenie.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
