//! A release font must cover the actual Chinese catalogs, not only a sample word.
use skrifa::MetadataProvider;

#[test]
fn bundled_chinese_covers_the_interface_catalogs() {
    let faces = pdfcraft_fonts::ui_chinese_fonts();
    if faces.is_empty() && std::env::var_os("PDFCRAFT_CHINESE_REQUIRED").is_none() {
        return;
    }
    assert!(!faces.is_empty(), "the release must embed a Hans face");
    for catalog in [
        include_str!("../../ui-egui/src/i18n/zh-hans.tsv"),
        include_str!("../../ui-egui/src/i18n/zh-hant.tsv"),
    ] {
        for ch in catalog.lines().filter_map(|l| l.rsplit('\t').next()).flat_map(str::chars).filter(|c| ('\u{3400}'..='\u{9fff}').contains(c)) {
            assert!(
                faces.iter().any(|face| skrifa::FontRef::new(face.bytes).ok().is_some_and(|font| font.charmap().map(ch).is_some())),
                "no Chinese glyph for {ch} (U+{:04X})", ch as u32
            );
        }
    }
}
