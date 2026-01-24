//! Waveform display component for sampler UIs
//!
//! Renders audio waveforms with interactive markers for sample editing.
//! Generic over sample data type for maximum flexibility.

use armas::theme::Theme;
use armas::ext::ArmasContextExt;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

/// Configuration for waveform display
#[derive(Debug, Clone)]
pub struct WaveformConfig {
    /// Pixels per second of audio
    pub pixels_per_second: f32,
    /// Height of waveform display in pixels
    pub height: f32,
    /// Peak color
    pub peak_color: Color32,
    /// RMS color (average level)
    pub rms_color: Color32,
    /// Grid lines enabled
    pub show_grid: bool,
    /// Grid interval in seconds
    pub grid_interval: f32,
    /// Enable frequency-based color coding (warm for low freqs, cool for high)
    pub color_by_frequency: bool,
    /// Enable spectrogram mode (heat map overlay)
    pub show_spectrogram: bool,
    /// Zoom level (1.0 = normal, 2.0 = 2x zoomed, 0.5 = zoomed out)
    pub zoom_level: f32,
}

impl Default for WaveformConfig {
    fn default() -> Self {
        Self {
            pixels_per_second: 100.0,
            height: 150.0,
            // Use primary color for better integration with Armas theme
            peak_color: Color32::from_rgb(100, 200, 255),
            rms_color: Color32::from_rgb(100, 200, 255).gamma_multiply(0.5),
            show_grid: false,            // Grid disabled by default (like professional DAWs)
            grid_interval: 1.0,
            color_by_frequency: false,  // Disabled by default for clarity
            show_spectrogram: false,    // Disabled by default
            zoom_level: 1.0,             // Normal zoom
        }
    }
}

/// Interactive waveform marker (sample boundary or loop point)
#[derive(Debug, Clone, Copy)]
pub enum MarkerType {
    SampleStart,
    SampleEnd,
    LoopStart,
    LoopEnd,
}

/// Response from waveform display interaction
#[derive(Debug, Clone)]
pub struct WaveformResponse {
    pub response: Response,
    pub marker_dragged: Option<(MarkerType, f64)>, // (marker type, new position in seconds)
    pub region_selected: Option<(f64, f64)>,        // (start, end) time range
    pub playhead_clicked: Option<f64>,               // clicked at time
}

/// Waveform display component - generic over sample data type
pub struct WaveformDisplay<'a, T> {
    sample_data: &'a [T],
    amplitude_fn: fn(&T) -> f32,
    duration: f64,
    sample_rate: u32,
    theme: &'a Theme,
    config: WaveformConfig,

    // Interactive state
    playhead_pos: Option<f64>,
    sample_start: f64,
    sample_end: f64,
    loop_start: f64,
    loop_end: f64,
    loop_enabled: bool,
}

impl<'a, T> WaveformDisplay<'a, T> {
    /// Create a new waveform display
    pub fn new(
        sample_data: &'a [T],
        amplitude_fn: fn(&T) -> f32,
        duration: f64,
        sample_rate: u32,
        theme: &'a Theme,
    ) -> Self {
        Self {
            sample_data,
            amplitude_fn,
            duration,
            sample_rate,
            theme,
            config: WaveformConfig::default(),
            playhead_pos: None,
            sample_start: 0.0,
            sample_end: duration,
            loop_start: 0.0,
            loop_end: duration,
            loop_enabled: false,
        }
    }

    /// Set configuration
    pub fn config(mut self, config: WaveformConfig) -> Self {
        self.config = config;
        self
    }

    /// Set playhead position in seconds
    pub fn playhead(mut self, pos: f64) -> Self {
        self.playhead_pos = Some(pos);
        self
    }

    /// Set sample boundaries
    pub fn sample_bounds(mut self, start: f64, end: f64) -> Self {
        self.sample_start = start;
        self.sample_end = end;
        self
    }

    /// Set loop region
    pub fn loop_region(mut self, start: f64, end: f64) -> Self {
        self.loop_start = start;
        self.loop_end = end;
        self
    }

    /// Enable/disable looping
    pub fn loop_enabled(mut self, enabled: bool) -> Self {
        self.loop_enabled = enabled;
        self
    }

