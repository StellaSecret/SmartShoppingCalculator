package com.stellasecret.smartshoppingcalculator

import android.content.Context
import android.content.res.Configuration
import android.content.res.Resources
import android.widget.Toast
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldMatch
import io.mockk.*
import org.junit.jupiter.api.*

/**
 * Unit tests for AndroidBridge — the JS ↔ Kotlin interface.
 *
 * Android framework classes (Context, Toast, Resources) are all mocked
 * via Mockk so these run on the JVM with no emulator.
 */
@TestMethodOrder(MethodOrderer.DisplayName::class)
class AndroidBridgeTest {

    private lateinit var mockContext: Context
    private lateinit var mockResources: Resources
    private lateinit var bridge: AndroidBridge

    @BeforeEach
    fun setUp() {
        mockContext   = mockk(relaxed = true)
        mockResources = mockk(relaxed = true)

        every { mockContext.resources } returns mockResources

        // Static Toast mock
        mockkStatic(Toast::class)
        val toastMock = mockk<Toast>(relaxed = true)
        every { Toast.makeText(any(), any<String>(), any()) } returns toastMock

        bridge = AndroidBridge(mockContext)
    }

    @AfterEach
    fun tearDown() {
        unmockkAll()
    }

    // ── showToast ────────────────────────────────────────────────────────────

    @Test
    @DisplayName("showToast — calls Toast.makeText with the correct message")
    fun `showToast calls Toast makeText with message`() {
        bridge.showToast("Hello from JS!")

        verify(exactly = 1) {
            Toast.makeText(mockContext, "Hello from JS!", Toast.LENGTH_SHORT)
        }
    }

    @Test
    @DisplayName("showToast — calls show() on the returned Toast")
    fun `showToast calls show on the toast`() {
        val toastMock = mockk<Toast>(relaxed = true)
        every { Toast.makeText(any(), any<String>(), any()) } returns toastMock

        bridge.showToast("test")

        verify(exactly = 1) { toastMock.show() }
    }

    @Test
    @DisplayName("showToast — empty string does not throw")
    fun `showToast with empty string does not throw`() {
        assertDoesNotThrow { bridge.showToast("") }
    }

    @Test
    @DisplayName("showToast — long string is passed through as-is")
    fun `showToast with long string passes full text`() {
        val longMsg = "A".repeat(500)
        bridge.showToast(longMsg)

        verify { Toast.makeText(mockContext, longMsg, Toast.LENGTH_SHORT) }
    }

    // ── isSystemDarkMode ─────────────────────────────────────────────────────

    @Test
    @DisplayName("isSystemDarkMode — returns true when UI_MODE_NIGHT_YES")
    fun `isSystemDarkMode returns true in night mode`() {
        stubNightMode(Configuration.UI_MODE_NIGHT_YES)
        bridge.isSystemDarkMode() shouldBe true
    }

    @Test
    @DisplayName("isSystemDarkMode — returns false when UI_MODE_NIGHT_NO")
    fun `isSystemDarkMode returns false in light mode`() {
        stubNightMode(Configuration.UI_MODE_NIGHT_NO)
        bridge.isSystemDarkMode() shouldBe false
    }

    @Test
    @DisplayName("isSystemDarkMode — returns false when UI_MODE_NIGHT_UNDEFINED")
    fun `isSystemDarkMode returns false when night mode is undefined`() {
        stubNightMode(Configuration.UI_MODE_NIGHT_UNDEFINED)
        bridge.isSystemDarkMode() shouldBe false
    }

    @Test
    @DisplayName("isSystemDarkMode — ignores unrelated uiMode bits (type=normal)")
    fun `isSystemDarkMode masks unrelated uiMode bits`() {
        // UI_MODE_TYPE_NORMAL (0x01) combined with NIGHT_YES (0x20)
        val combined = Configuration.UI_MODE_TYPE_NORMAL or Configuration.UI_MODE_NIGHT_YES
        stubNightMode(combined)
        bridge.isSystemDarkMode() shouldBe true
    }

    // ── getAppVersion ────────────────────────────────────────────────────────

    @Test
    @DisplayName("getAppVersion — returns a non-blank string")
    fun `getAppVersion returns non-blank string`() {
        val version = bridge.getAppVersion()
        version.isBlank() shouldBe false
    }

    @Test
    @DisplayName("getAppVersion — follows semver pattern (X.Y.Z)")
    fun `getAppVersion follows semver pattern`() {
        val version = bridge.getAppVersion()
        version shouldMatch Regex("""\d+\.\d+\.\d+.*""")
    }

    @Test
    @DisplayName("getAppVersion — is consistent across multiple calls")
    fun `getAppVersion is idempotent`() {
        val v1 = bridge.getAppVersion()
        val v2 = bridge.getAppVersion()
        v1 shouldBe v2
    }

    @Test
    @DisplayName("scanBarcode — does not throw")
    fun `scanBarcode does not throw`() {
        assertDoesNotThrow { bridge.scanBarcode("pro", 1) }
    }

    // ── Helper ───────────────────────────────────────────────────────────────

    private fun stubNightMode(uiModeFlag: Int) {
        // Configuration() throws RuntimeException("Stub!") in the android stub jar,
        // and Configuration.uiMode is a Java field that Mockk cannot stub via every{}.
        //
        // Fix: use Objenesis (bundled transitively with Mockk) to allocate a real
        // Configuration instance without invoking the stub constructor, then write
        // the uiMode field directly via reflection.
        val objenesis = org.objenesis.ObjenesisStd()
        val config = objenesis.newInstance(Configuration::class.java)
        Configuration::class.java.getField("uiMode").setInt(config, uiModeFlag)
        every { mockResources.configuration } returns config
    }
}
