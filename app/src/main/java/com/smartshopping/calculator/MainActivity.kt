package com.stellasecret.smartshoppingcalculator

import android.Manifest
import android.annotation.SuppressLint
import android.content.pm.PackageManager
import android.content.res.Configuration
import android.graphics.BitmapFactory
import android.net.Uri
import android.os.Bundle
import android.os.Environment
import android.util.Log
import android.view.View
import android.webkit.*
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import androidx.core.content.FileProvider
import com.google.mlkit.vision.barcode.BarcodeScanning
import com.google.mlkit.vision.common.InputImage
import com.stellasecret.smartshoppingcalculator.databinding.ActivityMainBinding
import okhttp3.*
import org.json.JSONObject
import java.io.File
import java.io.IOException

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    // ── Barcode scan state ────────────────────────────────────────────────────
    private var photoUri: Uri? = null
    private var pendingScanNs: String = ""
    private var pendingScanId: Int = -1

    // Launchers must be registered before onStart
    private val cameraPermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestPermission()
    ) { granted ->
        if (granted) launchCamera()
        else deliverScanError("Camera permission denied")
    }

    private val takePictureLauncher = registerForActivityResult(
        ActivityResultContracts.TakePicture()
    ) { success ->
        if (success) {
            photoUri?.let { processBarcodeImage(it) }
                ?: deliverScanError("No photo captured")
        } else {
            deliverScanError("Camera cancelled")
        }
    }

    // ── Lifecycle ─────────────────────────────────────────────────────────────
    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setupWebView()
        setupSwipeRefresh()

        if (savedInstanceState != null) {
            binding.webView.restoreState(savedInstanceState)
        } else {
            binding.webView.loadUrl("file:///android_asset/calculator.html")
        }
    }

    @SuppressLint("SetJavaScriptEnabled")
    private fun setupWebView() {
        val wv = binding.webView
        wv.settings.apply {
            javaScriptEnabled        = true
            domStorageEnabled        = true
            allowFileAccess          = true
            allowContentAccess       = true
            useWideViewPort          = true
            loadWithOverviewMode     = true
            setSupportZoom(false)
            builtInZoomControls      = false
            displayZoomControls      = false
            textZoom                 = 100
        }

        wv.addJavascriptInterface(AndroidBridge(this), "AndroidBridge")

        wv.webViewClient = object : WebViewClient() {
            override fun onPageFinished(view: WebView, url: String) {
                super.onPageFinished(view, url)
                binding.progressBar.visibility = View.GONE
                binding.swipeRefresh.isRefreshing = false
                injectDarkModeIfNeeded()
            }

            override fun onReceivedError(
                view: WebView,
                request: WebResourceRequest,
                error: WebResourceError
            ) {
                super.onReceivedError(view, request, error)
                binding.progressBar.visibility = View.GONE
            }
        }

        wv.webChromeClient = object : WebChromeClient() {
            override fun onProgressChanged(view: WebView, newProgress: Int) {
                if (newProgress < 100) {
                    binding.progressBar.visibility = View.VISIBLE
                    binding.progressBar.progress   = newProgress
                } else {
                    binding.progressBar.visibility = View.GONE
                }
            }
        }
    }

    private fun setupSwipeRefresh() {
        binding.swipeRefresh.setOnRefreshListener {
            binding.webView.reload()
        }
        binding.webView.setOnScrollChangeListener { _, _, scrollY, _, _ ->
            binding.swipeRefresh.isEnabled = (scrollY == 0)
        }
    }

    // ── Dark mode injection ───────────────────────────────────────────────────
    private fun injectDarkModeIfNeeded() {
        val isDark = (resources.configuration.uiMode and
                Configuration.UI_MODE_NIGHT_MASK) == Configuration.UI_MODE_NIGHT_YES
        val js = if (isDark) {
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
            """
            (function(){
              if(localStorage.getItem('theme')==='dark') return;
            })();
            """.trimIndent()
        }
        binding.webView.evaluateJavascript(js, null)
    }

    // ── Barcode scan flow ─────────────────────────────────────────────────────

    /** Called by AndroidBridge (JS thread) — must post to main thread. */
    fun startBarcodeScan(ns: String, id: Int) {
        pendingScanNs = ns
        pendingScanId = id
        runOnUiThread {
            if (ContextCompat.checkSelfPermission(this, Manifest.permission.CAMERA)
                == PackageManager.PERMISSION_GRANTED
            ) {
                launchCamera()
            } else {
                cameraPermissionLauncher.launch(Manifest.permission.CAMERA)
            }
        }
    }

    private fun launchCamera() {
        val photoFile = File(
            cacheDir.resolve("barcode_photos").also { it.mkdirs() },
            "scan_${System.currentTimeMillis()}.jpg"
        )
        val uri = FileProvider.getUriForFile(
            this, "${packageName}.fileprovider", photoFile
        )
        photoUri = uri
        takePictureLauncher.launch(uri)
    }

    private fun processBarcodeImage(uri: Uri) {
        val bitmap = contentResolver.openInputStream(uri)?.use {
            BitmapFactory.decodeStream(it)
        } ?: run { deliverScanError("Could not read photo"); return }

        val image = InputImage.fromBitmap(bitmap, 0)
        BarcodeScanning.getClient().process(image)
            .addOnSuccessListener { barcodes ->
                val barcode = barcodes.firstOrNull()?.rawValue
                if (barcode != null) {
                    lookupBarcode(barcode)
                } else {
                    deliverScanError("No barcode found — try again")
                }
            }
            .addOnFailureListener { e ->
                deliverScanError("Scan failed: ${e.message}")
            }
    }

    /** Fetch product data from Open Food Facts (background thread via OkHttp). */
    private fun lookupBarcode(barcode: String) {
        deliverScanStatus("Looking up barcode…")

        val url = "https://world.openfoodfacts.org/api/v2/product/$barcode" +
                "?fields=product_name,nutriments,product_quantity,serving_quantity,serving_size"

        val request = Request.Builder().url(url)
            .header("User-Agent", "SmartShoppingCalculator/1.0 (Android; barcode-lookup)")
            .build()

        OkHttpClient().newCall(request).enqueue(object : Callback {
            override fun onFailure(call: Call, e: IOException) {
                deliverScanError("Network error: ${e.message}")
            }

            override fun onResponse(call: Call, response: Response) {
                val body = response.body?.string()
                if (!response.isSuccessful || body == null) {
                    deliverScanError("Product not found (${response.code})")
                    return
                }
                try {
                    parseAndDeliverOFF(barcode, body)
                } catch (e: Exception) {
                    Log.w("BarcodeOFF", "Parse error for $barcode", e)
                    deliverScanError("Could not parse product data")
                }
            }
        })
    }

    /**
     * Parse Open Food Facts response and call the JS callback.
     *
     * For protein powder (ns="pro") we extract:
     *   - product_name
     *   - product_quantity (bag weight in g)
     *   - nutriments.proteins_100g or proteins_serving
     *   - serving_quantity (g per serving)
     *
     * For toilet paper (ns="tp") we deliver the product name only —
     * OFF has no useful TP metrics.
     */
    private fun parseAndDeliverOFF(barcode: String, body: String) {
        val root    = JSONObject(body)
        val status  = root.optInt("status", 0)
        if (status != 1) { deliverScanError("Product not in database"); return }

        val product = root.getJSONObject("product")
        val name    = product.optString("product_name", "").trim()
            .ifEmpty { "Product $barcode" }

        if (pendingScanNs == "pro") {
            val nutriments   = product.optJSONObject("nutriments") ?: JSONObject()
            val proteins100g = nutriments.optDouble("proteins_100g", Double.NaN)
            val servingQty   = product.optDouble("serving_quantity", Double.NaN) // g per serving
            val bagWeight    = product.optDouble("product_quantity", Double.NaN)  // total g

            // Protein per serving: either direct or derive from per-100g
            val proteinPerServing = when {
                !servingQty.isNaN() && !proteins100g.isNaN() -> proteins100g * servingQty / 100.0
                else -> Double.NaN
            }

            // Servings per bag
            val servings = when {
                !bagWeight.isNaN() && !servingQty.isNaN() && servingQty > 0 ->
                    (bagWeight / servingQty)
                else -> Double.NaN
            }

            val js = buildJsCallback(
                ns     = "pro",
                id     = pendingScanId,
                name   = name,
                weight = if (!bagWeight.isNaN()) bagWeight.toInt().toString() else "",
                servings = if (!servings.isNaN()) "%.0f".format(servings) else "",
                protein  = if (!proteinPerServing.isNaN()) "%.1f".format(proteinPerServing) else ""
            )
            runOnUiThread { binding.webView.evaluateJavascript(js, null) }
        } else {
            // TP: just deliver the name so the user doesn't have to type it
            val js = buildJsCallback(ns = "tp", id = pendingScanId, name = name)
            runOnUiThread { binding.webView.evaluateJavascript(js, null) }
        }
    }

    // ── JS delivery helpers ───────────────────────────────────────────────────

    private fun buildJsCallback(
        ns: String, id: Int, name: String,
        weight: String = "", servings: String = "", protein: String = ""
    ): String {
        val escaped = name.replace("'", "\\'")
        return "onBarcodeResult('$ns',$id,'$escaped','$weight','$servings','$protein');"
    }

    private fun deliverScanStatus(msg: String) {
        val escaped = msg.replace("'", "\\'")
        runOnUiThread {
            binding.webView.evaluateJavascript("onBarcodeStatus('$escaped');", null)
        }
    }

    private fun deliverScanError(msg: String) {
        val escaped = msg.replace("'", "\\'")
        runOnUiThread {
            binding.webView.evaluateJavascript("onBarcodeError('$escaped');", null)
        }
    }

    // ── Back navigation ───────────────────────────────────────────────────────
    @Suppress("OVERRIDE_DEPRECATION")
    override fun onBackPressed() {
        if (binding.webView.canGoBack()) {
            binding.webView.goBack()
        } else {
            @Suppress("DEPRECATION")
            super.onBackPressed()
        }
    }

    // ── Lifecycle ─────────────────────────────────────────────────────────────
    override fun onSaveInstanceState(outState: Bundle) {
        super.onSaveInstanceState(outState)
        binding.webView.saveState(outState)
    }

    override fun onResume()  { super.onResume();  binding.webView.onResume()  }
    override fun onPause()   { super.onPause();   binding.webView.onPause()   }
    override fun onDestroy() {
        binding.webView.apply { stopLoading(); clearHistory(); removeAllViews(); destroy() }
        super.onDestroy()
    }
}
