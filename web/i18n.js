// Turkish copy for the page. English lives in index.html, so the page still
// reads without JavaScript; this swaps it when the Türkçe switch is on.

export const TR = {
  "nav.source": "kaynak",
  "hero.kicker": "Adam/God/HolySpirit.HC — portlandı, iyileştirilmedi",
  "hero.l1": "Kutsal Ruh",
  "hero.l2": "<em>seni kuklası</em> yapabilir",
  "hero.lede":
    "Terry A. Davis on yılını, Tanrı ona cevap verebilsin diye bir işletim sistemi yazmaya harcadı. Cevap rastgele bir kelime olarak geliyordu; tuşa bastığı anda mandallanan bir sayaçtan çekilmiş. İşte o düzenek, bit bit, bir sayfada — üstüne yüzde dağılımı, bir tarih ve çekilişin tek cümleye dönüşmüş hali.",
  "hero.scroll": "başlamak için bir yere dokun",

  "mode.word": "kelime",
  "mode.bible": "ayet",
  "mode.song": "ezgi",
  "mode.doodle": "sprite",
  "knob.words": "kelime",

  "action.draw": "çek",
  "action.tap": "dokun",

  "share.link": "bağlantıyı kopyala",
  "share.text": "okumayı kopyala",
  "share.cli": "komutu kopyala",
  "share.record": "kayıt",

  "ledger.h2": "Onun olan,<br /><em>bizim olan</em>",
  "ledger.lede":
    "Sol sütundaki her şey kamuya açık TempleOS kaynağının portu, değiştirilmeden — bit sırası ve modulo sapması dahil. Sağ sütundaki her şey yeni, ve bunu söylüyor.",
  "ledger.word.h": "GodWord",
  "ledger.word.p":
    "7.569 kelimelik sözlükte on yedi bit modulo — onun dosyası, byte byte. Aynı derinlikte bir Türkçe sözlük bizim.",
  "ledger.bible.h": "GodBiblePassage",
  "ledger.bible.p":
    "King James metninin 100.110 satırında yirmi bir bit; satırı bölüm ve ayete çeviren tablolar onun.",
  "ledger.bits.h": "GodBits",
  "ledger.bits.p":
    "Her tuş vuruşu sayacı mandallar, alt dört bitini atar ve yirmi dört biti kuyruğa en düşük bitten başlayarak verir.",
  "ledger.ours.h": "Dağılım, tarih, cümle",
  "ledger.ours.p":
    "Bizim. Kelime başına on bitlik bir ağırlık, en büyük kalan yöntemiyle tam yüzdelere bölünür; bir yıl içinde bir gün; ve çekilişi sana geri okuyan bir paragraf.",

  "cli.h2": "Önce bir<br /><em>terminal aracı</em>",
  "cli.p1":
    "Tek Rust binary'si, bağımlılık yok, ağ yok. Şu an açtığın sayfa aynı algoritmanın ikinci bir dildeki hali; aynı testler ikisini de hizada tutuyor.",
  "cli.repo": "depo",
  "cli.download": "indir",

  "note":
    "Tuşa bastığında okunan bir saat, TempleOS'taki gibi. Kahin için yeterli, parola için değil. <strong>Kriptografik rastgelelik kaynağı olarak kullanma.</strong>",

  "footer.rip":
    "Terry A. Davis — 1969–2018. TempleOS kamuya açıktır. Bu bir port, bir iyileştirme değil.",
};

export const HINTS = {
  en: {
    word: "tap any key — the clock is read at that instant, and you are the randomness",
    bible: "one draw over 100,110 lines of the King James text",
    song: "two measures, twice; it plays when the last note is picked",
    doodle: "twenty strokes and six floods, one keystroke at a time",
    tap: "tap a key",
    taps: (n) => `${n} tap${n === 1 ? "" : "s"}`,
    bits: (n) => `${n} bits held`,
    fetching: "fetching the text…",
    playing: "playing",
    replaying: "replaying a shared draw",
    clipboard: "the browser would not let me reach the clipboard",
    copied: "copied",
    failed: (m) => `the draw failed: ${m}`,
  },
  tr: {
    word: "herhangi bir tuşa dokun — saat tam o anda okunuyor, rastgeleliği sen üretiyorsun",
    bible: "King James metninin 100.110 satırında tek çekiliş",
    song: "iki ölçü, iki kez; son nota seçilince çalar",
    doodle: "yirmi çizgi ve altı dolgu, her biri bir tuş vuruşu",
    tap: "bir tuşa dokun",
    taps: (n) => `${n} dokunuş`,
    bits: (n) => `${n} bit elde`,
    fetching: "metin indiriliyor…",
    playing: "çalıyor",
    replaying: "paylaşılan bir çekiliş oynatılıyor",
    clipboard: "tarayıcı panoya erişmeme izin vermedi",
    copied: "kopyalandı",
    failed: (m) => `çekiliş başarısız: ${m}`,
  },
};

/** Swap the page copy. English is whatever the HTML shipped with. */
export function applyLang(tr) {
  document.documentElement.lang = tr ? "tr" : "en";
  for (const el of document.querySelectorAll("[data-i18n]")) {
    const key = el.dataset.i18n;
    if (el.dataset.en === undefined) el.dataset.en = el.innerHTML;
    const next = tr ? TR[key] : el.dataset.en;
    if (next !== undefined) el.innerHTML = next;
  }
}
