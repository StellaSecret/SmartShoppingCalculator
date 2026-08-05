use crate::models::Lang;

pub struct Strings {
    pub subtitle: &'static str,
    pub dark_btn: &'static str,
    pub light_btn: &'static str,
    pub tab_tp: &'static str,
    pub tab_pro: &'static str,
    pub add_roll: &'static str,
    pub add_powder: &'static str,
    pub rolls_word: &'static str,
    pub powders_word: &'static str,
    pub save_session: &'static str,
    pub save_session_title: &'static str,
    pub save_session_title_pro: &'static str,
    pub restore_session: &'static str,
    pub restore_title: &'static str,
    pub clear_saved: &'static str,
    pub clear_title: &'static str,
    pub toast_saved: &'static str,
    pub toast_restored: &'static str,
    pub toast_restore_fail: &'static str,
    pub toast_cleared: &'static str,
    pub by_weight: &'static str,
    pub by_sheets: &'static str,
    pub by_diameter: &'static str,
    pub by_hand: &'static str,
    pub hand_title: &'static str,
    pub hand_subtitle: &'static str,
    pub hand_finger: &'static str,
    pub hand_palm: &'static str,
    pub hand_thumb: &'static str,
    pub hand_tip: &'static str,
    pub card_name: &'static str,
    pub rolls_in_pack: &'static str,
    pub total_weight: &'static str,
    pub tube_weight: &'static str,
    pub sheet_count: &'static str,
    pub sheet_length: &'static str,
    pub sheet_width: &'static str,
    pub outer_diam: &'static str,
    pub tube_diam: &'static str,
    pub roll_width: &'static str,
    pub hand_roll_diam: &'static str,
    pub hand_tube_size: &'static str,
    pub hand_tube_opt1: &'static str,
    pub hand_tube_opt2: &'static str,
    pub hand_roll_width: &'static str,
    pub hand_width_opt1: &'static str,
    pub hand_width_opt2: &'static str,
    pub hand_width_opt3: &'static str,
    pub hand_width_opt4: &'static str,
    pub hand_est_tip: &'static str,
    pub fill_fields: &'static str,
    pub per_roll: &'static str,
    pub best_value: &'static str,
    pub empty_tp: &'static str,
    pub empty_tp_valid: &'static str,
    pub ranking_prefix: &'static str,
    pub method_weight: &'static str,
    pub method_sheets: &'static str,
    pub method_diam: &'static str,
    pub method_hand: &'static str,
    pub lifetime_title: &'static str,
    pub rolls_per_week_lbl: &'static str,
    pub rolls_per_week_ph: &'static str,
    pub bag_weight: &'static str,
    pub servings_per_bag: &'static str,
    pub protein_per_serving: &'static str,
    pub per_gram_protein: &'static str,
    pub per_serving: &'static str,
    pub empty_pro: &'static str,
    pub empty_pro_valid: &'static str,
    pub ranking_protein: &'static str,
    pub incomplete_label: &'static str,
    pub servings_per_week_lbl: &'static str,
    pub servings_per_week_ph: &'static str,
    pub footer: &'static str,
    pub more_suffix: &'static str,
}

