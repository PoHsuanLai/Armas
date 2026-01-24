//! Tests for Browser component using egui_kittest

use armas::components::navigation::{Browser, BrowserItem, BrowserTab, FileMetadata};
use armas::browser::BrowserMode;
use egui_kittest::Harness;
use std::path::PathBuf;

/// Test that Browser renders without panicking
#[test]
fn test_browser_renders() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new();
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser with fixed dimensions
#[test]
fn test_browser_fixed_dimensions() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser in IconOnly mode
#[test]
fn test_browser_icon_only_mode() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(100.0) // < 120px triggers IconOnly
            .height(300.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser in Compact mode
#[test]
fn test_browser_compact_mode() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(150.0) // 120-200px triggers Compact
            .height(300.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser in Normal mode
#[test]
fn test_browser_normal_mode() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(250.0) // 200-280px triggers Normal
            .height(300.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser in Expanded mode
#[test]
fn test_browser_expanded_mode() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(350.0) // > 280px triggers Expanded
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser with forced mode
#[test]
fn test_browser_forced_mode() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(400.0)
            .mode(BrowserMode::Compact) // Force Compact even though width is large
            .height(300.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test BrowserMode from_width
#[test]
fn test_browser_mode_from_width() {
    assert_eq!(BrowserMode::from_width(80.0), BrowserMode::IconOnly);
    assert_eq!(BrowserMode::from_width(150.0), BrowserMode::Compact);
    assert_eq!(BrowserMode::from_width(250.0), BrowserMode::Normal);
    assert_eq!(BrowserMode::from_width(350.0), BrowserMode::Expanded);
}

/// Test BrowserMode search_height
#[test]
fn test_browser_mode_search_height() {
    assert_eq!(BrowserMode::IconOnly.search_height(), 0.0);
    assert_eq!(BrowserMode::Compact.search_height(), 28.0);
    assert_eq!(BrowserMode::Normal.search_height(), 32.0);
    assert_eq!(BrowserMode::Expanded.search_height(), 32.0);
}

/// Test BrowserMode item_height
#[test]
fn test_browser_mode_item_height() {
    assert_eq!(BrowserMode::IconOnly.item_height(), 32.0);
    assert_eq!(BrowserMode::Compact.item_height(), 36.0);
    assert_eq!(BrowserMode::Normal.item_height(), 48.0);
    assert_eq!(BrowserMode::Expanded.item_height(), 56.0);
}

/// Test BrowserTab all
#[test]
fn test_browser_tab_all() {
    let tabs = BrowserTab::all();
    assert_eq!(tabs.len(), 5);
    assert!(tabs.contains(&BrowserTab::Audio));
    assert!(tabs.contains(&BrowserTab::Plugins));
    assert!(tabs.contains(&BrowserTab::Presets));
    assert!(tabs.contains(&BrowserTab::Loops));
    assert!(tabs.contains(&BrowserTab::Samples));
}

/// Test BrowserTab labels
#[test]
fn test_browser_tab_labels() {
    assert_eq!(BrowserTab::Audio.label(), "Audio");
    assert_eq!(BrowserTab::Plugins.label(), "Plugins");
    assert_eq!(BrowserTab::Presets.label(), "Presets");
    assert_eq!(BrowserTab::Loops.label(), "Loops");
    assert_eq!(BrowserTab::Samples.label(), "Samples");
}

/// Test BrowserTab icons
#[test]
fn test_browser_tab_icons() {
    assert_eq!(BrowserTab::Audio.icon(), "🎵");
    assert_eq!(BrowserTab::Plugins.icon(), "🔌");
    assert_eq!(BrowserTab::Presets.icon(), "🎛️");
    assert_eq!(BrowserTab::Loops.icon(), "🔁");
    assert_eq!(BrowserTab::Samples.icon(), "🥁");
}

/// Test Browser with custom items
#[test]
fn test_browser_custom_items() {
    let mut harness = Harness::new_ui(|ui| {
        let items = vec![
            BrowserItem {
                name: "Test File.wav".to_string(),
                path: PathBuf::from("/samples/Test File.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 44100,
                    sample_rate: Some(44100),
                    bpm: Some(120.0),
                    key: Some("C".to_string()),
                    duration: Some(1.0),
                }),
                is_favorite: false,
                tags: vec!["Test".to_string()],
            },
            BrowserItem {
                name: "Test Folder".to_string(),
                path: PathBuf::from("/samples/Test Folder"),
                is_directory: true,
                metadata: None,
                is_favorite: false,
                tags: vec![],
            },
        ];

        let mut browser = Browser::new()
            .items(items)
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test Browser getter methods
#[test]
fn test_browser_getters() {
    let browser = Browser::new();

    assert!(browser.selected_file().is_none());
    assert!(browser.current_folder().is_none());
    assert_eq!(browser.search_query(), "");
    assert_eq!(browser.active_tab(), BrowserTab::Audio);
    assert!(browser.active_filters().is_empty());
}

/// Test BrowserItem creation
#[test]
fn test_browser_item_creation() {
    let item = BrowserItem {
        name: "Kick.wav".to_string(),
        path: PathBuf::from("/samples/Drums/Kick.wav"),
        is_directory: false,
        metadata: Some(FileMetadata {
            size: 44100,
            sample_rate: Some(44100),
            bpm: Some(128.0),
            key: Some("E".to_string()),
            duration: Some(0.5),
        }),
        is_favorite: true,
        tags: vec!["Drums".to_string(), "Kick".to_string()],
    };

    assert_eq!(item.name, "Kick.wav");
    assert!(!item.is_directory);
    assert!(item.is_favorite);
    assert_eq!(item.tags.len(), 2);
}

/// Test FileMetadata creation
#[test]
fn test_file_metadata() {
    let metadata = FileMetadata {
        size: 882000,
        sample_rate: Some(44100),
        bpm: Some(120.0),
        key: Some("Am".to_string()),
        duration: Some(2.0),
    };

    assert_eq!(metadata.size, 882000);
    assert_eq!(metadata.sample_rate, Some(44100));
    assert_eq!(metadata.bpm, Some(120.0));
    assert_eq!(metadata.key, Some("Am".to_string()));
    assert_eq!(metadata.duration, Some(2.0));
}

/// Test Browser response fields
#[test]
fn test_browser_response() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .width(300.0)
            .height(400.0);
        let response = browser.show(ui);

        // Check response fields exist
        assert!(response.selected_file.is_none());
        assert!(response.selected_folder.is_none());
        assert!(response.tab_changed.is_none());
        assert!(!response.filters_changed);
    });

    harness.run();
}

/// Test Browser with empty items
#[test]
fn test_browser_empty_items() {
    let mut harness = Harness::new_ui(|ui| {
        let mut browser = Browser::new()
            .items(vec![])
            .width(300.0)
            .height(400.0);
        browser.show(ui);
    });

    harness.run();
}

/// Test BrowserMode feature flags
#[test]
fn test_browser_mode_features() {
    // IconOnly
    assert!(!BrowserMode::IconOnly.show_category_tabs());
    assert!(!BrowserMode::IconOnly.show_filter_chips());
    assert!(!BrowserMode::IconOnly.show_breadcrumb_text());
    assert!(!BrowserMode::IconOnly.show_metadata_inline());

    // Compact
    assert!(!BrowserMode::Compact.show_category_tabs());
    assert!(!BrowserMode::Compact.show_filter_chips());
    assert!(!BrowserMode::Compact.show_breadcrumb_text());
    assert!(!BrowserMode::Compact.show_metadata_inline());

    // Normal
    assert!(BrowserMode::Normal.show_category_tabs());
    assert!(BrowserMode::Normal.show_filter_chips());
    assert!(BrowserMode::Normal.show_breadcrumb_text());
    assert!(!BrowserMode::Normal.show_metadata_inline());

    // Expanded
    assert!(BrowserMode::Expanded.show_category_tabs());
    assert!(BrowserMode::Expanded.show_filter_chips());
    assert!(BrowserMode::Expanded.show_breadcrumb_text());
    assert!(BrowserMode::Expanded.show_metadata_inline());
}

/// Test BrowserMode max_filename_chars
#[test]
fn test_browser_mode_max_filename_chars() {
    assert_eq!(BrowserMode::IconOnly.max_filename_chars(), 0);
    assert_eq!(BrowserMode::Compact.max_filename_chars(), 12);
    assert_eq!(BrowserMode::Normal.max_filename_chars(), 20);
    assert_eq!(BrowserMode::Expanded.max_filename_chars(), 40);
}
