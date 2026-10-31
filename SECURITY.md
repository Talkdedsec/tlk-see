# Security Policy

## Reporting a vulnerability

Do not open a public issue.

- GitHub: **Security → Report a vulnerability** (private vulnerability reporting)
- E-mail: **talkdedsec@proton.me**

Include what you did, what happened, and the version (`tlk-see --version`).
Expect a first reply within a week.

## Scope

`tlk-see` reads nothing but your keystrokes and makes no network calls. The
likely issues are memory safety in the drawing code, panics on hostile
arguments, or a path where the tool is trusted as a random source.

**It is not a cryptographic random number generator.** Entropy is a CPU timer
read when you press a key, as in TempleOS. Using it for keys, passwords or
tokens is a misuse, not a vulnerability.

## Supported versions

The latest release only.

---

# Güvenlik Politikası

## Açık bildirimi

Herkese açık issue açma.

- GitHub: **Security → Report a vulnerability** (özel güvenlik bildirimi)
- E-posta: **talkdedsec@proton.me**

Ne yaptığını, ne olduğunu ve sürümü (`tlk-see --version`) yaz. İlk yanıt bir
hafta içinde gelir.

## Kapsam

`tlk-see` tuş vuruşların dışında bir şey okumaz, ağa çıkmaz. Olası konular:
çizim kodundaki bellek güvenliği, kötü argümanlarda panic, ya da aracın
rastgelelik kaynağı sanılması.

**Kriptografik rastgele sayı üreteci değildir.** Entropi, TempleOS'ta olduğu
gibi tuşa bastığında okunan bir CPU sayacıdır. Anahtar, parola veya token
üretiminde kullanmak hatalı kullanımdır, güvenlik açığı değil.

## Desteklenen sürümler

Yalnızca en son sürüm.
