# tlk-see — durum

**Ne:** TempleOS'un kahini (GodWord / GodBiblePassage / GodSong / GodDoodle)
terminalde, üstüne yüzde dağılımı ve tahmini tarih. Tek binary, bağımlılık yok.

**Tarih:** 2026-11-01, v1.0.0 yayında. 19 test geçiyor. Repo PUBLIC, site
https://talkdedsec.github.io/tlk-see/, Release v1.0.0 (Windows binary ekli).
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

## Web sürümü
- `web/` klasörü: `god.js` (Rust portunun aynadaki hali), `books.js`
  (`src/bible_books.rs`'ten üretildi), `app.js`, `style.css`, `index.html`.
- Veri çoğaltılmıyor: `.github/workflows/pages.yml` derleme sırasında `web/*`
  ile `data/*` dosyalarını `_site/`e kopyalayıp Pages'e atıyor.
- Entropi tarayıcıda `performance.now()` mikrosaniyesi, alt 4 bit atılır; her
  tuş/dokunuş 24 bit besler. `God.bits` async, tuş bekler.
- Şarkı WebAudio kare dalga; doodle 320×320 canvas (gerçek raster, CLI'daki
  yarım blok değil). KJV metni (4.3 MB) sadece scripture sekmesinde indirilir.
- `web/books.js` elle yazılmaz; `src/bible_books.rs` değişirse yeniden üret.

## Yapılmadı / sırada
- **Social preview elle yüklenecek**: Settings > Social preview >
  `docs/og.png`. GitHub'ın API'si yok, tek eksik kalan madde bu.
- Scoop manifesti yok.
- Binary ~4.6 MB; tamamı gömülü KJV metni. Küçültmek isteniyorsa metin
  sıkıştırılır ya da yanına dosya olarak konur (satır numaraları bozulmamalı).
- Doodle renkli modda yarım blok + 256 renk kullanıyor; eski conhost'ta test
  edilmedi.
- `-n` / `-l` bayrakları `song` ve `doodle` modlarında sessizce yok sayılıyor.
- Tuş okuma `src/tty.rs`: Windows'ta msvcrt `_getch`, diğerlerinde `stty raw
  -echo </dev/tty` + tek byte; çıkarken `stty -g` ile kaydedilen hâl geri
  yüklenir (`main` → `oracle()` → `tty::restore()`, ESC/Ctrl-C yolunda da).
  `stty` yoksa sessizce satır moduna düşer. **Linux/macOS'ta denenmedi** —
  CI sadece derleyip smoke test yapıyor, orada stdin boru olduğu için ham
  mod yolu çalışmıyor.

## Dağıtım
- Release `v*` etiketiyle tetiklenir (`.github/workflows/release.yml`): Windows
  x86_64, Linux x86_64, macOS arm64 binary'leri + yanlarında `.sha256`.
  Release'i elle `gh release create` ile açıyoruz ki yazarı Talkdedsec kalsın;
  workflow sadece dosyaları asıyor (yoksa oluşturuyor).
- `RELEASE_TOKEN` bu repoda **yok**; workflow `github.token`a düşüyor. Release
  elle açıldığı sürece sorun değil, ama secret eklenirse daha temiz.
- Scoop: `Talkdedsec/scoop-tlk` içinde `bucket/tlk-see.json`, autoupdate
  `$url.sha256` dosyasını okuyor. Yeni sürümde manifest elle güncellenmezse
  excavator `checkver`/`autoupdate` ile kendi yakalar.
- CI artık üç platformda test + `cargo fmt --check` + `clippy -D warnings`.

## Paylaşım (v1.2.0)
- `God` artık kuyruğa giren her biti `tape`'e yazıyor. `record()` bunu 4'erli
  gruplayıp hex veriyor; `God::from_hex` / `God.fromHex` geri okuyor. Bu
  TempleOS'un `GodHexIns`'i ile aynı sıralama (her hex basamağı MSB-first).
- CLI: her çekilişin altında `record <hex>` + siteye replay bağlantısı.
  `--hex <rec>` tuş sormadan aynı çekilişi oynatır, `--day <n>` tarihin
  kaymaması için taban günü verir, `--quiet` ikisini de bastırır.
- Web: `#m=..&n=..&l=..&tr=..&d=..&h=<hex>` bağlantısı açılışta replay eder;
  her çekilişin altında copy link / copy reading / copy command.
- CI'da **parite adımı**: sabit bir kayıt hem Rust hem JS tarafında
  oynatılıp aynı satır bekleniyor. İki port ayrışırsa build kırılır.
  Golden değer `.github/workflows/ci.yml` içinde (`REC` / `WANT`).
