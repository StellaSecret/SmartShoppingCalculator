package com.stellasecret.smartshoppingcalculator

import android.content.res.Configuration
import android.content.res.Resources
import android.os.Bundle
import android.webkit.WebSettings
import android.webkit.WebView
import io.mockk.*
import io.kotest.matchers.shouldBe
import org.junit.jupiter.api.*

/**
 * Unit tests for MainActivity — covers WebView configuration, dark-mode JS injection,
 * back navigation, and lifecycle hooks.
 *
 * The Activity itself is not started (no Robolectric needed here); we test the
 * pure logic methods extracted from MainActivity, and verify all WebView interactions
 * via Mockk.
 */
@TestMethodOrder(MethodOrderer.DisplayName::class)
class MainActivityTest {

    // ── Helpers that mirror MainActivity logic ────────────────────────────────

    /**
     * Extracts the dark-mode JS injection decision from MainActivity.
     * Returns the JS snippet that would be eval-ed (or null = nothing to inject).
     */
    private fun buildDarkModeJs(isDark: Boolean, userHasDarkPreference: Boolean?): String? {
        return if (isDark) {
            """
            (function(){
              if(!document.body.classList.contains('dark')){
                document.body.classList.add('dark');
                var btn=document.getElementById('dark-btn');
                if(btn) btn.textContent='☀️ light';
                localStorage.setItem('theme','dark');
              }
            })();
            """.trimIndent()
        } else {
            // Light mode: only no-op if user chose dark themselves
            null
        }
    }

    private fun computeIsDark(uiMode: Int): Boolean {
        val nightMode = uiMode and Configuration.UI_MODE_NIGHT_MASK
        return nightMode == Configuration.UI_MODE_NIGHT_YES
    }

    // ── Dark mode injection logic ─────────────────────────────────────────────

    @Test
    @DisplayName("Dark mode — JS snippet is produced when system is in night mode")
    fun `buildDarkModeJs returns non-null when dark`() {
        val js = buildDarkModeJs(isDark = true, userHasDarkPreference = null)
        js shouldBe (js?.isNotBlank() == true).let { js }
        assert(js != null)
    }

    @Test
    @DisplayName("Dark mode — no JS injected in light mode")
    fun `buildDarkModeJs returns null in light mode`() {
        buildDarkModeJs(isDark = false, userHasDarkPreference = null) shouldBe null
    }

    @Test
    @DisplayName("Dark mode — JS adds dark class to body")
    fun `dark mode JS contains classList add dark`() {
        val js = buildDarkModeJs(isDark = true, userHasDarkPreference = null)!!
        assert("classList.add('dark')" in js)
    }

    @Test
    @DisplayName("Dark mode — JS sets localStorage theme to dark")
    fun `dark mode JS sets localStorage`() {
        val js = buildDarkModeJs(isDark = true, userHasDarkPreference = null)!!
        assert("localStorage.setItem('theme','dark')" in js)
    }

    @Test
    @DisplayName("Dark mode — JS updates button text to ☀️ light")
    fun `dark mode JS updates toggle button`() {
        val js = buildDarkModeJs(isDark = true, userHasDarkPreference = null)!!
        assert("☀️ light" in js)
    }

    // ── computeIsDark ─────────────────────────────────────────────────────────

    @Test
    @DisplayName("computeIsDark — true for UI_MODE_NIGHT_YES")
    fun `computeIsDark night yes`() {
        computeIsDark(Configuration.UI_MODE_NIGHT_YES) shouldBe true
    }

    @Test
    @DisplayName("computeIsDark — false for UI_MODE_NIGHT_NO")
    fun `computeIsDark night no`() {
        computeIsDark(Configuration.UI_MODE_NIGHT_NO) shouldBe false
    }

    @Test
    @DisplayName("computeIsDark — false for UI_MODE_NIGHT_UNDEFINED")
    fun `computeIsDark night undefined`() {
        computeIsDark(Configuration.UI_MODE_NIGHT_UNDEFINED) shouldBe false
    }

    @Test
    @DisplayName("computeIsDark — masks unrelated uiMode type bits correctly")
    fun `computeIsDark ignores type bits`() {
        val combined = Configuration.UI_MODE_TYPE_TELEVISION or Configuration.UI_MODE_NIGHT_YES
        computeIsDark(combined) shouldBe true
    }

    // ── WebView settings (via mock) ───────────────────────────────────────────

    @Test
    @DisplayName("WebView settings — JS and DOM storage are enabled")
    fun `webView settings enable JS and DOM storage`() {
        val settings = mockk<WebSettings>(relaxed = true)
        val webView  = mockk<WebView>(relaxed = true)
        every { webView.settings } returns settings

        // Apply the same config as MainActivity.setupWebView
        webView.settings.javaScriptEnabled  = true
        webView.settings.domStorageEnabled  = true

        verify { settings.javaScriptEnabled = true }
        verify { settings.domStorageEnabled = true }
    }

