use dioxus::document::eval;
use serde::Deserialize;

const SCAN_JS: &str = r#"
try {
    const barcode = await new Promise((resolve, reject) => {
        if (typeof BarcodeDetector === 'undefined') {
            reject(new Error('scan.unsupported'));
            return;
        }
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = 'image/*';
        input.capture = 'environment';
        const timer = setTimeout(() => {
            input.remove();
            reject(new Error('scan.cancelled'));
        }, 60000);
        input.onchange = () => {
            clearTimeout(timer);
            const file = input.files && input.files[0];
            if (!file) {
                input.remove();
                reject(new Error('scan.cancelled'));
                return;
            }
            const detector = new BarcodeDetector();
            const img = new Image();
            const url = URL.createObjectURL(file);
            img.onload = () => {
                detector.detect(img).then((codes) => {
                    URL.revokeObjectURL(url);
                    input.remove();
                    const code = codes && codes.length ? codes[0].rawValue : null;
                    if (!code) reject(new Error('scan.no_barcode'));
                    else resolve(code);
                }).catch((e) => {
                    URL.revokeObjectURL(url);
                    input.remove();
                    reject(e);
                });
            };
            img.onerror = () => {
                URL.revokeObjectURL(url);
                input.remove();
                reject(new Error('scan.read'));
            };
            img.src = url;
        };
        input.click();
    });

    const api = 'https://world.openfoodfacts.org/api/v2/product/'
        + encodeURIComponent(barcode)
        + '?fields=product_name,nutriments,product_quantity,serving_quantity,serving_size';
    const res = await fetch(api, { headers: { Accept: 'application/json' } });
    if (!res.ok) return { ok: false, name: '', weight: '', servings: '', protein: '', error: 'scan.not_found' };
    const data = await res.json();
    if (!data || data.status !== 1 || !data.product) {
        return { ok: false, name: '', weight: '', servings: '', protein: '', error: 'scan.not_in_db' };
    }
    const p = data.product;
    const name = (p.product_name || '').trim() || ('Product ' + barcode);
    const p100 = p.nutriments && p.nutriments.proteins_100g != null ? Number(p.nutriments.proteins_100g) : NaN;
    const sq = p.serving_quantity != null ? Number(p.serving_quantity) : NaN;
    const bq = p.product_quantity != null ? Number(p.product_quantity) : NaN;
    let weight = '';
    let servings = '';
    let protein = '';
    if (!Number.isNaN(bq) && bq > 0) weight = String(Math.round(bq));
    if (!Number.isNaN(sq) && !Number.isNaN(p100) && sq > 0) protein = (p100 * sq / 100).toFixed(1);
    if (!Number.isNaN(bq) && !Number.isNaN(sq) && bq > 0 && sq > 0) servings = String(Math.round(bq / sq));
    return { ok: true, name: name, weight: weight, servings: servings, protein: protein, error: '' };
} catch (e) {
    const code = (e && e.message) || 'scan.unknown';
    return { ok: false, name: '', weight: '', servings: '', protein: '', error: code };
}
"#;

#[derive(Deserialize)]
pub struct ScanOutcome {
    pub ok: bool,
    pub name: String,
    pub weight: String,
    pub servings: String,
    pub protein: String,
    pub error: String,
}

impl Default for ScanOutcome {
    fn default() -> Self {
        Self {
            ok: false,
            name: String::new(),
            weight: String::new(),
            servings: String::new(),
            protein: String::new(),
            error: "scan.unknown".to_string(),
        }
    }
}

pub async fn scan_barcode() -> ScanOutcome {
    eval(SCAN_JS).join::<ScanOutcome>().await.unwrap_or_default()
}
