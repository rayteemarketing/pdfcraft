//! The About dialog and the command registry carry no community or social links (the home screen
//! is checked in tests/ui.rs), and the About dialog's Contributors and Models tabs.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_ui_egui::{Dialog, PdfCraftApp};

fn harness(setup: impl FnOnce(&mut PdfCraftApp) + 'static) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn no_community_links_in_about_or_commands() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Contributors");
    for text in ["Discord", "ArtCraft"] {
        assert_eq!(h.query_all_by_label_contains(text).count(), 0, "the About dialog shows {text:?}");
    }
    for command in pdfcraft_engine::commands::COMMANDS {
        assert!(!command.label.contains("Discord") && !command.label.contains("ArtCraft"), "{} is a community link", command.id);
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves. A
/// fixed number of frames is a guess at how long that takes: it was too few after the 0.5.0
/// contributors refresh (78 names instead of 5) on some machines, the click on Table landed where
/// the button had been, and the table never opened (#671).
fn settled<'a>(h: &'a mut Harness<'static, PdfCraftApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Contributors").click();
    // The owner is always in the compiled-in credits (contributors/contributors.json), shown by username.
    settled(&mut h, "@echelon");
    settled(&mut h, "Table").click();
    settled(&mut h, "PRs");
    settled(&mut h, "Display name").click();
    settled(&mut h, "Brandon Thomas");
    settled(&mut h, "Models").click();
    h.run_steps(4);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}
