# tlk-see — durum

**Ne:** TempleOS'un kahini (GodWord / GodBiblePassage / GodSong / GodDoodle)
terminalde, üstüne yüzde dağılımı ve tahmini tarih. Tek binary, bağımlılık yok.

**Tarih:** 2026-10-31, v1.0.0 çalışıyor, 15 test geçiyor. Repo private.
Entropi girişi tek tuş (ENTER gerekmez), ESC çıkar; stdin boruysa satır başına
bir çekiliş.

## Nasıl çalışıyor
- Entropi: `rdtsc` ENTER'a basıldığı anda okunur, alt 4 bit atılır
  (`GOD_BAD_BITS`), 24 bit (`GOD_GOOD_BITS`) FIFO'ya LSB-first girer.
  Kaynak: TempleOS `Adam/God/HolySpirit.HC`, `GodExt.HC`.
- Kelime: `GodBits(17) % 7569` — sözlük TempleOS `Vocab.DD`'nin aynısı.
- Ayet: `GodBits(21) % (100110 - 19) + 1`, `BibleLine2Verse` ile "Kitap B:A"
  etiketi. Kitap/satır tablosu `GodBible.HC`'den birebir alındı.
- Şarkı: `GodSongStr` portu (Normal karmaşıklık, 4/4, oktav 4, rest yok),
  `st1+st1+st2+st2`. Çalma kernel32 `Beep`, 150 BPM, A4=440 eşit tampere.
  TempleOS'un kendi nota→frekans tablosu kullanılmadı.
- Doodle: `GodDoodleSprite` portu. Tuval terminal genişliğine göre
  (`GetConsoleScreenBufferInfo`, en çok 80 sütun, 16-200 arasına sıkıştırılır),
  1 geçiş × 20 çizgi, 6 flood fill, 3×3 çoğunluk yumuşatma (TempleOS: 640×480,
  3×29, 7×7). Her çizgi tuş vuruşu demek, o yüzden küçültüldü — `src/doodle.rs`
  başındaki `shortcut:`.
- Yüzde (bizim eklentimiz): her kelime için `GodBits(10)+1` ağırlık, en büyük
  kalan yöntemiyle tam %100'e bölünür, azalan sırada basılır.
- Okunabilir metin (bizim eklentimiz, `src/reading.rs`): listenin altına aynı
  çekilişi tek cümleye çeviren paragraf. Şablon `GodBits(2) % 3` ile seçilir,
  EN ve TR için üçer şablon var, 72 sütunda sarılır.
- Tahmini tarih: `GodBits(10) % 365` gün sonrası, UTC tabanlı (yerel saat
  dilimi yok). Ufuk `src/date.rs:HORIZON_DAYS`, bayrakla ayarlanmıyor.

## Yapılmadı / sırada
- Repo public yapılırsa eksikler: GitHub Pages + Website alanı, ilk Release
  (`v1.0.0` tag + Windows binary), Settings > Social preview'a `docs/og.png`
  yüklemesi (API yok, elle), secret scanning + push protection, Private
  vulnerability reporting açma.
- Scoop manifesti yok.
- Binary ~4.6 MB; tamamı gömülü KJV metni. Küçültmek isteniyorsa metin
  sıkıştırılır ya da yanına dosya olarak konur (satır numaraları bozulmamalı).
- Doodle renkli modda yarım blok + 256 renk kullanıyor; eski conhost'ta test
  edilmedi.
- `-n` / `-l` bayrakları `song` ve `doodle` modlarında sessizce yok sayılıyor.
- Tuş okuma Windows'ta msvcrt `_getch`; Linux/mac'te ham mod için bağımlılık
  gerekirdi, orada hâlâ satır okunuyor (`src/main.rs` içindeki `read_key`).