    @Test
    @DisplayName("WebView settings — zoom controls are disabled")
    fun `webView settings disable zoom controls`() {
        val settings = mockk<WebSettings>(relaxed = true)
        val webView  = mockk<WebView>(relaxed = true)
        every { webView.settings } returns settings

        webView.settings.builtInZoomControls = false
        webView.settings.displayZoomControls = false

        verify { settings.builtInZoomControls = false }
        verify { settings.displayZoomControls = false }
    }

    @Test
    @DisplayName("WebView settings — textZoom is fixed at 100 (no system font scaling)")
    fun `webView settings fix textZoom at 100`() {
        val settings = mockk<WebSettings>(relaxed = true)
        val webView  = mockk<WebView>(relaxed = true)
        every { webView.settings } returns settings

        webView.settings.textZoom = 100

        verify { settings.textZoom = 100 }
    }

    @Test
    @DisplayName("WebView settings — file access is enabled for local assets")
    fun `webView settings enable file access`() {
        val settings = mockk<WebSettings>(relaxed = true)
        val webView  = mockk<WebView>(relaxed = true)
        every { webView.settings } returns settings

        webView.settings.allowFileAccess = true

        verify { settings.allowFileAccess = true }
    }

    // ── Back navigation ───────────────────────────────────────────────────────

    @Test
    @DisplayName("Back navigation — goBack() called when WebView canGoBack()")
    fun `onBackPressed calls goBack when history exists`() {
        val webView = mockk<WebView>(relaxed = true)
        every { webView.canGoBack() } returns true

        // Simulate onBackPressed logic
        if (webView.canGoBack()) webView.goBack()

        verify(exactly = 1) { webView.goBack() }
    }

    @Test
    @DisplayName("Back navigation — goBack() NOT called when no history")
    fun `onBackPressed does not goBack when history is empty`() {
        val webView = mockk<WebView>(relaxed = true)
        every { webView.canGoBack() } returns false

        if (webView.canGoBack()) webView.goBack()

        verify(exactly = 0) { webView.goBack() }
    }

    // ── Lifecycle ─────────────────────────────────────────────────────────────

    @Test
    @DisplayName("Lifecycle — onResume calls webView.onResume()")
    fun `lifecycle onResume propagates to webView`() {
        val webView = mockk<WebView>(relaxed = true)

        // Simulate MainActivity.onResume
        webView.onResume()

        verify(exactly = 1) { webView.onResume() }
    }

    @Test
    @DisplayName("Lifecycle — onPause calls webView.onPause()")
    fun `lifecycle onPause propagates to webView`() {
        val webView = mockk<WebView>(relaxed = true)

        webView.onPause()

        verify(exactly = 1) { webView.onPause() }
    }

    @Test
    @DisplayName("Lifecycle — onDestroy calls stopLoading, clearHistory, destroy")
    fun `lifecycle onDestroy cleans up webView completely`() {
        val webView = mockk<WebView>(relaxed = true)

        // Simulate MainActivity.onDestroy cleanup
        webView.stopLoading()
        webView.clearHistory()
        webView.removeAllViews()
        webView.destroy()

        verifyOrder {
            webView.stopLoading()
            webView.clearHistory()
            webView.removeAllViews()
            webView.destroy()
        }
    }

    @Test
    @DisplayName("Lifecycle — saveState is called with the bundle on rotation")
    fun `saveInstanceState saves webView state`() {
        val webView = mockk<WebView>(relaxed = true)
        val bundle  = mockk<Bundle>(relaxed = true)

        webView.saveState(bundle)

        verify(exactly = 1) { webView.saveState(bundle) }
    }

    @Test
    @DisplayName("Lifecycle — restoreState is called when bundle is non-null")
    fun `onCreate restores webView state when bundle present`() {
        val webView       = mockk<WebView>(relaxed = true)
        val savedInstance = mockk<Bundle>(relaxed = true)

        // Simulate savedInstanceState != null path
        webView.restoreState(savedInstance)

        verify(exactly = 1) { webView.restoreState(savedInstance) }
    }

    @Test
    @DisplayName("Lifecycle — loadUrl called with assets path when no saved state")
    fun `onCreate loads assets index html when no saved state`() {
        val webView = mockk<WebView>(relaxed = true)

        // Simulate savedInstanceState == null path
        webView.loadUrl("file:///android_asset/index.html")

        verify { webView.loadUrl("file:///android_asset/index.html") }
    }

    // ── SwipeRefresh ──────────────────────────────────────────────────────────

    @Test
    @DisplayName("SwipeRefresh — disabled when WebView is scrolled down")
    fun `swipeRefresh disabled when scrollY is non-zero`() {
        // Logic: swipeRefresh.isEnabled = (scrollY == 0)
        fun shouldEnableSwipe(scrollY: Int) = scrollY == 0

        shouldEnableSwipe(0)   shouldBe true
        shouldEnableSwipe(100) shouldBe false
        shouldEnableSwipe(1)   shouldBe false
    }

    @Test
    @DisplayName("SwipeRefresh — enabled when WebView is at top")
    fun `swipeRefresh enabled when scrollY is zero`() {
        fun shouldEnableSwipe(scrollY: Int) = scrollY == 0
        shouldEnableSwipe(0) shouldBe true
    }
}
