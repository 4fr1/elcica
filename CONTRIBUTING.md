# Kontribusi

1. Branch dari `dev`, bukan dari `main`:
   ```bash
   git checkout dev
   git pull
   git checkout -b feature/nama-fitur
   ```
2. Jangan commit `.env`, keystore, atau kredensial lain.
3. Pastikan CI hijau: `cargo fmt --all`, `cargo clippy --all-targets`, `cargo test`.
4. Buat PR ke `dev`. Setelah lolos, baru PR `dev → main` untuk production.
5. Perubahan skema DB: tambah file baru di `migrations/` (jangan edit migrasi lama).
6. Commit message singkat, mis. `feat: tambah endpoint upload`, `fix: health 503 saat DB mati`.
