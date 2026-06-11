package com.stellasecret.smartshoppingcalculator

import android.webkit.JavascriptInterface
import android.widget.Toast

/**
 * JavaScript ↔ Kotlin bridge exposed as `window.AndroidBridge`.
 *
 * Usage from JS:
 *   AndroidBridge.showToast("Saved!");
 *   const dark = AndroidBridge.isSystemDarkMode();
 *   AndroidBridge.scanBarcode("pro", 0);   // triggers camera; result delivered via onBarcodeResult()
 */
class AndroidBridge(private val activity: MainActivity) {

    @JavascriptInterface
    fun showToast(message: String) {
        Toast.makeText(activity, message, Toast.LENGTH_SHORT).show()
    }

    @JavascriptInterface
    fun isSystemDarkMode(): Boolean {
        val nightMode = activity.resources.configuration.uiMode and
                android.content.res.Configuration.UI_MODE_NIGHT_MASK
        return nightMode == android.content.res.Configuration.UI_MODE_NIGHT_YES
    }

    @JavascriptInterface
    fun getAppVersion(): String = BuildConfig.VERSION_NAME

    /**
     * Called by JS to start a barcode scan for a specific product card.
     * @param ns   "tp" or "pro" — which page the card belongs to
     * @param id   the item's JS id (integer) so the result can be routed back
     */
    @JavascriptInterface
    fun scanBarcode(ns: String, id: Int) {
        activity.startBarcodeScan(ns, id)
    }
}
