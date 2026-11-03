# Potato Recorder — Polski

**Podręcznik użytkownika**

Małe, natywne studio Windows do nagrywania rozgrywki.

## Rozpoczęcie

Zainstaluj FFmpeg 8.1 lub nowszy i pasującą sondę ffprobe. Umieść je na PATH lub obok aplikacji. Aplikacja nie pobiera narzędzi ani nie przesyła nagrań.

Otwórz Ustawienia, aby wybrać język. W Nagrywaniu wybierz wyświetlacz lub okno gry. Windows Graphics Capture obsługuje okna i wyświetlacze; DXGI obsługuje wyświetlacze.

## Wideo i występ

Zacznij od rozdzielczości 1280 × 720 przy 60 FPS na zintegrowanej karcie graficznej, a następnie zmierz liczbę klatek na sekundę w grze. Domyślną rozdzielczością pozostaje rozdzielczość źródłowa. Jest to zalecenie, a nie gwarancja wydajności.

Automatyczne testy rzeczywistych koderów sprzętowych. Jakość zmienia kwantyzację; prędkość zmienia wysiłek kodera. Dokładne ułamkowe liczby klatek na sekundę są obsługiwane do 120 FPS. Rzeczywista podaż źródła może być niższa.

Domyślny transfer zgodności pobiera ramki do pamięci RAM. To nie jest kopia zerowa. Dodatkowi pracownicy mogą zwiększyć rywalizację. Profile nigdy nie zmieniają ustawień gry ani nie zmniejszają po cichu rozdzielczości ani liczby klatek na sekundę.

## Dźwięk

Mikrofon jest domyślnie wyłączony. Włącz to jawnie w Audio i wybierz jego urządzenie. Dźwięk pulpitu przechwytuje wybrany punkt końcowy, a nie tylko jedną grę. Użyj słuchawek.

W razie potrzeby włącz routing mieszanego dźwięku i niezależnej ścieżki. Wynikowy plik zawiera naprawdę oddzielne ścieżki. RNNas, bramkę, ekspander, kompresor, limiter i wzmocnienie można włączyć i zmienić ich kolejność. Dźwięk klawiatury może pozostać.

Skorzystaj z ośmiosekundowego testu mikrofonu i odtwórz zapisany test, aby ocenić klarowność. Monitorowanie słuchawek na żywo i dźwięk w grze specyficzny dla procesu nie są zaimplementowane.

## Sterowanie nagrywaniem

F9 rozpoczyna lub kończy nagrywanie. F10 wstrzymuje lub wznawia. F11 wycisza mikrofon. F7 pokazuje lub ukrywa aplikację. F8 zapisuje jawnie uruchomiony bufor odtwarzania tylko wideo. Skomplikowane skróty można zmienić w Hotkeys.

Przeciągnij niestandardowy pasek tytułu, aby przesunąć okno. Kliknij dwukrotnie, aby zmaksymalizować. Zamknięcie w trakcie nagrywania zatrzymuje się i bezpiecznie je finalizuje. Ilustracja źródłowa nie jest podglądem na żywo.

## Pliki i odzyskiwanie

MKV jest domyślny. MP4 i pofragmentowany MP4 są finalizowane po zatrzymaniu. Dźwięk jest przechwytywany do wózków bocznych PCM i dodawany po zatrzymaniu. Zakończenie długich sesji może zająć trochę czasu.

Każde włączone źródło dźwięku stereo zużywa około 1,38 GB na godzinę w tymczasowym formacie PCM, oprócz kopii wideo i pośrednich. Foldery odzyskiwania przechowują oryginały. Sprawdź ukończone nagranie przed ręcznym usunięciem folderu sesji.

Użyj Nagrań, aby otwierać pliki, remuxować MKV do MP4 bez ponownego kodowania wideo lub odzyskiwać niedokończone sesje. Oryginalne nagrania nie są automatycznie usuwane.

## Limity i weryfikacja

Funkcja Replay obecnie zapisuje tylko wideo. HDR, optymalizacja adaptacyjna, izolacja dźwięku procesowego, monitorowanie na żywo i automatyczne przywracanie po zresetowaniu GPU są niedokończone. Szacowane jest początkowe dopasowanie audio/wideo; sprawdź synchronizację ruchu warg przed ważnymi nagraniami.

Chroniona zawartość, niektóre ekskluzywne gry pełnoekranowe i zminimalizowane okna mogą przestać dostarczać klatki. Alt+Tab utrzymuje sesję otwartą, ale nie może usunąć ograniczeń systemu operacyjnego.

Nie stwierdzono żadnej przewagi wydajnościowej nad OBS lub Xbox Game Bar. Zobacz protokół testu porównawczego i stan wdrożenia, aby uzyskać powtarzalne testy i pozostałą pracę.

## Notatka tłumaczeniowa

Ten pakiet językowy był wspomagany maszynowo i nie otrzymał jeszcze pełnej recenzji w języku rodzimym. Diagnostyka techniczna Windows i FFmpeg zachowuje swoje oryginalne brzmienie.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
