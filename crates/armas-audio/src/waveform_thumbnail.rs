//! Audio Thumbnail Data Structures
//!
//! Multi-resolution waveform representation for efficient display of large audio files.
//! Inspired by JUCE's AudioThumbnail system.
//!
//! # Overview
//!
//! An `AudioThumbnail` stores pre-computed peak data at multiple resolutions (mipmap-style).
//! This allows efficient rendering at any zoom level by selecting the appropriate
//! level of detail.
//!
//! # Example
//!
//! ```ignore
//! // Generate a thumbnail from audio samples
//! let thumbnail = ThumbnailGenerator::new()
//!     .base_resolution(64)
//!     .num_levels(5)
//!     .generate(&samples, 44100, 2, |s| *s);
//!
//! // Get peaks for rendering at a specific zoom level
//! let peaks = thumbnail.get_peaks_for_width(1000, 0.0, 10.0);
//! ```

use serde::{Deserialize, Serialize};

/// A single level in the thumbnail mipmap hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailLevel {
    /// Peak data per channel: Vec of (min, max) pairs
    /// Outer Vec = channels, Inner Vec = peaks
    pub peaks: Vec<Vec<(f32, f32)>>,
    /// Number of samples represented by each peak pair
    pub samples_per_peak: usize,
}

impl ThumbnailLevel {
    /// Create a new thumbnail level
    pub fn new(samples_per_peak: usize, num_channels: usize) -> Self {
        Self {
            peaks: vec![Vec::new(); num_channels],
            samples_per_peak,
        }
    }

    /// Get the number of peak pairs at this level
    pub fn len(&self) -> usize {
        self.peaks.first().map(|p| p.len()).unwrap_or(0)
    }

    /// Check if this level has no peaks
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get number of channels
    pub fn num_channels(&self) -> usize {
        self.peaks.len()
    }

    /// Get peaks for a specific channel
    pub fn channel_peaks(&self, channel: usize) -> Option<&[(f32, f32)]> {
        self.peaks.get(channel).map(|v| v.as_slice())
    }

    /// Get combined peaks across all channels (min of mins, max of maxes)
    pub fn combined_peaks(&self) -> Vec<(f32, f32)> {
        if self.peaks.is_empty() {
            return Vec::new();
        }

        let len = self.len();
        let mut combined = Vec::with_capacity(len);

        for i in 0..len {
            let mut min = f32::MAX;
            let mut max = f32::MIN;

            for channel in &self.peaks {
                if let Some(&(ch_min, ch_max)) = channel.get(i) {
                    min = min.min(ch_min);
                    max = max.max(ch_max);
                }
            }

            combined.push((min, max));
        }

        combined
    }
}

/// Multi-resolution audio thumbnail for efficient waveform display
///
/// Stores peak data at multiple resolutions, allowing efficient rendering
/// at any zoom level. Higher levels have lower resolution (more samples per peak).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioThumbnail {
    /// Mipmap levels, from highest resolution (level 0) to lowest
    levels: Vec<ThumbnailLevel>,
    /// Original sample rate of the audio
    sample_rate: u32,
    /// Number of audio channels
    num_channels: u8,
    /// Total number of samples in original audio (per channel)
    total_samples: usize,
}

impl AudioThumbnail {
    /// Create a new empty thumbnail
    pub fn new(sample_rate: u32, num_channels: u8, total_samples: usize) -> Self {
        Self {
            levels: Vec::new(),
            sample_rate,
            num_channels,
            total_samples,
        }
    }

    /// Create thumbnail from pre-computed levels
    pub fn from_levels(
        levels: Vec<ThumbnailLevel>,
        sample_rate: u32,
        num_channels: u8,
        total_samples: usize,
    ) -> Self {
        Self {
            levels,
            sample_rate,
            num_channels,
            total_samples,
        }
    }

    /// Get the sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get the number of channels
    pub fn num_channels(&self) -> u8 {
        self.num_channels
    }

    /// Get total number of samples (per channel)
    pub fn total_samples(&self) -> usize {
        self.total_samples
    }

    /// Get duration in seconds
    pub fn duration_secs(&self) -> f64 {
        self.total_samples as f64 / self.sample_rate as f64
    }

