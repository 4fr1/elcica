# elcica

Scaffold repo baru untuk stack:

```
GitHub ──push dev──> CI (fmt/clippy/test)
      ──PR ke main─> CI + migrasi Neon
      ──main───────> Render (Rust API)
      ──tag v*─────> GitHub Release + APK

Cloudflare: domain + DNS + Email Routing + R2 + Worker Cron (keep-alive)
Neon: Postgres (scale-to-zero)
Resend: email transaksional
Render: web service Rust (Free)
```

## Branch

| Branch | Fungsi | Render deploy? |
|---|---|---|
| `dev` | Sehari-hari, eksperimen, PR masuk ke sini | ❌ |
| `main` | Production, hanya lewat PR dari `dev` | ✅ |

Alur: `feature/xxx` → PR ke `dev` → test lulus → merge → PR `dev` → `main` → deploy + migrasi.

> `main` diproteksi: tidak bisa direct push, harus PR + CI hijau.

## Struktur

```
.github/workflows/
  ci.yml        # fmt, clippy, test — jalan di PR & push
  deploy.yml    # trigger Render deploy hook (manual; alternatif auto-deploy)
  migrate.yml   # sqlx migrate ke Neon
  android.yml   # build APK saat tag v*, upload ke Release
  backup.yml    # pg_dump Neon tiap 00:00 WIB + artifact 7 hari
.github/dependabot.yml
src/main.rs             # Axum: / dan /health
src/bin/migrate.rs      # runner migrasi
migrations/             # SQL versi
render.yaml             # blueprint Render
docs/index.html         # landing page GitHub Pages
app/README.md           # placeholder project Android
```

## Jalankan lokal

```bash
cp .env.example .env
# isi DATABASE_URL kalau mau nyambung ke Neon
cargo run
curl http://localhost:3000/health
```

Tanpa `DATABASE_URL`, server tetap jalan tapi `/health` balas `503 degraded`.

Migrasi manual:
```bash
$env:DATABASE_URL = "postgres://..."
cargo run --bin migrate
```

## GitHub Secrets yang harus diisi

| Secret | Dipakai oleh | Catatan |
|---|---|---|
| `NEON_DATABASE_URL` | `migrate.yml`, `backup.yml` | Connection string Neon (sslrequire) |
| `RENDER_DEPLOY_HOOK` | `deploy.yml` | Ambil di Render → Settings → Deploy Hook |
| `ANDROID_KEYSTORE_B64` | `android.yml` | `base64 release.jks` |
| `ANDROID_STORE_PASS` | `android.yml` | Password keystore |
| `ANDROID_KEY_PASS` | `android.yml` | Password key |
| `ANDROID_KEY_ALIAS` | `android.yml` | Alias key |

Tambahkan juga Environment **`production`** di Settings → Environments, lalu pasang *required reviewers* kalau mau approval manual sebelum deploy/migrasi.

## Deploy ke Render

Dua opsi, pilih **satu**:

1. **Auto-deploy (disarankan, paling simpel):** di Render, Connect Repository → branch `main`. `deploy.yml` boleh dibiarkan manual sebagai tombol cadangan.
2. **Manual:** matikan auto-deploy di Render, isi `RENDER_DEPLOY_HOOK`, lalu jalankan `deploy.yml` via *Run workflow*.

`render.yaml` juga bisa dipakai lewat Render Blueprint kalau mau dibuat dari dashboard.

## Keep-alive (jaga cold start)

Pakai Cloudflare Worker + Cron yang sudah dibahas, ping `/health` tiap 10 menit. Endpoint ini sengaja melakukan `SELECT 1` supaya Neon tidak suspend. Jangan pakai `/robots.txt` — Render tidak membangunkan service untuk path itu.

## Biaya gratis (cek 28 Nov 2026)

- Actions Free: **$12 compute/bln** ≈ 2.000 menit Linux 2-core. Runner repo public gratis.
- Artifact: **500 MB**, bareng GitHub Packages. Cache: 10 GB/repo.
- macOS runner **$0,062/menit** — hindari kecuali build iOS.
- Render Free: 750 jam/bln, 5 GB bandwidth, 500 menit build, spin-down setelah 15 menit idle.
- Neon Free: 1 GB DB, 100 CU-jam, 5 GB object storage, scale-to-zero 5 menit.

## Buat remote GitHub (karena `gh` belum terinstall)

```bash
# Remote: https://github.com/4fr1/elcica (sudah saya daftarkan sebagai origin)
git remote add origin https://github.com/4fr1/elcica.git
git push -u origin main
git push -u origin dev
```

Lalu di GitHub: Settings → Branches → Add rule untuk `main` (require PR + require status check `CI`).
