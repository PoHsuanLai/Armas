//! Browser/File Manager Component
//!
//! File browser for audio files, plugins, presets, and samples.
//! Features search, filtering, folder navigation, and drag-and-drop support.
//!
//! ## Responsive Design
//!
//! The browser automatically adapts to available space:
//!
//! | Width | Mode | Description |
//! |-------|------|-------------|
//! | < 120px | IconOnly | Just icons, tooltips for details |
//! | 120-200px | Compact | Icons + truncated names, no metadata |
//! | 200-280px | Normal | Full names, category chips, basic metadata |
//! | > 280px | Expanded | Full metadata inline (BPM, key, duration) |

use crate::components::basic::{Chip, ChipSize, ChipType};
use crate::components::cards::{Card, CardVariant};
use crate::ext::ArmasContextExt;
use egui::{Response, Ui, Vec2};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Browser display mode based on available width
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserMode {
    /// < 120px - Icons only, tooltips for names
    IconOnly,
    /// 120-200px - Icons + truncated names, minimal UI
    Compact,
    /// 200-280px - Normal layout with chips and basic info
    Normal,
    /// > 280px - Full metadata displayed inline
    Expanded,
}

impl BrowserMode {
    /// Determine mode from width
    pub fn from_width(width: f32) -> Self {
        match width {
            w if w < 120.0 => Self::IconOnly,
            w if w < 200.0 => Self::Compact,
            w if w < 280.0 => Self::Normal,
            _ => Self::Expanded,
        }
    }

    /// Get search box height for this mode
    pub fn search_height(&self) -> f32 {
        match self {
            Self::IconOnly => 0.0, // Hide search in icon-only mode
            Self::Compact => 28.0,
            Self::Normal | Self::Expanded => 32.0,
        }
    }

    /// Get item row height for this mode
    pub fn item_height(&self) -> f32 {
        match self {
            Self::IconOnly => 32.0,
            Self::Compact => 36.0,
            Self::Normal => 48.0,
            Self::Expanded => 56.0,
        }
    }

    /// Whether to show category tabs
    pub fn show_category_tabs(&self) -> bool {
        matches!(self, Self::Normal | Self::Expanded)
    }

    /// Whether to show filter chips
    pub fn show_filter_chips(&self) -> bool {
        matches!(self, Self::Normal | Self::Expanded)
    }

    /// Whether to show breadcrumb text (vs just icons)
    pub fn show_breadcrumb_text(&self) -> bool {
        matches!(self, Self::Normal | Self::Expanded)
    }

    /// Whether to show file metadata inline
    pub fn show_metadata_inline(&self) -> bool {
        matches!(self, Self::Expanded)
    }

    /// Max characters for filename before truncation
    pub fn max_filename_chars(&self) -> usize {
        match self {
            Self::IconOnly => 0,
            Self::Compact => 12,
            Self::Normal => 20,
            Self::Expanded => 40,
        }
    }
}

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

    /// Get tab icon
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Audio => "🎵",
            Self::Plugins => "🔌",
            Self::Presets => "🎛️",
            Self::Loops => "🔁",
            Self::Samples => "🥁",
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
/// ## Responsive Design
///
/// Use `fill_available()` to make the browser adapt to its container size.
/// The browser will automatically switch between display modes based on width.
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
/// // In your UI code - fills available space automatically:
/// let response = self.browser.show(ui);
///
/// // Or with fixed dimensions:
/// let response = Browser::new()
///     .width(300.0)
///     .height(600.0)
///     .show(ui);
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
    /// Browser width (0.0 = fill available)
    width: f32,
    /// Browser height (0.0 = fill available)
    height: f32,
    /// Available items (can be set externally or use mock data)
    items: Vec<BrowserItem>,
    /// Force a specific display mode (None = auto-detect from width)
    forced_mode: Option<BrowserMode>,
}