    /// Get number of mipmap levels
    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    /// Get a specific level
    pub fn level(&self, index: usize) -> Option<&ThumbnailLevel> {
        self.levels.get(index)
    }

    /// Get all levels
    pub fn levels(&self) -> &[ThumbnailLevel] {
        &self.levels
    }

    /// Add a level to the thumbnail
    pub fn add_level(&mut self, level: ThumbnailLevel) {
        self.levels.push(level);
    }

    /// Select the best level for rendering based on desired samples per pixel
    ///
    /// Returns the level index and the level itself
    pub fn select_level(&self, samples_per_pixel: f64) -> Option<(usize, &ThumbnailLevel)> {
        if self.levels.is_empty() {
            return None;
        }

        // Find the level with the closest samples_per_peak to our desired samples_per_pixel
        // We want a level where samples_per_peak <= samples_per_pixel (at least 1 peak per pixel)
        // but as close as possible to avoid wasting resolution

        let mut best_level = 0;

        for (i, level) in self.levels.iter().enumerate() {
            if level.samples_per_peak as f64 <= samples_per_pixel {
                best_level = i;
            } else {
                // Levels are ordered from high to low resolution, so we can stop
                break;
            }
        }

        Some((best_level, &self.levels[best_level]))
    }

    /// Get peaks suitable for rendering a specific pixel width
    ///
    /// # Arguments
    /// * `pixel_width` - Width in pixels to render
    /// * `start_time` - Start time in seconds
    /// * `end_time` - End time in seconds
    ///
    /// # Returns
    /// Combined peaks (min, max) for the requested range, resampled to fit pixel_width
    pub fn get_peaks_for_width(
        &self,
        pixel_width: usize,
        start_time: f64,
        end_time: f64,
    ) -> Vec<(f32, f32)> {
        if pixel_width == 0 || self.levels.is_empty() || start_time >= end_time {
            return Vec::new();
        }

        let duration = end_time - start_time;
        let samples_in_range = (duration * self.sample_rate as f64) as usize;
        let samples_per_pixel = samples_in_range as f64 / pixel_width as f64;

        // Select appropriate level
        let Some((_, level)) = self.select_level(samples_per_pixel) else {
            return Vec::new();
        };

        let combined = level.combined_peaks();
        if combined.is_empty() {
            return Vec::new();
        }

        // Calculate which peaks correspond to our time range
        let start_sample = (start_time * self.sample_rate as f64) as usize;
        let end_sample = (end_time * self.sample_rate as f64).ceil() as usize;

        let start_peak = start_sample / level.samples_per_peak;
        let end_peak = (end_sample + level.samples_per_peak - 1) / level.samples_per_peak;

        let start_peak = start_peak.min(combined.len());
        let end_peak = end_peak.min(combined.len());

        if start_peak >= end_peak {
            return Vec::new();
        }

        let source_peaks = &combined[start_peak..end_peak];

        // Resample to fit pixel_width
        Self::resample_peaks(source_peaks, pixel_width)
    }

    /// Get peaks for a specific channel
    pub fn get_channel_peaks_for_width(
        &self,
        channel: usize,
        pixel_width: usize,
        start_time: f64,
        end_time: f64,
    ) -> Vec<(f32, f32)> {
        if pixel_width == 0 || self.levels.is_empty() || start_time >= end_time {
            return Vec::new();
        }

        let duration = end_time - start_time;
        let samples_in_range = (duration * self.sample_rate as f64) as usize;
        let samples_per_pixel = samples_in_range as f64 / pixel_width as f64;

        let Some((_, level)) = self.select_level(samples_per_pixel) else {
            return Vec::new();
        };

        let Some(channel_peaks) = level.channel_peaks(channel) else {
            return Vec::new();
        };

        if channel_peaks.is_empty() {
            return Vec::new();
        }

        // Calculate which peaks correspond to our time range
        let start_sample = (start_time * self.sample_rate as f64) as usize;
        let end_sample = (end_time * self.sample_rate as f64).ceil() as usize;

        let start_peak = start_sample / level.samples_per_peak;
        let end_peak = (end_sample + level.samples_per_peak - 1) / level.samples_per_peak;

        let start_peak = start_peak.min(channel_peaks.len());
        let end_peak = end_peak.min(channel_peaks.len());

        if start_peak >= end_peak {
            return Vec::new();
        }

        let source_peaks = &channel_peaks[start_peak..end_peak];

        Self::resample_peaks(source_peaks, pixel_width)
    }

