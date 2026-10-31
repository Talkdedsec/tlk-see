# tlk-see

Rust CLI. Proje durumu ve yapılacaklar: `DEVAM.md`.

## Komutlar
- `cargo test` — birim testler (entropi, yüzde dağılımı, ayet eşleme, sözlük boyu)
- `cargo build --release` — `target/release/tlk-see.exe`
- `printf '\n\n\n\n\n' | ./target/release/tlk-see.exe` — etkileşimsiz deneme

## Kırmızı çizgiler
- `src/god.rs` ve `src/bible.rs` TempleOS kaynağının portu. Algoritmayı
  "iyileştirmek" yok: bit sırası, 17/21/24 bit genişlikleri, modulo sapması
  dahil aynen kalır. Değişirse `data/` ile birlikte test de güncellenmeli.
- `data/vocab-en.txt` TempleOS `Vocab.DD` ile byte-byte aynı, `data/bible-kjv.txt`
  TempleOS `Misc/Bible.TXT` ile aynı. Satır numaraları `src/bible_books.rs`
  tablosuna bağlı — dosyaları düzenleme, Gutenberg başlığını silme.
- Yüzde katmanı bizim eklentimiz; tek kaynağı yine God entropisi olmalı
  (`rand` gibi bir bağımlılık eklenmez).
