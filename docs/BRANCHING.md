# Catatan cabang (dev)

Dokumen ini hanya ada di branch `dev` sebagai contoh bahwa cabang development bisa berisi catatan yang belum siap production.

## Aturan

- `dev` = integrasi. Semua feature branch merge ke sini.
- `main` = production. Hanya menerima PR dari `dev`.
- Tag `v*` dibuat dari `main` setelah merge, lalu memicu build APK.

## Checklist sebelum PR dev → main

- [ ] CI hijau di `dev`
- [ ] Migrasi DB sudah ada di `migrations/` (jangan edit lama)
- [ ] Secret baru sudah didaftarkan di README
- [ ] `/health` masih balas 200 setelah deploy
