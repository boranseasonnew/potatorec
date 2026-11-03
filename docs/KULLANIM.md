# Potato Recorder — kullanım

Bu sürüm gerçek kayıt alır; bütün ürün gereksinimlerinin tamamlandığı anlamına gelmez. Özellikle oyun performansı ve uzun kayıtlarda ses/görüntü uyumu kendi bilgisayarında sınanmalı.

1. `potato-recorder.exe` dosyasını aç. FFmpeg 8.1+ ve ffprobe sistem PATH'inde veya uygulamanın yanında olmalı. Gerekirse Ayarlar'dan FFmpeg'in tam yolunu yaz.
2. **Kayıt** ekranından monitör ya da oyunun penceresini seç. Oyun/pencere için WGC; tüm ekran için WGC veya DXGI kullanabilirsin.
3. **Video** ekranında çözünürlük ve FPS seç. Vega 8 gibi dahili grafiklerde ilk deneme için 1280×720 / 60 FPS makul bir başlangıçtır; daha iyi performans garantisi değildir.
4. Kodlayıcıyı Otomatik bırakabilirsin. Uygulama gerçek kısa kodlama testi geçen donanım seçeneklerini gösterir. Varsayılan uyumluluk yolu GPU'dan RAM'e aktarım yapar; sıfır kopya değildir.
5. **Ses** ekranında mikrofonu açıkça etkinleştir. Masaüstü sesi ve mikrofon ayrı aygıtlardan seçilir. Karma kanala dahil etme ve bağımsız kanal seçenekleri dosyanın gerçek ses kanallarını değiştirir.
6. Mikrofon testi sekiz saniye sürer; sonucu **Son testi dinle** ile aç. Clean Voice RNNoise'u etkinleştirir. Gürültü azaltma klavye seslerini her durumda yok edemez.
7. **Kaydı başlat** ya da F9 kullan. F10 duraklatır/devam ettirir, F11 mikrofonu susturur. F7 pencereyi gizler/gösterir.
8. Durdurduğunda sesler dosyaya eklenir. İşlem bitmeden uygulamayı zorla kapatma. Kayıtlar sayfasından dosyayı veya klasörü açabilirsin.

## Tasarım ve pencere kontrolleri

Özel başlık çubuğundaki — küçültür, □ büyütür, × güvenli çıkış yapar. Başlık çubuğunu sürükleyerek pencereyi taşıyabilir, çift tıklayarak büyütebilirsin. Sol menüde stüdyo, kayıt ayarları ve kütüphane bölümleri bulunur. Ana ekrandaki monitör çizimi canlı önizleme değildir; önizleme çizimi kapalı tutularak yük azaltılır. Açık/koyu tema Ayarlar'dan seçilir.

## Ses kanalları

Varsayılan masaüstü + mikrofon yönlendirmesinde:

- Karma ses: yayına/dinlemeye hazır karışım.
- Masaüstü: seçili çıkış aygıtının sistem sesi.
- Mikrofon: işlenmiş konuşma.
- Etkinleştirilirse ek giriş: bağımsız ek ses kaynağı.

Masaüstü sesi yalnızca oyunu ayırmaz; süreç bazlı oyun sesi bu sürümde yok. Düzenleme programında ses kanallarını ayrı aç. İstersen kayıt sonunda WAV kanalları da dışa aktarılır. Canlı kulaklık izleme ve bas-konuş henüz yok.

## Tekrar tamponu

Kayıt sayfasından açıkça başlatılır; uygulama açıldığında kendi kendine çalışmaz. Son 15/30/60/120 saniyenin sıkıştırılmış **videosunu** RAM'de tutar. F8 kaydeder. Sesli tekrar henüz desteklenmiyor. Seçilen bellek sınırı veya anahtar kare konumu nedeniyle klip daha kısa olabilir. Normal kayıt ve tekrar tamponu birlikte çalıştırılmaz.

## Disk kullanımı ve kurtarma

Sesler kayıt sırasında ham PCM dosyalarına yazılır, durdurunca videoya eklenir. Her stereo ses kaynağı saatte yaklaşık 1,38 GB ek alan kullanır. Video ara kopyalarıyla birlikte daha fazla boş alan gerekir. `.potato-session-*` klasörleri orijinalleri saklar. Son kaydı açıp doğruladıktan sonra tamamlanmış oturumun klasörünü elle silebilirsin; uygulama kendisi silmez.

Yarım kalan oturumları Kayıtlar → Kurtar ile deneyebilirsin. Her kesinti kurtarılabilir değildir. Kaynak pencere kapanırsa ya da sürücü hata verirse kayıt durabilir; hata ve oturum dosyası gösterilir.

Alt+Tab oturumu kendiliğinden sonlandırmaz. Küçültülen veya özel tam ekran kullanan bazı oyunlar Windows'a kare vermeyi bırakabilir. Bu bir işletim sistemi/oyun sınırı olabilir. Kısa test yapmadan önemli kayda başlama.

Dil seçimi ilk açılışta Windows'a göre belirlenir. Türkçe, İngilizce, Azerbaycanca ve Rusça dahil toplam 23 dil paketi bulunur. Her dilin ayrı kullanım kılavuzu `docs/languages` klasöründedir. Çeviriler makine desteğiyle hazırlanmıştır; ana dil incelemesi tamamlanmamıştır. Ham Windows/FFmpeg tanı çıktıları özgün dilinde kalır.