    /// Enable/disable frequency-based color coding
    pub fn color_by_frequency(mut self, enabled: bool) -> Self {
        self.config.color_by_frequency = enabled;
        self
    }

    /// Enable/disable spectrogram mode
    pub fn show_spectrogram(mut self, enabled: bool) -> Self {
        self.config.show_spectrogram = enabled;
        self
    }

    /// Set zoom level (1.0 = normal, 2.0 = 2x, 0.5 = zoomed out)
    pub fn zoom(mut self, level: f32) -> Self {
        self.config.zoom_level = level.max(0.1).min(10.0);
        self
    }

    /// Show the waveform display
    pub fn show(mut self, ui: &mut Ui, size: Vec2) -> WaveformResponse {
        // Update config colors from theme if using defaults
        let theme = ui.ctx().armas_theme();

        // Check if colors are still at default hardcoded values
        let is_default_peak = self.config.peak_color == Color32::from_rgb(100, 200, 255);
        if is_default_peak {
            self.config.peak_color = theme.primary();
            self.config.rms_color = theme.primary().gamma_multiply(0.5);
        }

        let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());

        if ui.is_rect_visible(rect) {
            self.render(ui.painter(), rect);
        }

        WaveformResponse {
            response,
            marker_dragged: None,
            region_selected: None,
            playhead_clicked: None,
        }
    }

    /// Get frequency-based color (warm for low freq, cool for high freq)
    /// freq_estimate: 0.0 (low bass) to 1.0 (high treble)
    fn get_frequency_color(&self, freq_estimate: f32, base_color: Color32) -> Color32 {
        if !self.config.color_by_frequency {
            return base_color;
        }

        // Interpolate between warm (red/orange) for low freq and cool (blue/cyan) for high freq
        let warm = Color32::from_rgb(255, 140, 60);  // Orange (low freq)
        let cool = Color32::from_rgb(100, 200, 255); // Cyan (high freq)

        let f = freq_estimate.clamp(0.0, 1.0);
        Color32::from_rgba_unmultiplied(
            ((warm.r() as f32) * (1.0 - f) + (cool.r() as f32) * f) as u8,
            ((warm.g() as f32) * (1.0 - f) + (cool.g() as f32) * f) as u8,
            ((warm.b() as f32) * (1.0 - f) + (cool.b() as f32) * f) as u8,
            base_color.a(),
        )
    }

    /// Render the waveform display
    fn render(&self, painter: &egui::Painter, rect: Rect) {
        // Dark, minimal background like professional DAWs
        // Keep background dark and neutral to let waveform bars stand out

        // Main background with rounded corners for aesthetic frame
        let corner_radius = 8.0;
        painter.rect_filled(
            rect,
            corner_radius,
            Color32::from_rgb(30, 30, 35)  // Dark neutral background
        );

        // Add subtle frame stroke for visual boundary
        painter.rect_stroke(
            rect,
            corner_radius,
            Stroke::new(1.5, Color32::from_rgb(80, 80, 90)),
            egui::epaint::StrokeKind::Inside,
        );

        // Draw grid if enabled
        if self.config.show_grid {
            self.draw_grid(painter, rect);
        }

        // Draw waveform
        self.draw_waveform(painter, rect);

        // Draw loop region if enabled
        if self.loop_enabled {
            self.draw_loop_region(painter, rect);
        }

        // Draw sample boundaries
        self.draw_markers(painter, rect);

        // Draw playhead if set
        if let Some(pos) = self.playhead_pos {
            self.draw_playhead(painter, rect, pos);
        }
    }

    /// Draw grid lines with sophisticated styling
    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let mut time = 0.0f64;
        let grid_interval = self.config.grid_interval as f64;

        while time <= self.duration {
            let x = rect.min.x + ((time / self.duration) * rect.width() as f64) as f32;

            if x >= rect.min.x && x <= rect.max.x {
                // Primary grid lines (major intervals)
                if (time % (grid_interval * 4.0)).abs() < 0.01 {
                    painter.line_segment(
                        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                        Stroke::new(0.75, self.theme.primary().gamma_multiply(0.15)),
                    );
                } else {
                    // Secondary grid lines (minor intervals) - more subtle
                    painter.line_segment(
                        [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                        Stroke::new(0.4, self.theme.border().gamma_multiply(0.15)),
                    );
                }
            }

            time += grid_interval;
        }
    }

    /// Draw the waveform using selected render mode
    fn draw_waveform(&self, painter: &egui::Painter, rect: Rect) {
        let effective_pps = self.config.pixels_per_second * self.config.zoom_level;
        let samples_per_pixel = (self.sample_data.len() as f64 / self.duration / effective_pps as f64).max(1.0);
        let center_y = rect.center().y;

        // Collect all amplitude data
        let mut amplitudes = Vec::new();
        for x_pixel in 0..(rect.width() as i32) {
            let time = (x_pixel as f64) / (self.config.pixels_per_second as f64);
            let sample_index = (time * self.sample_rate as f64) as usize;
            let window_size = (samples_per_pixel as usize).max(1);
            let window_start = sample_index.saturating_sub(window_size / 2);
            let window_end = (sample_index + window_size / 2).min(self.sample_data.len());

            let mut peak = 0.0f32;
            let mut rms = 0.0f32;
            let mut count = 0;

            for i in window_start..window_end {
                if i < self.sample_data.len() {
                    let amp = (self.amplitude_fn)(&self.sample_data[i]).abs();
                    peak = peak.max(amp);
                    rms += amp * amp;
                    count += 1;
                }
            }

            if count > 0 {
                rms = (rms / count as f32).sqrt();
            }

            amplitudes.push((peak.clamp(0.0, 1.0), rms.clamp(0.0, 1.0)));
        }

        // Draw bars waveform
        self.draw_bars_waveform(painter, rect, &amplitudes, center_y);

        // Center line as reference
        painter.line_segment(
            [Pos2::new(rect.min.x, center_y), Pos2::new(rect.max.x, center_y)],
            Stroke::new(0.5, self.theme.border().gamma_multiply(0.2)),
        );
    }

    /// Draw a smooth tessellated waveform envelope
    fn draw_tessellated_waveform(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        amplitudes: &[(f32, f32)],
        center_y: f32,
        height_scale: f32,
        color: Color32,
        alpha: u8,
    ) {
        if amplitudes.is_empty() {
            return;
        }

        // Build smooth top and bottom paths using Catmull-Rom spline interpolation
        let mut top_points = Vec::new();
        let mut bottom_points = Vec::new();

        // Collect amplitude envelope points
        for (x_pixel, amp_data) in amplitudes.iter().enumerate() {
            // Use peak data (first element) for consistent sizing with bars mode
            let amplitude = amp_data.0;
            let x = rect.min.x + x_pixel as f32;
            let height = (amplitude * rect.height() * height_scale).min(rect.height() / 2.0);

            top_points.push(Pos2::new(x, center_y - height));
            bottom_points.push(Pos2::new(x, center_y + height));
        }

        if top_points.len() < 2 {
            return;
        }

        let fill_color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

        // No filled polygon - draw just the outlines for consistent appearance with bars mode

        // Draw smooth outline strokes for the top curve
        for i in 0..top_points.len().saturating_sub(1) {
            let p1 = top_points[i];
            let p2 = top_points[i + 1];

            // Draw multiple segments for smooth appearance
            for segment in 0..3 {
                let t1 = segment as f32 / 3.0;
                let t2 = (segment + 1) as f32 / 3.0;

                let y1 = p1.y + (p2.y - p1.y) * (t1 * t1 * (3.0 - 2.0 * t1));
                let y2 = p1.y + (p2.y - p1.y) * (t2 * t2 * (3.0 - 2.0 * t2));

                let start_x = p1.x + (p2.x - p1.x) * t1;
                let end_x = p1.x + (p2.x - p1.x) * t2;

                painter.line_segment(
                    [Pos2::new(start_x, y1), Pos2::new(end_x, y2)],
                    Stroke::new(1.2, color),
                );
            }
        }

        // Draw smooth outline strokes for the bottom curve
        for i in (0..bottom_points.len().saturating_sub(1)).rev() {
            let p1 = bottom_points[i];
            let p2 = bottom_points[i + 1];

            for segment in 0..3 {
                let t1 = segment as f32 / 3.0;
                let t2 = (segment + 1) as f32 / 3.0;

                let y1 = p1.y + (p2.y - p1.y) * (t1 * t1 * (3.0 - 2.0 * t1));
                let y2 = p1.y + (p2.y - p1.y) * (t2 * t2 * (3.0 - 2.0 * t2));

                let start_x = p1.x + (p2.x - p1.x) * t1;
                let end_x = p1.x + (p2.x - p1.x) * t2;

                painter.line_segment(
                    [Pos2::new(start_x, y1), Pos2::new(end_x, y2)],
                    Stroke::new(1.2, color),
                );
            }
        }
    }

    /// Draw waveform as vertical bars (classic DAW style)
    fn draw_bars_waveform(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        amplitudes: &[(f32, f32)],
        center_y: f32,
    ) {
        if amplitudes.is_empty() {
            return;
        }

        let bar_width = (rect.width() / amplitudes.len() as f32).max(0.5);
        let bar_spacing = (bar_width * 0.15).max(0.5); // Small gap between bars
        let effective_bar_width = bar_width - bar_spacing;

        // Calculate playhead position in pixels if present
        let playhead_x = self.playhead_pos.map(|pos| {
            rect.min.x + ((pos / self.duration) * rect.width() as f64) as f32
        });

        // Draw RMS bars (background/lighter)
        for (x_pixel, (_, rms)) in amplitudes.iter().enumerate() {
            let x = rect.min.x + x_pixel as f32 * bar_width + bar_spacing / 2.0;
            let height_rms = (rms * rect.height() * 0.35).min(rect.height() / 2.0);

            // Determine if this bar is past the playhead (dimmed region)
            let is_past_playhead = playhead_x.map_or(false, |px| x > px);
            let rms_color = if is_past_playhead {
                self.config.rms_color.gamma_multiply(0.3) // Dim to 30%
            } else {
                self.config.rms_color
            };

            let bar_rect_top = Rect::from_min_max(
                Pos2::new(x, center_y - height_rms),
                Pos2::new(x + effective_bar_width, center_y),
            );
            let bar_rect_bottom = Rect::from_min_max(
                Pos2::new(x, center_y),
                Pos2::new(x + effective_bar_width, center_y + height_rms),
            );

            painter.rect_filled(bar_rect_top, 2.0, rms_color);
            painter.rect_filled(bar_rect_bottom, 2.0, rms_color);
        }

        // Draw peak bars (foreground/brighter)
        for (x_pixel, (peak, _)) in amplitudes.iter().enumerate() {
            let x = rect.min.x + x_pixel as f32 * bar_width + bar_spacing / 2.0;
            let height_peak = (peak * rect.height() * 0.40).min(rect.height() / 2.0);

            // Determine if this bar is past the playhead (dimmed region)
            let is_past_playhead = playhead_x.map_or(false, |px| x > px);
            let peak_color = if is_past_playhead {
                self.config.peak_color.gamma_multiply(0.3) // Dim to 30%
            } else {
                self.config.peak_color
            };

            let bar_rect_top = Rect::from_min_max(
                Pos2::new(x, center_y - height_peak),
                Pos2::new(x + effective_bar_width, center_y),
            );
            let bar_rect_bottom = Rect::from_min_max(
                Pos2::new(x, center_y),
                Pos2::new(x + effective_bar_width, center_y + height_peak),
            );

            // Fill with semi-transparent color (respecting dimming)
            painter.rect_filled(
                bar_rect_top,
                2.0,
                Color32::from_rgba_unmultiplied(
                    peak_color.r(),
                    peak_color.g(),
                    peak_color.b(),
                    200,
                ),
            );
            painter.rect_filled(
                bar_rect_bottom,
                2.0,
                Color32::from_rgba_unmultiplied(
                    peak_color.r(),
                    peak_color.g(),
                    peak_color.b(),
                    200,
                ),
            );

            // Optional: Add subtle stroke for definition
            painter.rect_stroke(
                bar_rect_top,
                2.0,
                Stroke::new(0.5, self.config.peak_color.gamma_multiply(0.5)),
                egui::epaint::StrokeKind::Inside,
            );
            painter.rect_stroke(
                bar_rect_bottom,
                2.0,
                Stroke::new(0.5, self.config.peak_color.gamma_multiply(0.5)),
                egui::epaint::StrokeKind::Inside,
            );
        }
    }

    /// Draw loop region highlighting
    fn draw_loop_region(&self, painter: &egui::Painter, rect: Rect) {
        let loop_start_x = rect.min.x + ((self.loop_start / self.duration) * rect.width() as f64) as f32;
        let loop_end_x = rect.min.x + ((self.loop_end / self.duration) * rect.width() as f64) as f32;

        if loop_start_x >= rect.min.x && loop_start_x <= rect.max.x && loop_end_x >= rect.min.x && loop_end_x <= rect.max.x {
            let loop_rect = Rect::from_x_y_ranges(loop_start_x..=loop_end_x, rect.min.y..=rect.max.y);
            let loop_color = self.theme.primary().gamma_multiply(0.1);

            painter.rect_filled(loop_rect, 0.0, loop_color);
            painter.rect_stroke(
                loop_rect,
                egui::epaint::CornerRadius::ZERO,
                Stroke::new(1.0, self.theme.primary().gamma_multiply(0.3)),
                egui::epaint::StrokeKind::Inside,
            );
        }
    }

    /// Draw sample boundary markers
    fn draw_markers(&self, painter: &egui::Painter, rect: Rect) {
        const MARKER_WIDTH: f32 = 8.0;
        const MARKER_HEIGHT: f32 = 15.0;
        const TOLERANCE: f64 = 0.001; // Small tolerance for floating point comparison

        // Only draw sample start marker if it's not at the very beginning (has been trimmed)
        if self.sample_start > TOLERANCE {
            let start_x = rect.min.x + ((self.sample_start / self.duration) * rect.width() as f64) as f32;
            if start_x >= rect.min.x && start_x <= rect.max.x {
                self.draw_marker(painter, start_x, rect.top(), self.theme.chart_2(), MARKER_WIDTH, MARKER_HEIGHT);
            }
        }

        // Only draw sample end marker if it's not at the very end (has been trimmed)
        if self.sample_end < (self.duration - TOLERANCE) {
            let end_x = rect.min.x + ((self.sample_end / self.duration) * rect.width() as f64) as f32;
            if end_x >= rect.min.x && end_x <= rect.max.x {
                self.draw_marker(painter, end_x, rect.bottom() - MARKER_HEIGHT, self.theme.destructive(), MARKER_WIDTH, MARKER_HEIGHT);
            }
        }
    }

    /// Draw a single marker
    fn draw_marker(&self, painter: &egui::Painter, x: f32, y: f32, color: Color32, width: f32, height: f32) {
        // Draw glow effect behind marker
        for i in 0..3 {
            let offset = (i as f32 + 1.0) * 2.0;
            let alpha = ((1.0 - i as f32 / 3.0) * 20.0) as u8;
            let glow_color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

            let glow_points = vec![
                Pos2::new(x - (width / 2.0 + offset), y - offset),
                Pos2::new(x + (width / 2.0 + offset), y - offset),
                Pos2::new(x, y + height + offset),
            ];
            painter.add(egui::Shape::convex_polygon(
                glow_points,
                Color32::TRANSPARENT,
                Stroke::new(0.5, glow_color),
            ));
        }

        // Draw triangle marker
        let points = vec![
            Pos2::new(x - width / 2.0, y),
            Pos2::new(x + width / 2.0, y),
            Pos2::new(x, y + height),
        ];

        painter.add(egui::Shape::convex_polygon(
            points,
            color,
            Stroke::new(1.0, Color32::WHITE.gamma_multiply(0.6)),
        ));

        // Draw vertical line with subtle gradient effect
        painter.line_segment(
            [Pos2::new(x, y + height), Pos2::new(x, y + height + 100.0)],
            Stroke::new(1.0, color.gamma_multiply(0.4)),
        );
    }

    /// Draw playhead indicator (simple Studio One style - just a vertical line)
    fn draw_playhead(&self, painter: &egui::Painter, rect: Rect, pos: f64) {
        let x = rect.min.x + ((pos / self.duration) * rect.width() as f64) as f32;

        if x >= rect.min.x && x <= rect.max.x {
            // Simple vertical line like Studio One
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.5, self.theme.foreground()),
            );
        }
    }
}
