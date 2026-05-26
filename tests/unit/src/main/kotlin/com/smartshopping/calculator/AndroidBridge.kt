package com.stellasecret.smartshoppingcalculator

import android.content.Context
import android.widget.Toast

/**
 * TEST-MODULE COPY — do not add logic here.
 *
 * Mirrors app/src/main/java/.../AndroidBridge.kt so AndroidBridgeTest can
 * compile in the JVM-only tests/unit module without depending on the app
 * module or BuildConfig.
 *
 * ⚠️  KEEP IN SYNC: when you add/change a method in the production
 * AndroidBridge, make the same change here. The pre-commit hook warns if
 * the public method signatures diverge.
 *
 * Intentional differences from production:
 *  - @JavascriptInterface omitted (android.webkit absent from JVM classpath;
 *    annotation is runtime-only and has no effect on logic).
 *  - BuildConfig.VERSION_NAME → companion object constant VERSION_NAME.
 */
class AndroidBridge(private val context: Context) {

    fun showToast(message: String) {
        Toast.makeText(context, message, Toast.LENGTH_SHORT).show()
    }

    fun isSystemDarkMode(): Boolean {
        val nightMode = context.resources.configuration.uiMode and
                android.content.res.Configuration.UI_MODE_NIGHT_MASK
        return nightMode == android.content.res.Configuration.UI_MODE_NIGHT_YES
    }

    fun getAppVersion(): String = VERSION_NAME

    companion object {
        // Keep in sync with versionName in app/build.gradle.
        const val VERSION_NAME = "1.0.0"
    }
}
