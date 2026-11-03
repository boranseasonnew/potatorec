# Potato Recorder — Română

**Ghidul utilizatorului**

Un mic studio nativ Windows pentru înregistrarea jocului.

## Pentru început

Instalați FFmpeg 8.1 sau mai nou și ffprobe potrivit. Pune-le pe PATH sau lângă aplicație. Aplicația nu descarcă instrumente și nu încarcă înregistrări.

Deschide Setări pentru a-ți selecta limba. În Înregistrare, selectați o fereastră de afișare sau de joc. Windows Graphics Capture acceptă ferestre și afișaje; DXGI acceptă afișaje.

## Video și performanță

Începeți cu 1280 × 720 la 60 FPS pe grafică integrată, apoi măsurați timpul de cadru al jocului. Rezoluția implicită rămâne rezoluția sursă. Aceasta este o recomandare, nu o garanție de performanță.

Testează automat codificatoarele hardware reale. Cuantificarea schimbărilor de calitate; efortul codificatorului modifică viteza. Ratele exacte de cadre fracționale sunt acceptate până la 120 FPS. Livrarea sursă reală poate fi mai mică.

Transferul de compatibilitate implicit descarcă cadre în RAM. Nu este zero-copie. Lucrătorii suplimentari pot crește disputele. Profilurile nu schimbă niciodată setările jocului și nici nu scad rezoluția sau FPS-ul.

## Audio

Microfonul este oprit implicit. Activați-l în mod explicit în Audio și alegeți dispozitivul său. Sunetul de pe desktop captează punctul final de ieșire selectat, nu doar un joc. Folosiți căști.

Activați rutarea audio mixtă și a pieselor independente după cum este necesar. Fișierul rezultat conține piese reale separate. RNNoise, poarta, expander, compresor, limitator și câștig pot fi activate și reordonate. Este posibil să rămână zgomot de la tastatură.

Utilizați testul de opt secunde pentru microfon și redați testul salvat pentru a aprecia claritatea. Monitorizarea live a căștilor și sunetul jocului specific procesului nu sunt implementate.

## Controale de înregistrare

F9 pornește sau oprește înregistrarea. F10 se întrerupe sau reia. F11 dezactivează microfonul. F7 afișează sau ascunde aplicația. F8 salvează buffer-ul de redare numai pentru videoclipuri pornit explicit. Comenzile rapide aflate în conflict pot fi modificate în Hotkeys.

Trageți bara de titlu personalizată pentru a muta fereastra. Faceți dublu clic pe el pentru a maximiza. Închiderea în timpul unei înregistrări se oprește și o finalizează în siguranță. Ilustrația sursă nu este o previzualizare live.

## Fișiere și recuperare

MKV este implicit. MP4 și MP4 fragmentat sunt finalizate după oprire. Audio este capturat în sidecar-uri PCM și adăugat după oprire. Sesiunile lungi pot dura până la finalizare.

Fiecare sursă audio stereo activată utilizează aproximativ 1,38 GB pe oră în PCM temporar, în plus față de copiile video și intermediare. Dosarele de recuperare păstrează originalele. Verificați înregistrarea finalizată înainte de a elimina manual folderul de sesiune.

Folosiți Înregistrări pentru a deschide fișiere, remix MKV în MP4 fără recodificare video sau recupera sesiuni neterminate. Înregistrările originale nu sunt șterse automat.

## Limite și verificare

În prezent, redarea salvează numai videoclipul. HDR, optimizarea adaptivă, izolarea procesului audio, monitorizarea live și recuperarea automată a GPU-ului sunt neterminate. Alinierea audio/video la pornire este estimată; verificați sincronizarea buzelor înainte de înregistrările importante.

Conținutul protejat, unele jocuri exclusive pe ecran complet și ferestrele minimizate pot să nu mai livreze cadre. Alt+Tab menține sesiunea deschisă, dar nu poate elimina restricțiile sistemului de operare.

Nu a fost stabilit niciun avantaj de performanță față de OBS sau Xbox Game Bar. Consultați protocolul de referință și starea implementării pentru teste reproductibile și munca rămasă.

## Nota de traducere

Acest pachet de limbi a fost asistat de mașină și nu a primit încă o evaluare completă a vorbitorului nativ. Diagnosticarea tehnică Windows și FFmpeg își păstrează formularea originală.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