pub const EN: Strings = Strings {
    subtitle: "Stop guessing — compare products by what actually matters.",
    dark_btn: "🌙 dark",
    light_btn: "☀️ light",
    tab_tp: "🧻 toilet paper",
    tab_pro: "💪 protein powder",
    add_roll: "+ add roll",
    add_powder: "+ add powder",
    rolls_word: "rolls",
    powders_word: "powders",
    save_session: "💾 save session",
    save_session_title: "Save current rolls to browser storage",
    save_session_title_pro: "Save current powders to browser storage",
    restore_session: "↩ restore",
    restore_title: "Restore last saved session",
    clear_saved: "✕ clear saved",
    clear_title: "Clear saved session",
    toast_saved: "✓ saved",
    toast_restored: "✓ restored",
    toast_restore_fail: "restore failed",
    toast_cleared: "cleared",
    by_weight: "By weight",
    by_sheets: "By sheet count",
    by_diameter: "By diameter",
    by_hand: "👋 By hand",
    hand_title: "your hand measurements",
    hand_subtitle: "(set once, used for all rolls)",
    hand_finger: "finger width (mm)",
    hand_palm: "palm width (mm)",
    hand_thumb: "thumb length (mm)",
    hand_tip: "Tip: measure your finger width and palm width with a ruler once. Defaults are average adult values.",
    card_name: "Name",
    rolls_in_pack: "Rolls in pack",
    total_weight: "Total roll weight (g)",
    tube_weight: "Tube weight (g)",
    sheet_count: "Sheet count",
    sheet_length: "Sheet length (mm)",
    sheet_width: "Sheet width (mm)",
    outer_diam: "Outer diameter (mm)",
    tube_diam: "Tube diameter (mm)",
    roll_width: "Roll width (mm)",
    hand_roll_diam: "roll diameter — finger-widths across",
    hand_tube_size: "tube size",
    hand_tube_opt1: "fits 1 finger (~40mm, standard)",
    hand_tube_opt2: "fits 2 fingers (~36mm, compact)",
    hand_roll_width: "roll width",
    hand_width_opt1: "about 1 palm wide (~85mm)",
    hand_width_opt2: "about 1 thumb length (~60mm)",
    hand_width_opt3: "about 5 finger-widths",
    hand_width_opt4: "about 6 finger-widths",
    hand_est_tip: "estimate how many fingers wide the roll is when looking at it straight on",
    fill_fields: "fill in all fields",
    per_roll: "/roll",
    best_value: "best value",
    empty_tp: "add at least two rolls to compare",
    empty_tp_valid: "add at least two valid rolls to compare",
    ranking_prefix: "ranking — ",
    method_weight: "cost per gram of paper",
    method_sheets: "cost per 100cm²",
    method_diam: "cost per cm³",
    method_hand: "estimated cost per cm³ (hand method)",
    lifetime_title: "lifetime cost projection",
    rolls_per_week_lbl: "rolls used per week",
    rolls_per_week_ph: "e.g. 1",
    bag_weight: "Bag weight (g)",
    servings_per_bag: "Servings per bag",
    protein_per_serving: "Protein per serving (g)",
    per_gram_protein: "per gram of protein",
    per_serving: "/serving",
    empty_pro: "add at least two powders to compare",
    empty_pro_valid: "add at least two valid powders to compare",
    ranking_protein: "ranking — cost per gram of protein",
    incomplete_label: "incomplete",
    servings_per_week_lbl: "servings per week",
    servings_per_week_ph: "e.g. 5",
    footer: "no data is sent anywhere · runs entirely in your browser",
    more_suffix: "more",
};

pub const FR: Strings = Strings {
    subtitle: "Fini les approximations — comparez les produits par ce qui compte vraiment.",
    dark_btn: "🌙 sombre",
    light_btn: "☀️ clair",
    tab_tp: "🧻 papier toilette",
    tab_pro: "💪 protéine en poudre",
    add_roll: "+ ajouter un rouleau",
    add_powder: "+ ajouter une poudre",
    rolls_word: "rouleaux",
    powders_word: "poudres",
    save_session: "💾 sauvegarder",
    save_session_title: "Sauvegarder les rouleaux dans le navigateur",
    save_session_title_pro: "Sauvegarder les poudres dans le navigateur",
    restore_session: "↩ restaurer",
    restore_title: "Restaurer la dernière session sauvegardée",
    clear_saved: "✕ effacer",
    clear_title: "Effacer la session sauvegardée",
    toast_saved: "✓ sauvegardé",
    toast_restored: "✓ restauré",
    toast_restore_fail: "échec de restauration",
    toast_cleared: "effacé",
    by_weight: "Par poids",
    by_sheets: "Par nombre de feuilles",
    by_diameter: "Par diamètre",
    by_hand: "👋 À la main",
    hand_title: "mesures de votre main",
    hand_subtitle: "(à définir une fois, utilisé pour tous les rouleaux)",
    hand_finger: "largeur d'un doigt (mm)",
    hand_palm: "largeur de la paume (mm)",
    hand_thumb: "longueur du pouce (mm)",
    hand_tip: "Conseil : mesurez la largeur d'un doigt et de votre paume avec une règle une fois. Les valeurs par défaut correspondent à un adulte moyen.",
    card_name: "Nom",
    rolls_in_pack: "Rouleaux par paquet",
    total_weight: "Poids total du rouleau (g)",
    tube_weight: "Poids du tube (g)",
    sheet_count: "Nombre de feuilles",
    sheet_length: "Longueur d'une feuille (mm)",
    sheet_width: "Largeur d'une feuille (mm)",
    outer_diam: "Diamètre extérieur (mm)",
    tube_diam: "Diamètre du tube (mm)",
    roll_width: "Largeur du rouleau (mm)",
    hand_roll_diam: "diamètre du rouleau — largeurs de doigt",
    hand_tube_size: "taille du tube",
    hand_tube_opt1: "rentre 1 doigt (~40mm, standard)",
    hand_tube_opt2: "rentre 2 doigts (~36mm, compact)",
    hand_roll_width: "largeur du rouleau",
    hand_width_opt1: "environ 1 paume (~85mm)",
    hand_width_opt2: "environ 1 longueur de pouce (~60mm)",
    hand_width_opt3: "environ 5 largeurs de doigt",
    hand_width_opt4: "environ 6 largeurs de doigt",
    hand_est_tip: "estimez combien de doigts de large fait le rouleau vu de face",
    fill_fields: "remplissez tous les champs",
    per_roll: "/rouleau",
    best_value: "meilleur rapport",
    empty_tp: "ajoutez au moins deux rouleaux à comparer",
    empty_tp_valid: "ajoutez au moins deux rouleaux valides à comparer",
    ranking_prefix: "classement — ",
    method_weight: "coût par gramme de papier",
    method_sheets: "coût par 100cm²",
    method_diam: "coût par cm³",
    method_hand: "coût estimé par cm³ (méthode main)",
    lifetime_title: "projection sur un an",
    rolls_per_week_lbl: "rouleaux utilisés par semaine",
    rolls_per_week_ph: "ex. 1",
    bag_weight: "Poids du sachet (g)",
    servings_per_bag: "Portions par sachet",
    protein_per_serving: "Protéines par portion (g)",
    per_gram_protein: "par gramme de protéine",
    per_serving: "/portion",
    empty_pro: "ajoutez au moins deux poudres à comparer",
    empty_pro_valid: "ajoutez au moins deux poudres valides à comparer",
    ranking_protein: "classement — coût par gramme de protéine",
    incomplete_label: "incomplet",
    servings_per_week_lbl: "portions par semaine",
    servings_per_week_ph: "ex. 5",
    footer: "aucune donnée envoyée · fonctionne entièrement dans votre navigateur",
    more_suffix: "plus",
};

