//! Browser/File Manager Component
//!
//! File browser for audio files, plugins, presets, and samples.
//! Features search, filtering, folder navigation, and drag-and-drop support.

use crate::components::basic::{Chip, ChipSize, ChipType};
use crate::components::cards::{Card, CardVariant};
use crate::ext::ArmasContextExt;
use egui::{Response, Ui, Vec2};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Browser category tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserTab {
    /// Audio files (.wav, .mp3, .flac, etc.)
    Audio,
    /// VST/AU plugins
    Plugins,
    /// Plugin presets
    Presets,
    /// Audio loops
    Loops,
    /// One-shot samples
    Samples,
}

impl BrowserTab {
    /// Get all available tabs
    pub fn all() -> Vec<Self> {
        vec![
            Self::Audio,
            Self::Plugins,
            Self::Presets,
            Self::Loops,
            Self::Samples,
        ]
    }

    /// Get tab label
    pub fn label(&self) -> &'static str {
        match self {
            Self::Audio => "Audio",
            Self::Plugins => "Plugins",
            Self::Presets => "Presets",
            Self::Loops => "Loops",
            Self::Samples => "Samples",
        }
    }
}

/// Sort mode for file list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortMode {
    /// Sort by name alphabetically
    Name,
    /// Sort by date modified
    Date,
    /// Sort by file size
    Size,
    /// Sort by BPM (audio files only)
    Bpm,
    /// Sort by key (audio files only)
    Key,
}

/// File metadata for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// File size in bytes
    pub size: u64,
    /// Sample rate (for audio)
    pub sample_rate: Option<u32>,
    /// BPM (for audio)
    pub bpm: Option<f32>,
    /// Musical key (for audio)
    pub key: Option<String>,
    /// Duration in seconds (for audio)
    pub duration: Option<f32>,
}

/// Browser item (file or folder)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserItem {
    /// Item name
    pub name: String,
    /// Full path
    pub path: PathBuf,
    /// Is this a directory?
    pub is_directory: bool,
    /// File metadata (None for directories)
    pub metadata: Option<FileMetadata>,
    /// Is this item favorited?
    pub is_favorite: bool,
    /// Tags for filtering (e.g., "Kick", "Snare", "Bass", "Lead")
    pub tags: Vec<String>,
}

/// Response from the browser
#[derive(Debug, Clone)]
pub struct BrowserResponse {
    /// The UI response
    pub response: Response,
    /// Selected file path (if a file was clicked this frame)
    pub selected_file: Option<PathBuf>,
    /// Selected folder path (if a folder was clicked this frame)
    pub selected_folder: Option<PathBuf>,
    /// Active tab changed (if tab was switched this frame)
    pub tab_changed: Option<BrowserTab>,
    /// Active filters changed (if filters were toggled this frame)
    pub filters_changed: bool,
}

/// Browser/File Manager component
///
/// A file browser for audio files, plugins, presets, and samples.
/// Features search, filtering, folder navigation, and preview.
///
/// **Important**: Store the `Browser` instance in your app state to preserve
/// search query, selection, and tab state across frames.
///
/// # Example
///
/// ```rust,no_run
/// use armas::components::Browser;
///
/// // In your app struct:
/// struct MyApp {
///     browser: Browser,
/// }
///
/// // In your UI code:
/// let response = self.browser.show(ui);
///
/// // Check if a file was selected this frame
/// if let Some(path) = response.selected_file {
///     println!("Selected: {:?}", path);
/// }
///
/// // Check if tab changed
/// if let Some(new_tab) = response.tab_changed {
///     println!("Switched to tab: {:?}", new_tab);
/// }
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct Browser {
    /// Current search query
    search_query: String,
    /// Active tab
    active_tab: BrowserTab,
    /// Current folder path
    current_folder: Option<PathBuf>,
    /// Selected file
    selected_file: Option<PathBuf>,
    /// Active filter tags
    filter_tags: Vec<String>,
    /// Sort mode
    sort_mode: SortMode,
    /// Browser width
    width: f32,
    /// Browser height
    height: f32,
    /// Available items (can be set externally or use mock data)
    items: Vec<BrowserItem>,
}

