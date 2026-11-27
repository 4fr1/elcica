# Android app (placeholder)

Folder ini sengaja dikosongkan dulu supaya workflow `android.yml` tidak langsung error.

Workflow akan **skip otomatis** kalau belum ada `app/android/gradlew`.

## Kalau mau mulai build APK

Pilih salah satu, taruh project Gradle di `app/android/`:

- **Flutter:** `flutter create android_app` lalu pindahkan folder `android/` ke sini
- **Native Android:** buat project di Android Studio, simpan di `app/android/`
- **Capacitor:** `npx cap add android` → `app/android/`

Setelah `app/android/gradlew` ada:

1. Isi 4 secret Android di GitHub (lihat README utama).
2. Pastikan `build.gradle` membaca `keystore.properties` untuk signing config release.
3. Push tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
4. APK otomatis muncul di GitHub Releases.

> Untuk Play Store, ganti `assembleRelease` dengan `bundleRelease` dan upload `.aab` lewat Fastlane `supply`.

⚠️ **Jangan pernah commit keystore / `keystore.properties`.** Simpan backup keystore di password manager atau R2 private — kalau hilang, aplikasi tidak bisa di-update dengan identitas yang sama.
