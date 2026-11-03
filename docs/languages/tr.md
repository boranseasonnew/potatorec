# Potato Recorder — Türkçe

**Kullanım kılavuzu**

Oyun kaydı için küçük bir yerel Windows stüdyosu.

## Başlarken

FFmpeg 8.1 veya daha yenisini ve eşleşen ffprobe'u yükleyin. Bunları PATH'e veya uygulamanın yanına koyun. Uygulama araçları indirmez veya kayıtları yüklemez.

Dilinizi seçmek için Ayarlar'ı açın. Kayıt'ta bir ekran veya oyun penceresi seçin. Windows Graphics Capture, pencereleri ve ekranları destekler; DXGI ekranları destekler.

## Video ve performans

Entegre grafiklerde 60 FPS'de 1280 × 720 ile başlayın, ardından oyun kare sürelerini ölçün. Varsayılan kaynak çözünürlüğü olarak kalır. Bu bir öneridir, performans garantisi değildir.

Gerçek donanım kodlayıcılarını otomatik olarak test eder. Kalite nicemlemeyi değiştirir; hız kodlayıcı çabasını değiştirir. Tam kesirli kare hızları 120 FPS'ye kadar desteklenir. Gerçek kaynak teslimi daha düşük olabilir.

Varsayılan uyumluluk aktarımı, çerçeveleri RAM'e indirir. Sıfır kopya değildir. Fazladan çalışanlar çekişmeyi artırabilir. Profiller hiçbir zaman oyun ayarlarını değiştirmez veya çözünürlüğü veya FPS'yi sessizce düşürmez.

## Ses

Mikrofon varsayılan olarak kapalıdır. Bunu Ses'te açıkça etkinleştirin ve cihazını seçin. Masaüstü sesi yalnızca bir oyunu değil, seçilen çıkış uç noktasını yakalar. Kulaklık kullanın.

Gerektiğinde Karışık Ses ve Bağımsız Parça yönlendirmeyi etkinleştirin. Ortaya çıkan dosya gerçek ayrı parçalar içeriyor. RN Gürültü, geçit, genişletici, kompresör, sınırlayıcı ve kazanç etkinleştirilebilir ve yeniden sıralanabilir. Klavye gürültüsü kalabilir.

Netliği değerlendirmek için sekiz saniyelik mikrofon testini kullanın ve kaydedilen testi oynatın. Canlı kulaklık izleme ve sürece özel oyun sesi uygulanmaz.

## Kayıt kontrolleri

F9 kaydı başlatır veya durdurur. F10 duraklar veya devam eder. F11 mikrofonu sessize alır. F7 uygulamayı gösterir veya gizler. F8, açıkça başlatılan yalnızca video yeniden oynatma arabelleğini kaydeder. Çakışan kısayollar Kısayol Tuşları'nda değiştirilebilir.

Pencereyi taşımak için özel başlık çubuğunu sürükleyin. Büyütmek için çift tıklayın. Kayıt sırasında kapatma işlemi güvenli bir şekilde durdurur ve sonlandırır. Kaynak çizimi canlı bir önizleme değildir.

## Dosyalar ve kurtarma

MKV varsayılandır. MP4 ve parçalanmış MP4 durdurulduktan sonra sonlandırılır. Ses PCM sepetlerine yakalanır ve durduktan sonra eklenir. Uzun oturumların sonuçlanması zaman alabilir.

Etkinleştirilen her stereo ses kaynağı, video ve ara kopyalara ek olarak geçici PCM'de saatte yaklaşık 1,38 GB kullanır. Kurtarma klasörleri orijinalleri saklar. Oturum klasörünü manuel olarak kaldırmadan önce tamamlanmış kaydı kontrol edin.

Dosyaları açmak, videoyu yeniden kodlamadan MKV'yi MP4'e yeniden düzenlemek veya tamamlanmamış oturumları kurtarmak için Kayıtlar'ı kullanın. Orijinal kayıtlar otomatik olarak silinmez.

## Sınırlar ve doğrulama

Tekrar oynatma özelliği şu anda yalnızca videoyu kaydetmektedir. HDR, uyarlanabilir optimizasyon, işlem sesi yalıtımı, canlı izleme ve otomatik GPU sıfırlama kurtarma henüz tamamlanmadı. Başlangıç ​​ses/görüntü hizalaması tahmin edilir; Önemli kayıtlardan önce dudak senkronizasyonunu kontrol edin.

Korumalı içerik, bazı özel tam ekran oyunlar ve simge durumuna küçültülmüş pencereler, çerçevelerin yayınlanmasını durdurabilir. Alt+Tab oturumu açık tutar ancak işletim sistemi kısıtlamalarını kaldıramaz.

OBS veya Xbox Game Bar'a göre herhangi bir performans avantajı belirlenmemiştir. Tekrarlanabilir testler ve kalan çalışma için karşılaştırma protokolüne ve uygulama durumuna bakın.

## Çeviri notu

Bu dil paketi makine desteklidir ve henüz anadili konuşanlar için tam bir inceleme almamıştır. Teknik Windows ve FFmpeg tanılamaları orijinal ifadelerini korur.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