impl Browser {
    /// Create a new browser
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            active_tab: BrowserTab::Audio,
            current_folder: None,
            selected_file: None,
            filter_tags: Vec::new(),
            sort_mode: SortMode::Name,
            width: 300.0,
            height: 600.0,
            items: Self::mock_items(),
        }
    }

    /// Set browser width
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set browser height
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set custom items (replaces mock data)
    pub fn items(mut self, items: Vec<BrowserItem>) -> Self {
        self.items = items;
        self
    }

    /// Get currently selected file
    pub fn selected_file(&self) -> Option<&PathBuf> {
        self.selected_file.as_ref()
    }

    /// Get current folder
    pub fn current_folder(&self) -> Option<&PathBuf> {
        self.current_folder.as_ref()
    }

    /// Get current search query
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// Get active tab
    pub fn active_tab(&self) -> BrowserTab {
        self.active_tab
    }

    /// Get active filter tags
    pub fn active_filters(&self) -> &[String] {
        &self.filter_tags
    }

    /// Get all available filter tags from items
    fn collect_available_tags(&self) -> Vec<String> {
        let mut tags = std::collections::HashSet::new();
        for item in &self.items {
            for tag in &item.tags {
                tags.insert(tag.clone());
            }
        }
        let mut tags: Vec<_> = tags.into_iter().collect();
        tags.sort();
        tags
    }

    /// Generate mock items for testing with nested folder structure
    fn mock_items() -> Vec<BrowserItem> {
        vec![
            // Root level folders
            BrowserItem {
                name: "Drums".to_string(),
                path: PathBuf::from("/samples/Drums"),
                is_directory: true,
                metadata: None,
                is_favorite: false,
                tags: vec![],
            },
            BrowserItem {
                name: "Bass".to_string(),
                path: PathBuf::from("/samples/Bass"),
                is_directory: true,
                metadata: None,
                is_favorite: false,
                tags: vec![],
            },
            BrowserItem {
                name: "Synths".to_string(),
                path: PathBuf::from("/samples/Synths"),
                is_directory: true,
                metadata: None,
                is_favorite: false,
                tags: vec![],
            },
            // Drums subfolder
            BrowserItem {
                name: "Kicks".to_string(),
                path: PathBuf::from("/samples/Drums/Kicks"),
                is_directory: true,
                metadata: None,
                is_favorite: false,
                tags: vec![],
            },
            BrowserItem {
                name: "Snares".to_string(),
                path: PathBuf::from("/samples/Drums/Snares"),
                is_directory: true,
                metadata: None,
                is_favorite: false,
                tags: vec![],
            },
            BrowserItem {
                name: "Hi-Hats".to_string(),
                path: PathBuf::from("/samples/Drums/Hi-Hats"),
                is_directory: true,
                metadata: None,
                is_favorite: false,
                tags: vec![],
            },
            // Kick samples
            BrowserItem {
                name: "Kick_01.wav".to_string(),
                path: PathBuf::from("/samples/Drums/Kicks/Kick_01.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 441000,
                    sample_rate: Some(44100),
                    bpm: Some(120.0),
                    key: Some("C".to_string()),
                    duration: Some(1.0),
                }),
                is_favorite: true,
                tags: vec!["Kicks".to_string(), "Drums".to_string()],
            },
            BrowserItem {
                name: "Kick_808.wav".to_string(),
                path: PathBuf::from("/samples/Drums/Kicks/Kick_808.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 352000,
                    sample_rate: Some(44100),
                    bpm: Some(140.0),
                    key: Some("F".to_string()),
                    duration: Some(0.8),
                }),
                is_favorite: false,
                tags: vec!["Kicks".to_string(), "808".to_string(), "Drums".to_string()],
            },
            // Snare samples
            BrowserItem {
                name: "Snare_Heavy.wav".to_string(),
                path: PathBuf::from("/samples/Drums/Snares/Snare_Heavy.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 220500,
                    sample_rate: Some(44100),
                    bpm: None,
                    key: None,
                    duration: Some(0.5),
                }),
                is_favorite: false,
                tags: vec!["Snares".to_string(), "Drums".to_string()],
            },
            BrowserItem {
                name: "Snare_Clap.wav".to_string(),
                path: PathBuf::from("/samples/Drums/Snares/Snare_Clap.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 176400,
                    sample_rate: Some(44100),
                    bpm: None,
                    key: None,
                    duration: Some(0.4),
                }),
                is_favorite: true,
                tags: vec![
                    "Snares".to_string(),
                    "Claps".to_string(),
                    "Drums".to_string(),
                ],
            },
            // Hi-Hat samples
            BrowserItem {
                name: "Hat_Closed.wav".to_string(),
                path: PathBuf::from("/samples/Drums/Hi-Hats/Hat_Closed.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 88200,
                    sample_rate: Some(44100),
                    bpm: None,
                    key: None,
                    duration: Some(0.2),
                }),
                is_favorite: false,
                tags: vec!["Hi-Hats".to_string(), "Drums".to_string()],
            },
            BrowserItem {
                name: "Hat_Open.wav".to_string(),
                path: PathBuf::from("/samples/Drums/Hi-Hats/Hat_Open.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 132300,
                    sample_rate: Some(44100),
                    bpm: None,
                    key: None,
                    duration: Some(0.3),
                }),
                is_favorite: false,
                tags: vec!["Hi-Hats".to_string(), "Drums".to_string()],
            },
            // Bass samples
            BrowserItem {
                name: "Bass_Sub.wav".to_string(),
                path: PathBuf::from("/samples/Bass/Bass_Sub.wav"),
                is_directory: false,
                metadata: Some(FileMetadata {
                    size: 882000,
                    sample_rate: Some(44100),
                    bpm: Some(128.0),
                    key: Some("A".to_string()),
                    duration: Some(2.0),
                }),
                is_favorite: true,
                tags: vec!["Bass".to_string(), "Sub".to_string()],
            },
        ]
    }

    /// Show the browser
    pub fn show(&mut self, ui: &mut Ui) -> BrowserResponse {
        let theme = ui.ctx().armas_theme();

        // Track changes this frame
        let mut selected_file_this_frame = None;
        let mut selected_folder_this_frame = None;
        let mut tab_changed_this_frame = None;
        let old_tab = self.active_tab;
        let old_filters = self.filter_tags.clone();
        let mut filters_changed = false;

        // Main container
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(self.width, self.height), egui::Sense::hover());

        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.vertical(|ui| {
                // Search box
                ui.add_space(theme.spacing.sm);
                ui.horizontal(|ui| {
                    ui.add_space(theme.spacing.sm);

                    let search_width = self.width - theme.spacing.sm * 2.0;
                    ui.add_sized(
                        Vec2::new(search_width, 32.0),
                        egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text("🔍 Search files...")
                            .frame(true),
                    );
                });

                ui.add_space(theme.spacing.sm);

                // Breadcrumb navigation
                ui.horizontal(|ui| {
                    ui.add_space(theme.spacing.sm);

                    // Back/Up button
                    if let Some(ref current) = self.current_folder {
                        if ui.button("⬅ Back").clicked() {
                            // Navigate to parent folder
                            if let Some(parent) = current.parent() {
                                if parent == std::path::Path::new("/samples") {
                                    self.current_folder = None; // Go to root
                                } else {
                                    self.current_folder = Some(parent.to_path_buf());
                                }
                            } else {
                                self.current_folder = None; // Go to root
                            }
                        }
                        ui.add_space(theme.spacing.xs);
                    }

                    // Root button
                    if ui.button("📁 Samples").clicked() {
                        self.current_folder = None;
                    }

                    // Build breadcrumb path from current folder
                    if let Some(ref current) = self.current_folder {
                        let path_str = current.to_string_lossy().to_string();
                        let parts: Vec<String> = path_str
                            .trim_start_matches("/samples/")
                            .trim_start_matches("/samples")
                            .split('/')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect();

                        for (idx, part) in parts.iter().enumerate() {
                            ui.label("›");

                            // Build path up to this point
                            let path_to_here = if idx == 0 {
                                format!("/samples/{}", part)
                            } else {
                                format!("/samples/{}", parts[..=idx].join("/"))
                            };

                            if ui.button(part).clicked() {
                                self.current_folder = Some(PathBuf::from(path_to_here));
                            }
                        }
                    }
                });

                ui.add_space(theme.spacing.sm);

                // Category tabs - horizontally scrollable
                ui.horizontal(|ui| {
                    ui.add_space(theme.spacing.sm);
                    ui.colored_label(theme.on_surface_variant(), "Category:");
                    ui.add_space(theme.spacing.xs);
                });

                egui::ScrollArea::horizontal()
                    .id_salt("category_scroll")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(theme.spacing.sm);

                            for tab in BrowserTab::all() {
                                let is_active = self.active_tab == tab;
                                let chip_response = Chip::new(tab.label())
                                    .chip_type(ChipType::Assist)
                                    .size(ChipSize::Small)
                                    .selected(is_active)
                                    .show(ui);

                                if chip_response.clicked() {
                                    self.active_tab = tab;
                                    if old_tab != tab {
                                        tab_changed_this_frame = Some(tab);
                                    }
                                }
                                ui.add_space(theme.spacing.xs);
                            }
                        });
                    });

                ui.add_space(theme.spacing.sm);

                // Filter chips - horizontally scrollable
                let available_tags = self.collect_available_tags();
                if !available_tags.is_empty() {
                    ui.horizontal(|ui| {
                        ui.add_space(theme.spacing.sm);
                        ui.colored_label(theme.on_surface_variant(), "Filters:");
                        ui.add_space(theme.spacing.xs);
                    });

                    egui::ScrollArea::horizontal()
                        .id_salt("filters_scroll")
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(theme.spacing.sm);

                                for tag in &available_tags {
                                    let is_active = self.filter_tags.contains(tag);
                                    let chip_response = Chip::new(tag)
                                        .chip_type(ChipType::Filter)
                                        .size(ChipSize::Small)
                                        .selected(is_active)
                                        .show(ui);

                                    if chip_response.clicked() {
                                        if is_active {
                                            // Remove filter
                                            self.filter_tags.retain(|t| t != tag);
                                        } else {
                                            // Add filter
                                            self.filter_tags.push(tag.clone());
                                        }
                                        filters_changed = true;
                                    }

                                    ui.add_space(theme.spacing.xs);
                                }
                            });
                        });

                    ui.add_space(theme.spacing.sm);
                }

                // File list in scrollable area
                egui::ScrollArea::vertical()
                    .id_salt("browser_scroll")
                    .show(ui, |ui| {
                        ui.add_space(theme.spacing.xs);

                        // Filter items based on folder, search and tags
                        let search_query = self.search_query.to_lowercase();
                        let filtered_items: Vec<_> = self
                            .items
                            .iter()
                            .filter(|item| {
                                // Folder hierarchy filter - only show items in current folder
                                let in_current_folder =
                                    if let Some(ref current) = self.current_folder {
                                        // Get parent path of the item
                                        if let Some(parent) = item.path.parent() {
                                            parent == current.as_path()
                                        } else {
                                            false
                                        }
                                    } else {
                                        // At root - show only items in /samples/ (direct children)
                                        if let Some(parent) = item.path.parent() {
                                            parent == std::path::Path::new("/samples")
                                        } else {
                                            false
                                        }
                                    };

                                // Search filter
                                let matches_search = if search_query.is_empty() {
                                    true
                                } else {
                                    item.name.to_lowercase().contains(&search_query)
                                };

                                // Tag filter (if any filters active, item must have at least one matching tag)
                                let matches_tags = if self.filter_tags.is_empty() {
                                    true
                                } else {
                                    self.filter_tags
                                        .iter()
                                        .any(|filter_tag| item.tags.contains(filter_tag))
                                };

                                in_current_folder && matches_search && matches_tags
                            })
                            .cloned()
                            .collect();

                        // Show items
                        for item in &filtered_items {
                            if let Some((file, folder)) = self.show_item(ui, item) {
                                if let Some(f) = file {
                                    selected_file_this_frame = Some(f);
                                }
                                if let Some(d) = folder {
                                    selected_folder_this_frame = Some(d);
                                }
                            }
                            ui.add_space(theme.spacing.xs);
                        }
                    });
            });
        });

        BrowserResponse {
            response,
            selected_file: selected_file_this_frame,
            selected_folder: selected_folder_this_frame,
            tab_changed: tab_changed_this_frame,
            filters_changed: filters_changed || self.filter_tags != old_filters,
        }
    }

    /// Show a single browser item
    /// Returns (file_clicked, folder_clicked) if item was clicked
    fn show_item(
        &mut self,
        ui: &mut Ui,
        item: &BrowserItem,
    ) -> Option<(Option<PathBuf>, Option<PathBuf>)> {
        let theme = ui.ctx().armas_theme();
        let is_selected = self
            .selected_file
            .as_ref()
            .map(|p| p == &item.path)
            .unwrap_or(false);

        let mut result = None;

        ui.horizontal(|ui| {
            ui.add_space(theme.spacing.sm);

            let item_width = self.width - theme.spacing.sm * 2.0;

            // Item card
            let card = Card::new()
                .variant(if is_selected {
                    CardVariant::Outlined
                } else {
                    CardVariant::Filled
                })
                .width(item_width)
                .corner_radius(8.0)
                .inner_margin(theme.spacing.sm);

            let card_response = card.show(ui, &theme, |ui| {
                ui.horizontal(|ui| {
                    // Icon
                    let icon = if item.is_directory {
                        "📁"
                    } else if item.is_favorite {
                        "⭐"
                    } else {
                        "🎵"
                    };
                    ui.label(icon);

                    ui.add_space(theme.spacing.xs);

                    // Name and metadata
                    ui.vertical(|ui| {
                        ui.colored_label(theme.on_surface(), &item.name);

                        if let Some(ref metadata) = item.metadata {
                            // Build metadata string
                            let mut meta_parts = Vec::new();

                            if let Some(bpm) = metadata.bpm {
                                meta_parts.push(format!("{:.0} BPM", bpm));
                            }
                            if let Some(ref key) = metadata.key {
                                meta_parts.push(key.clone());
                            }
                            if let Some(sr) = metadata.sample_rate {
                                meta_parts.push(format!("{:.1}kHz", sr as f32 / 1000.0));
                            }

                            if !meta_parts.is_empty() {
                                ui.colored_label(
                                    theme.on_surface_variant(),
                                    meta_parts.join(" • "),
                                );
                            }
                        }
                    });
                });
            });

            // Handle selection
            if card_response.response.clicked() {
                if item.is_directory {
                    self.current_folder = Some(item.path.clone());
                    result = Some((None, Some(item.path.clone())));
                } else {
                    self.selected_file = Some(item.path.clone());
                    result = Some((Some(item.path.clone()), None));
                }
            }
        });

        result
    }
}

impl Default for Browser {
    fn default() -> Self {
        Self::new()
    }
}