impl Browser {
    /// Create a new browser that fills available space
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            active_tab: BrowserTab::Audio,
            current_folder: None,
            selected_file: None,
            filter_tags: Vec::new(),
            sort_mode: SortMode::Name,
            width: 0.0,  // 0 = fill available
            height: 0.0, // 0 = fill available
            items: Self::mock_items(),
            forced_mode: None,
        }
    }

    /// Set fixed browser width (default: fills available space)
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set fixed browser height (default: fills available space)
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Force a specific display mode regardless of width
    pub fn mode(mut self, mode: BrowserMode) -> Self {
        self.forced_mode = Some(mode);
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

        // Determine actual dimensions
        let available = ui.available_size();
        let width = if self.width > 0.0 { self.width } else { available.x };
        let height = if self.height > 0.0 { self.height } else { available.y };

        // Determine display mode
        let mode = self.forced_mode.unwrap_or_else(|| BrowserMode::from_width(width));

        // Track changes this frame
        let mut selected_file_this_frame = None;
        let mut selected_folder_this_frame = None;
        let mut tab_changed_this_frame = None;
        let old_tab = self.active_tab;
        let old_filters = self.filter_tags.clone();
        let mut filters_changed = false;

        // Main container
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());

        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.vertical(|ui| {
                // Search box (hidden in IconOnly mode)
                if mode != BrowserMode::IconOnly {
                    ui.add_space(theme.spacing.xs);
                    ui.horizontal(|ui| {
                        ui.add_space(theme.spacing.xs);

                        let search_width = width - theme.spacing.xs * 2.0;
                        let hint = if mode == BrowserMode::Compact { "🔍" } else { "🔍 Search..." };
                        ui.add_sized(
                            Vec2::new(search_width, mode.search_height()),
                            egui::TextEdit::singleline(&mut self.search_query)
                                .hint_text(hint)
                                .frame(true),
                        );
                    });
                    ui.add_space(theme.spacing.xs);
                }

                // Breadcrumb navigation
                self.show_breadcrumb(ui, width, mode, &theme);

                // Category tabs (Normal and Expanded modes)
                if mode.show_category_tabs() {
                    tab_changed_this_frame = self.show_category_tabs_ui(ui, mode, &theme);
                }

                // Filter chips (Normal and Expanded modes)
                if mode.show_filter_chips() {
                    if self.show_filter_chips_ui(ui, mode, &theme) {
                        filters_changed = true;
                    }
                }

                // File list
                egui::ScrollArea::vertical()
                    .id_salt("browser_scroll")
                    .show(ui, |ui| {
                        ui.add_space(theme.spacing.xs);

                        // Filter items
                        let filtered_items = self.filter_items();

                        // Show items
                        for item in &filtered_items {
                            if let Some((file, folder)) = self.show_item(ui, item, width, mode, &theme) {
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

    /// Show breadcrumb navigation
    fn show_breadcrumb(&mut self, ui: &mut Ui, width: f32, mode: BrowserMode, theme: &crate::Theme) {
        ui.horizontal(|ui| {
            ui.add_space(theme.spacing.xs);

            // Back button
            if let Some(ref current) = self.current_folder {
                let back_text = if mode.show_breadcrumb_text() { "⬅ Back" } else { "⬅" };
                if ui.button(back_text).clicked() {
                    if let Some(parent) = current.parent() {
                        if parent == std::path::Path::new("/samples") {
                            self.current_folder = None;
                        } else {
                            self.current_folder = Some(parent.to_path_buf());
                        }
                    } else {
                        self.current_folder = None;
                    }
                }
                ui.add_space(theme.spacing.xs);
            }

            // Root button
            let root_text = if mode.show_breadcrumb_text() { "📁 Samples" } else { "📁" };
            let root_btn = ui.button(root_text);
            if root_btn.clicked() {
                self.current_folder = None;
            } else if mode == BrowserMode::IconOnly || mode == BrowserMode::Compact {
                root_btn.on_hover_text("Samples (root)");
            }

            // Breadcrumb path (only in modes that show text)
            if mode.show_breadcrumb_text() {
                if let Some(ref current) = self.current_folder {
                    let path_str = current.to_string_lossy().to_string();
                    let parts: Vec<String> = path_str
                        .trim_start_matches("/samples/")
                        .trim_start_matches("/samples")
                        .split('/')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect();

                    // Calculate available width for breadcrumbs
                    let used_width = if self.current_folder.is_some() { 120.0 } else { 80.0 };
                    let available_crumb_width = width - used_width;
                    let max_parts = (available_crumb_width / 60.0).max(1.0) as usize;

                    // Show ellipsis if too many parts
                    let show_parts = if parts.len() > max_parts {
                        ui.label("›");
                        ui.label("...");
                        &parts[parts.len() - max_parts..]
                    } else {
                        &parts[..]
                    };

                    for (idx, part) in show_parts.iter().enumerate() {
                        ui.label("›");

                        let part_idx = if parts.len() > max_parts {
                            parts.len() - max_parts + idx
                        } else {
                            idx
                        };

                        let path_to_here = format!("/samples/{}", parts[..=part_idx].join("/"));

                        if ui.button(part).clicked() {
                            self.current_folder = Some(PathBuf::from(path_to_here));
                        }
                    }
                }
            }
        });

        ui.add_space(theme.spacing.xs);
    }

    /// Show category tabs, returns Some(tab) if tab changed
    fn show_category_tabs_ui(&mut self, ui: &mut Ui, mode: BrowserMode, theme: &crate::Theme) -> Option<BrowserTab> {
        let mut tab_changed = None;
        let old_tab = self.active_tab;

        // Category label (only in expanded mode)
        if mode == BrowserMode::Expanded {
            ui.horizontal(|ui| {
                ui.add_space(theme.spacing.xs);
                ui.colored_label(theme.on_surface_variant(), "Category:");
            });
        }

        egui::ScrollArea::horizontal()
            .id_salt("category_scroll")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(theme.spacing.xs);

                    for tab in BrowserTab::all() {
                        let is_active = self.active_tab == tab;

                        // Use icons in compact/normal, full labels in expanded
                        let label = if mode == BrowserMode::Expanded {
                            format!("{} {}", tab.icon(), tab.label())
                        } else {
                            tab.icon().to_string()
                        };

                        let chip_response = Chip::new(&label)
                            .chip_type(ChipType::Assist)
                            .size(ChipSize::Small)
                            .selected(is_active)
                            .show(ui);

                        if chip_response.clicked() {
                            self.active_tab = tab;
                            if old_tab != tab {
                                tab_changed = Some(tab);
                            }
                        } else if mode != BrowserMode::Expanded {
                            // Tooltip for icon-only (only if not clicked)
                            chip_response.response.on_hover_text(tab.label());
                        }
                        ui.add_space(theme.spacing.xs);
                    }
                });
            });

        ui.add_space(theme.spacing.xs);
        tab_changed
    }

    /// Show filter chips, returns true if filters changed
    fn show_filter_chips_ui(&mut self, ui: &mut Ui, mode: BrowserMode, theme: &crate::Theme) -> bool {
        let available_tags = self.collect_available_tags();
        if available_tags.is_empty() {
            return false;
        }

        let mut changed = false;

        // Filter label (only in expanded mode)
        if mode == BrowserMode::Expanded {
            ui.horizontal(|ui| {
                ui.add_space(theme.spacing.xs);
                ui.colored_label(theme.on_surface_variant(), "Filters:");
                if !self.filter_tags.is_empty() {
                    ui.colored_label(theme.primary(), format!("({})", self.filter_tags.len()));
                }
            });
        }

        egui::ScrollArea::horizontal()
            .id_salt("filters_scroll")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(theme.spacing.xs);

                    for tag in &available_tags {
                        let is_active = self.filter_tags.contains(tag);

                        // Truncate tag in normal mode
                        let display_tag = if mode == BrowserMode::Normal && tag.len() > 8 {
                            format!("{}…", &tag[..7])
                        } else {
                            tag.clone()
                        };

                        let chip_response = Chip::new(&display_tag)
                            .chip_type(ChipType::Filter)
                            .size(ChipSize::Small)
                            .selected(is_active)
                            .show(ui);

                        if chip_response.clicked() {
                            if is_active {
                                self.filter_tags.retain(|t| t != tag);
                            } else {
                                self.filter_tags.push(tag.clone());
                            }
                            changed = true;
                        } else if display_tag != *tag {
                            // Show full tag on hover if truncated (only if not clicked)
                            chip_response.response.on_hover_text(tag);
                        }

                        ui.add_space(theme.spacing.xs);
                    }
                });
            });

        ui.add_space(theme.spacing.xs);
        changed
    }

    /// Filter items based on current folder, search, and tags
    fn filter_items(&self) -> Vec<BrowserItem> {
        let search_query = self.search_query.to_lowercase();

        self.items
            .iter()
            .filter(|item| {
                // Folder hierarchy filter
                let in_current_folder = if let Some(ref current) = self.current_folder {
                    item.path.parent().map(|p| p == current.as_path()).unwrap_or(false)
                } else {
                    item.path.parent().map(|p| p == std::path::Path::new("/samples")).unwrap_or(false)
                };

                // Search filter
                let matches_search = search_query.is_empty()
                    || item.name.to_lowercase().contains(&search_query);

                // Tag filter
                let matches_tags = self.filter_tags.is_empty()
                    || self.filter_tags.iter().any(|t| item.tags.contains(t));

                in_current_folder && matches_search && matches_tags
            })
            .cloned()
            .collect()
    }

    /// Show a single browser item
    fn show_item(
        &mut self,
        ui: &mut Ui,
        item: &BrowserItem,
        width: f32,
        mode: BrowserMode,
        theme: &crate::Theme,
    ) -> Option<(Option<PathBuf>, Option<PathBuf>)> {
        let is_selected = self.selected_file.as_ref().map(|p| p == &item.path).unwrap_or(false);
        let mut result = None;

        ui.horizontal(|ui| {
            ui.add_space(theme.spacing.xs);

            let item_width = width - theme.spacing.xs * 2.0;

            // Icon
            let icon = if item.is_directory {
                "📁"
            } else if item.is_favorite {
                "⭐"
            } else {
                "🎵"
            };

            match mode {
                BrowserMode::IconOnly => {
                    // Just an icon button
                    let btn = ui.add_sized(
                        Vec2::new(item_width, mode.item_height()),
                        egui::Button::new(icon),
                    );
                    if btn.clicked() {
                        result = Some(self.handle_item_click(item));
                    } else {
                        btn.on_hover_text(&item.name);
                    }
                }
                BrowserMode::Compact => {
                    // Icon + truncated name in a simple row
                    let card = Card::new()
                        .variant(if is_selected { CardVariant::Outlined } else { CardVariant::Filled })
                        .width(item_width)
                        .corner_radius(6.0)
                        .inner_margin(theme.spacing.xs);

                    let card_response = card.show(ui, theme, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(icon);
                            ui.add_space(theme.spacing.xs);

                            let display_name = truncate_string(&item.name, mode.max_filename_chars());
                            let label = ui.colored_label(theme.on_surface(), &display_name);
                            if display_name != item.name {
                                label.on_hover_text(&item.name);
                            }
                        });
                    });

                    if card_response.response.clicked() {
                        result = Some(self.handle_item_click(item));
                    }
                }
                BrowserMode::Normal => {
                    // Icon + name, metadata on hover
                    let card = Card::new()
                        .variant(if is_selected { CardVariant::Outlined } else { CardVariant::Filled })
                        .width(item_width)
                        .corner_radius(8.0)
                        .inner_margin(theme.spacing.sm);

                    let card_response = card.show(ui, theme, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(icon);
                            ui.add_space(theme.spacing.xs);

                            let display_name = truncate_string(&item.name, mode.max_filename_chars());
                            ui.colored_label(theme.on_surface(), &display_name);
                        });
                    });

                    if card_response.response.clicked() {
                        result = Some(self.handle_item_click(item));
                    } else if let Some(ref metadata) = item.metadata {
                        // Tooltip with metadata (only if not clicked)
                        let tooltip = build_metadata_tooltip(&item.name, metadata);
                        card_response.response.on_hover_text(tooltip);
                    } else if item.name.len() > mode.max_filename_chars() {
                        card_response.response.on_hover_text(&item.name);
                    }
                }
                BrowserMode::Expanded => {
                    // Full layout with inline metadata
                    let card = Card::new()
                        .variant(if is_selected { CardVariant::Outlined } else { CardVariant::Filled })
                        .width(item_width)
                        .corner_radius(8.0)
                        .inner_margin(theme.spacing.sm);

                    let card_response = card.show(ui, theme, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(icon);
                            ui.add_space(theme.spacing.xs);

                            ui.vertical(|ui| {
                                ui.colored_label(theme.on_surface(), &item.name);

                                if let Some(ref metadata) = item.metadata {
                                    let meta_str = build_metadata_string(metadata);
                                    if !meta_str.is_empty() {
                                        ui.colored_label(theme.on_surface_variant(), meta_str);
                                    }
                                }
                            });
                        });
                    });

                    if card_response.response.clicked() {
                        result = Some(self.handle_item_click(item));
                    }
                }
            }
        });

        result
    }

    /// Handle item click - navigate folder or select file
    fn handle_item_click(&mut self, item: &BrowserItem) -> (Option<PathBuf>, Option<PathBuf>) {
        if item.is_directory {
            self.current_folder = Some(item.path.clone());
            (None, Some(item.path.clone()))
        } else {
            self.selected_file = Some(item.path.clone());
            (Some(item.path.clone()), None)
        }
    }
}