    /// Resample peaks to a target width
    fn resample_peaks(source: &[(f32, f32)], target_width: usize) -> Vec<(f32, f32)> {
        if source.is_empty() || target_width == 0 {
            return Vec::new();
        }

        if source.len() == target_width {
            return source.to_vec();
        }

        let mut result = Vec::with_capacity(target_width);
        let ratio = source.len() as f64 / target_width as f64;

        for i in 0..target_width {
            let start = (i as f64 * ratio) as usize;
            let end = ((i + 1) as f64 * ratio).ceil() as usize;
            let end = end.min(source.len());

            if start >= end {
                // Edge case: just use nearest
                let idx = (i as f64 * ratio) as usize;
                let idx = idx.min(source.len() - 1);
                result.push(source[idx]);
            } else {
                // Aggregate peaks in this range
                let mut min = f32::MAX;
                let mut max = f32::MIN;

                for &(p_min, p_max) in &source[start..end] {
                    min = min.min(p_min);
                    max = max.max(p_max);
                }

                result.push((min, max));
            }
        }

        result
    }
}

impl Default for AudioThumbnail {
    fn default() -> Self {
        Self::new(44100, 2, 0)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thumbnail_level_creation() {
        let level = ThumbnailLevel::new(64, 2);
        assert_eq!(level.samples_per_peak, 64);
        assert_eq!(level.num_channels(), 2);
        assert!(level.is_empty());
    }

    #[test]
    fn test_thumbnail_level_combined_peaks() {
        let mut level = ThumbnailLevel::new(64, 2);
        level.peaks[0] = vec![(-0.5, 0.5), (-0.3, 0.8)];
        level.peaks[1] = vec![(-0.7, 0.4), (-0.2, 0.6)];

        let combined = level.combined_peaks();
        assert_eq!(combined.len(), 2);
        assert_eq!(combined[0], (-0.7, 0.5)); // min of mins, max of maxes
        assert_eq!(combined[1], (-0.3, 0.8));
    }

    #[test]
    fn test_thumbnail_duration() {
        let thumbnail = AudioThumbnail::new(44100, 2, 44100 * 10); // 10 seconds
        assert!((thumbnail.duration_secs() - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_level_selection() {
        let mut thumbnail = AudioThumbnail::new(44100, 2, 44100 * 60);

        // Add levels with increasing samples_per_peak
        let mut level0 = ThumbnailLevel::new(64, 2);
        level0.peaks[0] = vec![(0.0, 0.0); 100];
        level0.peaks[1] = vec![(0.0, 0.0); 100];

        let mut level1 = ThumbnailLevel::new(256, 2);
        level1.peaks[0] = vec![(0.0, 0.0); 25];
        level1.peaks[1] = vec![(0.0, 0.0); 25];

        let mut level2 = ThumbnailLevel::new(1024, 2);
        level2.peaks[0] = vec![(0.0, 0.0); 6];
        level2.peaks[1] = vec![(0.0, 0.0); 6];

        thumbnail.add_level(level0);
        thumbnail.add_level(level1);
        thumbnail.add_level(level2);

        // Should select level 0 for high detail
        let (idx, _) = thumbnail.select_level(100.0).unwrap();
        assert_eq!(idx, 0);

        // Should select level 1 for medium detail
        let (idx, _) = thumbnail.select_level(300.0).unwrap();
        assert_eq!(idx, 1);

        // Should select level 2 for low detail
        let (idx, _) = thumbnail.select_level(2000.0).unwrap();
        assert_eq!(idx, 2);
    }

    #[test]
    fn test_resample_peaks() {
        let source = vec![(-1.0, 1.0), (-0.5, 0.5), (-0.8, 0.8), (-0.3, 0.3)];

        // Downsample to 2
        let result = AudioThumbnail::resample_peaks(&source, 2);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (-1.0, 1.0)); // min/max of first two
        assert_eq!(result[1], (-0.8, 0.8)); // min/max of last two

        // Upsample to 8 (each source peak appears twice approximately)
        let result = AudioThumbnail::resample_peaks(&source, 8);
        assert_eq!(result.len(), 8);
    }
}
