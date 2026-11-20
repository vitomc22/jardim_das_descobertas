plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.plugin.compose")
}

android {
    namespace = "org.jardim.descobertas.kt"
    compileSdk = 36
    defaultConfig {
        applicationId = "org.jardim.descobertas.kt"
        minSdk = 26
        targetSdk = 35
        versionCode = 3
        versionName = "0.3.0-kt"
    }
    signingConfigs {
        create("jardim") {
            storeFile = file(System.getenv("KEYSTORE") ?: "${System.getProperty("user.home")}/.android/jardim-rs.keystore")
            storePassword = "android"; keyAlias = "jardim"; keyPassword = "android"
        }
    }
    buildTypes {
        release {
            // R8: remove código não usado e otimiza o bytecode do Compose.
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            signingConfig = signingConfigs.getByName("jardim")
        }
    }
    buildFeatures { compose = true }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

dependencies {
    val bom = platform("androidx.compose:compose-bom:2025.06.00")
    implementation(bom)
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.foundation:foundation")
    implementation("androidx.activity:activity-compose:1.10.1")
    // Instala os perfis AOT que vêm nas bibliotecas do Compose (sem isso o
    // Compose roda interpretado/JIT nas primeiras execuções).
    implementation("androidx.profileinstaller:profileinstaller:1.4.1")
    // Android não lê SVG em texto: AndroidSVG rasteriza uma vez para Bitmap.
    implementation("com.caverock:androidsvg-aar:1.4")
    testImplementation("junit:junit:4.13.2")
}