impl Default for Browser {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Truncate a string to max_chars, adding ellipsis if needed
fn truncate_string(s: &str, max_chars: usize) -> String {
    if max_chars == 0 || s.len() <= max_chars {
        s.to_string()
    } else if max_chars <= 3 {
        "…".to_string()
    } else {
        format!("{}…", &s[..max_chars - 1])
    }
}

/// Build metadata string for display (BPM • Key • Duration)
fn build_metadata_string(metadata: &FileMetadata) -> String {
    let mut parts = Vec::new();

    if let Some(bpm) = metadata.bpm {
        parts.push(format!("{:.0} BPM", bpm));
    }
    if let Some(ref key) = metadata.key {
        parts.push(key.clone());
    }
    if let Some(duration) = metadata.duration {
        parts.push(format!("{:.1}s", duration));
    }

    parts.join(" • ")
}

/// Build tooltip text with full metadata
fn build_metadata_tooltip(name: &str, metadata: &FileMetadata) -> String {
    let mut lines = vec![name.to_string()];

    if let Some(bpm) = metadata.bpm {
        lines.push(format!("BPM: {:.0}", bpm));
    }
    if let Some(ref key) = metadata.key {
        lines.push(format!("Key: {}", key));
    }
    if let Some(duration) = metadata.duration {
        lines.push(format!("Duration: {:.2}s", duration));
    }
    if let Some(sr) = metadata.sample_rate {
        lines.push(format!("Sample Rate: {:.1}kHz", sr as f32 / 1000.0));
    }

    lines.join("\n")
}
