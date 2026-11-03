# Potato Recorder — Azərbaycanca

**İstifadəçi təlimatı**

Oyunun yazılması üçün kiçik doğma Windows studiyası.

## Başlanır

FFmpeg 8.1 və ya daha yenisini və uyğun ffprobe quraşdırın. Onları PATH-ə və ya proqramın yanına qoyun. Proqram alətləri endirmir və ya qeydləri yükləmir.

Dilinizi seçmək üçün Parametrləri açın. Qeydiyyatda ekran və ya oyun pəncərəsini seçin. Windows Graphics Capture pəncərələri və displeyləri dəstəkləyir; DXGI displeyləri dəstəkləyir.

## Video və performans

İnteqrasiya edilmiş qrafikada 60 FPS-də 1280 × 720 ilə başlayın, sonra oyun çərçivə vaxtlarını ölçün. Defolt mənbə qətnaməsi olaraq qalır. Bu performans zəmanəti deyil, tövsiyədir.

Real hardware kodlayıcılarını avtomatik sınaqdan keçirir. Keyfiyyət dəyişikliklərinin kvantlaşdırılması; sürət kodlayıcının səyini dəyişir. Dəqiq fraksiya kadr sürətləri 120 FPS-ə qədər dəstəklənir. Faktiki mənbə çatdırılması daha aşağı ola bilər.

Standart uyğunluq ötürülməsi çərçivələri RAM-a endirir. Sıfır nüsxə deyil. Əlavə işçilər mübahisəni artıra bilər. Profillər heç vaxt oyun parametrlərini dəyişdirmir və ya səssizcə aşağı qətnamə və ya FPS-ni dəyişdirmir.

## Audio

Mikrofon defolt olaraq söndürülür. Onu Audioda açıq şəkildə aktivləşdirin və onun cihazını seçin. Desktop səsi yalnız bir oyunu deyil, seçilmiş çıxış son nöqtəsini tutur. Qulaqlıqdan istifadə edin.

Lazım olduqda Qarışıq Audio və Müstəqil Track marşrutunu aktivləşdirin. Nəticədə fayl real ayrı-ayrı trekləri ehtiva edir. RNNoise, qapı, genişləndirici, kompressor, məhdudlaşdırıcı və qazanc aktivləşdirilə və yenidən sıralana bilər. Klaviatura səsi qala bilər.

Səkkiz saniyəlik mikrofon testindən istifadə edin və aydınlığı qiymətləndirmək üçün yadda saxlanılan testi oynayın. Canlı qulaqlıq monitorinqi və prosesə aid oyun səsi həyata keçirilmir.

## Qeydə nəzarət

F9 yazmağa başlayır və ya dayandırır. F10 fasilə verir və ya davam etdirir. F11 mikrofonu səssizləşdirir. F7 proqramı göstərir və ya gizlədir. F8 açıq şəkildə başlayan yalnız video təkrar oxutma buferini saxlayır. Ziddiyyətli qısa yollar İsti düymələrdə dəyişdirilə bilər.

Pəncərəni hərəkət etdirmək üçün fərdi başlıq çubuğunu dartın. Maksimum artırmaq üçün üzərinə iki dəfə klikləyin. Səsyazma zamanı bağlanma dayanır və onu təhlükəsiz şəkildə tamamlayır. Mənbə illüstrasiya canlı önizləmə deyil.

## Fayllar və bərpa

MKV standartdır. MP4 və parçalanmış MP4 dayandırıldıqdan sonra yekunlaşdırılır. Audio PCM yan vaqonlarına çəkilir və dayandıqdan sonra əlavə edilir. Uzun sessiyalar başa çatdırmaq üçün vaxt apara bilər.

Hər bir aktivləşdirilmiş stereo audio mənbəyi, video və aralıq nüsxələrə əlavə olaraq, müvəqqəti PCM-də saatda təxminən 1,38 GB istifadə edir. Bərpa qovluqları orijinalları saxlayır. Sessiya qovluğunu əl ilə silməzdən əvvəl tamamlanmış qeydi yoxlayın.

Faylları açmaq, videonu yenidən kodlaşdırmadan MKV-ni MP4-ə remux etmək və ya bitməmiş seansları bərpa etmək üçün Yazılardan istifadə edin. Orijinal qeydlər avtomatik silinmir.

## Limitlər və yoxlama

Təkrar oxutma hazırda yalnız videonu saxlayır. HDR, adaptiv optimallaşdırma, prosesin audio izolyasiyası, canlı monitorinq və avtomatik GPU sıfırlama bərpası tamamlanmayıb. Başlanğıcda audio/videonun uyğunlaşdırılması təxmin edilir; vacib qeydlərdən əvvəl dodaq sinxronizasiyasını yoxlayın.

Qorunan məzmun, bəzi eksklüziv tam ekran oyunları və kiçildilmiş pəncərələr çərçivələrin çatdırılmasını dayandıra bilər. Alt+Tab seansı açıq saxlayır, lakin əməliyyat sistemi məhdudiyyətlərini aradan qaldıra bilmir.

OBS və ya Xbox Game Bar üzərində heç bir performans üstünlüyü müəyyən edilməmişdir. Təkrarlana bilən testlər və qalan işlər üçün etalon protokola və icra vəziyyətinə baxın.

## Tərcümə qeydi

Bu dil paketi maşın dəstəyi ilə idi və hələ tam anadilli rəy almamışdır. Texniki Windows və FFmpeg diaqnostikası orijinal sözlərini saxlayır.

[Architecture](../ARCHITECTURE.md) · [Status](../STATUS.md) · [Benchmarks](../BENCHMARKS.md)
