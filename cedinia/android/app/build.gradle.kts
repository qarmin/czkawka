// App module – wraps the prebuilt libcedinia.so into an AAB for Google Play.
//
// Build flow:
//   1. cargo apk build (or cargo ndk) compiles libcedinia.so and embeds the
//      Java DEX via include_bytes! – no Java compilation is needed here.
//   2. The .so is copied to src/main/jniLibs/arm64-v8a/ before running Gradle.
//   3. `gradle bundleRelease` produces app/build/outputs/bundle/release/app-release.aab
//
// Signing:
//   The keystore password is read from env vars KEYSTORE_PASSWORD / KEY_PASSWORD,
//   falling back to the same test password used by cargo-apk.
//   For a real Play Store release, supply proper secrets.

plugins {
    id("com.android.application")
}

android {
    namespace = "io.github.qarmin.cedinia"
    compileSdk = 35

    defaultConfig {
        applicationId = "io.github.qarmin.cedinia"
        minSdk = 26
        targetSdk = 35
        // Increment versionCode for every Play Store upload.
        versionCode = 2
        versionName = "11.0.1"
    }

    signingConfigs {
        create("release") {
            storeFile = file("../keystore/release.keystore")
            storePassword = System.getenv("KEYSTORE_PASSWORD") ?: "123456"
            keyAlias = "release"
            keyPassword = System.getenv("KEY_PASSWORD") ?: "123456"
        }
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
            signingConfig = signingConfigs.getByName("release")
        }
    }

    // Redirect resources to the shared res/ directory in cedinia/.
    // jniLibs default (src/main/jniLibs/) is used as-is – the .so is copied
    // there before invoking Gradle.
    sourceSets {
        named("main") {
            res.setSrcDirs(setOf(file("../../res")))
        }
    }
}
