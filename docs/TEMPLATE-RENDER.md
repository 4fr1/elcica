# Template belajar update: GitHub → Render

Branch: `template/render-update`
Service Render: `elcica-template`

## Kenapa template terpisah?

- Tidak menyentuh `dev` / `main`
- Tidak butuh Neon (`DATABASE_URL`) — health check pakai `/`, bukan `/health`
- Nama service beda, jadi tidak bentrok dengan `elcica-api`

## Cara bikin di Render (dari file GitHub)

1. Buka https://dashboard.render.com
2. **New → Blueprint Instance**
3. Connect GitHub → authorize → pilih repo `4fr1/elcica`
4. Render akan mendeteksi `render.yaml` dari branch `template/render-update`
5. Review service `elcica-template`, lalu **Create Blueprint Instance**
6. Tunggu build selesai (`cargo build --release`)

Render yang connect ke GitHub memakai **GitHub App milik Render** — itu contoh nyata
GitHub App yang sudah kamu pelajari, bukan App yang kamu buat sendiri.

## Cek pertama

```text
https://elcica-template.onrender.com/
```

Harus keluar:

```json
{"app":"elcica","version":"0.1.0","latihan":"versi-1"}
```

## Latihan update (lintas tiga lapis)

1. Buat branch baru dari template:
   ```powershell
   git checkout template/render-update
   git checkout -b feature/template-versi-2
   ```
2. Ubah `"latihan": "versi-1"` jadi `"versi-2"` di `src/main.rs`
3. Commit + push
4. Buat PR dengan **base = `template/render-update`**
   - `ci.yml` memantau `pull_request` tanpa filter branch → **CI tetap jalan**
5. Lihat check `rust` hijau → merge PR
6. Render otomatis deploy ulang dari branch template
7. Buka lagi URL `/` → sekarang `"latihan":"versi-2"`

## Peta pengecekan

| Lapisan | Yang dicek |
|---|---|
| GitHub Actions | `cargo fmt`, `clippy`, `test` sebelum merge |
| Render | Build + deploy otomatis setelah merge |
| Browser / curl | Hasil akhir: JSON berubah ke `versi-2` |
