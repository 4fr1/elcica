# Belajar trigger GitHub Actions

Sembilan workflow kecil di folder ini. **Semua cuma `echo`**, tidak mengubah apa pun.
Tujuannya: melihat sendiri perbedaan tiap trigger.

| # | File | Metode | Cara memicu | Catatan penting |
|---|---|---|---|---|
| 1 | `belajar-push.yml` | `push` | Push commit ke `dev` atau `feature/belajar-trigger` | `paths-ignore: docs/**` → perubahan di `docs/` diabaikan |
| 2 | `belajar-pull-request.yml` | `pull_request` | Buat PR ke `dev` | `branches` = branch **tujuan**, bukan sumber. Jalan lagi kalau ada push baru ke PR |
| 3 | `belajar-workflow-dispatch.yml` | `workflow_dispatch` | Tab Actions → **Run workflow** → isi input | Manual. Tidak pernah jalan sendiri |
| 4 | `belajar-schedule.yml` | `schedule` | Tunggu kelipatan 15 menit UTC | Hanya jalan di branch **default** (`dev`). Cron pakai UTC, bisa delay |
| 5 | `belajar-tag.yml` | `push tags` | `git tag demo-1 && git push origin demo-1` | Tag ≠ branch. Filter `branches` tidak berlaku |
| 6 | `belajar-workflow-run.yml` | `workflow_run` | Tunggu workflow **CI** selesai | Bisa dipakai sebagai gerbang "deploy setelah CI hijau" |
| 7 | `belajar-repository-dispatch.yml` | `repository_dispatch` | `gh api ... -f event_type=cek-harga` | Dipicu dari luar lewat API, tidak ada tombol UI |
| 8 | `belajar-release.yml` | `release` | Publish release di GitHub | Beda dengan tag: ini pas *release*-nya dipublish |
| 9 | `belajar-issue-comment.yml` | `issue_comment` | Komentar `/halo` di issue/PR | Contoh command, mirip `@dependabot rebase` |

## Urutan belajar yang disarankan

1. **Manual dulu** → `belajar-workflow-dispatch.yml`. Paling mudah: klik tombol, lihat input muncul di log.
2. **Push vs PR** → push commit ke `feature/belajar-trigger`, lalu buat PR ke `dev`. Bandingkan run #1 dan #2.
3. **Workflow run** → setelah CI selesai, lihat `belajar-workflow-run.yml` ikut jalan.
4. **Tag vs Release** → push tag `demo-1`, lalu publish release dari tag itu. Dua workflow berbeda akan jalan.
5. **Schedule** → baru aktif setelah di-merge ke `dev` (branch default).
6. **Repository dispatch** → paling lanjut, butuh token/API.

## Kapan file ini aktif?

| File | Aktif setelah |
|---|---|
| `belajar-push.yml` | Push ke branch yang tercantum (termasuk branch kerja ini) |
| `belajar-pull-request.yml` | Ada PR ke `dev` |
| `belajar-workflow-dispatch.yml` | Kapan saja tombol ditekan (di branch mana pun yang punya tab Actions) |
| `belajar-schedule.yml` | **Setelah merge ke `dev`**, karena schedule hanya jalan di branch default |
| `belajar-tag.yml` | Tag `demo-*` di-push |
| `belajar-workflow-run.yml` | Setiap kali workflow `CI` selesai |
| `belajar-repository-dispatch.yml` | Ada API call dengan `event_type: cek-harga` |
| `belajar-release.yml` | Release dipublish |
| `belajar-issue-comment.yml` | Ada komentar di issue/PR |

## Cara baca hasil

Tab **Actions** → klik run → klik job `info` → baca baris `Trigger : ...`.

Kalau sebuah workflow **tidak muncul**, itu bukan error — berarti **trigger-nya belum terpenuhi**. Itu justru pelajaran utamanya. 🔍