pub fn s(lang: Lang) -> &'static Strings {
    match lang {
        Lang::En => &EN,
        Lang::Fr => &FR,
    }
}

pub fn card_price_label(lang: Lang, cur: &str) -> String {
    match lang {
        Lang::En => format!("Price ({cur})"),
        Lang::Fr => format!("Prix ({cur})"),
    }
}

pub fn count_rolls(lang: Lang, n: usize) -> String {
    format!("{n} / 4 {}", s(lang).rolls_word)
}

pub fn count_powders(lang: Lang, n: usize) -> String {
    format!("{n} / 4 {}", s(lang).powders_word)
}

/// `choosingSaves` — HTML snippet (mirrors the JS template literal with
/// embedded `<strong>` tags), rendered with `dangerous_inner_html`.
pub fn choosing_saves(lang: Lang, name: &str, pct: &str, per_gram: bool) -> String {
    match lang {
        Lang::En => {
            let suffix = if per_gram { " per gram of protein." } else { "." };
            format!(
                "choosing <strong style=\"color:var(--text)\">{name}</strong> over the most expensive saves you <strong style=\"color:var(--win)\">{pct}%</strong>{suffix}"
            )
        }
        Lang::Fr => {
            let suffix = if per_gram { " par gramme de protéine." } else { "." };
            format!(
                "choisir <strong style=\"color:var(--text)\">{name}</strong> plutôt que {other} vous fait économiser <strong style=\"color:var(--win)\">{pct}%</strong>{suffix}",
                other = if per_gram { "la plus chère" } else { "le plus cher" }
            )
        }
    }
}

/// `lifetimeTp` / `lifetimePro` — same shape for both pages, just with a
/// different unit word ("roll(s)" vs "serving(s)") and time frame word.
pub fn lifetime_line(lang: Lang, n: f64, unit_singular_en: &str, unit_plural_en: &str, unit_fr: &str, unit_fr_plural: &str, best: &str, worst: &str, save: &str, cur: &str) -> String {
    match lang {
        Lang::En => {
            let unit_word = if (n - 1.0).abs() < f64::EPSILON { unit_singular_en } else { unit_plural_en };
            format!(
                "at <strong style=\"color:var(--text)\">{n} {unit_word}/week</strong> · best option costs <strong style=\"color:var(--win)\">{cur}{best}/yr</strong> vs {cur}{worst}/yr for the most expensive — you save <strong>{cur}{save}/year</strong>."
            )
        }
        Lang::Fr => {
            let unit_word = if (n - 1.0).abs() < f64::EPSILON { unit_fr } else { unit_fr_plural };
            format!(
                "à <strong style=\"color:var(--text)\">{n} {unit_word}/semaine</strong> · la meilleure option coûte <strong style=\"color:var(--win)\">{cur}{best}/an</strong> contre {cur}{worst}/an pour la plus chère — vous économisez <strong>{cur}{save}/an</strong>."
            )
        }
    }
}
