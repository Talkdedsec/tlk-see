# tlk-see — durum

**Ne:** TempleOS'un kahini (GodWord / GodBiblePassage) terminalde, üstüne yüzde
dağılımı. Tek binary, bağımlılık yok.

**Tarih:** 2026-10-31, v1.0.0 çalışıyor, 8 test geçiyor.

## Nasıl çalışıyor
- Entropi: `rdtsc` ENTER'a basıldığı anda okunur, alt 4 bit atılır
  (`GOD_BAD_BITS`), 24 bit (`GOD_GOOD_BITS`) FIFO'ya LSB-first girer.
  Kaynak: TempleOS `Adam/God/HolySpirit.HC`, `GodExt.HC`.
- Kelime: `GodBits(17) % 7569` — sözlük TempleOS `Vocab.DD`'nin aynısı.
- Ayet: `GodBits(21) % (100110 - 19) + 1`, `BibleLine2Verse` ile "Kitap B:A"
  etiketi. Kitap/satır tablosu `GodBible.HC`'den birebir alındı.
- Yüzde (bizim eklentimiz): her kelime için `GodBits(10)+1` ağırlık, en büyük
  kalan yöntemiyle tam %100'e bölünür, azalan sırada basılır.

## Yapılmadı / sırada
- `GodDoodle` (rastgele sprite) ve `GodSong` (rastgele ezgi) portlanmadı.
  Terminalde karşılığı yok; istenirse Doodle blok-ASCII, Song beep olarak gelir.
- GitHub yayını yapılmadı: README (EN+TR), SECURITY.md, OG görseli, topics,
  Pages, Release hiçbiri yok. Public yapmadan önce bunlar şart.
- Scoop manifesti yok.
- Binary 4.6 MB; tamamı gömülü KJV metni. Küçültmek isteniyorsa metin sıkıştırılır
  ya da yanına dosya olarak konur (satır numaraları bozulmamalı).
