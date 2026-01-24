//! Thumbnail Generator
//!
//! Generates multi-resolution audio thumbnails from raw sample data.
//!
//! # Example
//!
//! ```ignore
//! use armas_audio::{ThumbnailGenerator, AudioThumbnail};
//!
//! // Mono f32 samples
//! let samples: Vec<f32> = load_audio_file("song.wav");
//! let thumbnail = ThumbnailGenerator::new()
//!     .base_resolution(64)
//!     .num_levels(5)
//!     .generate_mono(&samples, 44100);
//!
//! // Interleaved stereo samples
//! let stereo_samples: Vec<f32> = load_stereo_file("song.wav");
//! let thumbnail = ThumbnailGenerator::new()
//!     .generate_interleaved(&stereo_samples, 44100, 2);
//! ```

use crate::waveform_thumbnail::{AudioThumbnail, ThumbnailLevel};

/// Generator for creating audio thumbnails from sample data
#[derive(Debug, Clone)]
pub struct ThumbnailGenerator {
    /// Samples per peak at the highest resolution level (level 0)
    base_resolution: usize,
    /// Number of mipmap levels to generate
    num_levels: usize,
    /// Multiplier between levels (typically 4)
    level_multiplier: usize,
}

impl ThumbnailGenerator {
    /// Create a new thumbnail generator with default settings
    ///
    /// Defaults:
    /// - Base resolution: 64 samples per peak
    /// - Number of levels: 5
    /// - Level multiplier: 4x
    pub fn new() -> Self {
        Self {
            base_resolution: 64,
            num_levels: 5,
            level_multiplier: 4,
        }
    }

    /// Set the base resolution (samples per peak at level 0)
    ///
    /// Lower values = higher quality but more memory
    /// Typical range: 32-256
    pub fn base_resolution(mut self, samples_per_peak: usize) -> Self {
        self.base_resolution = samples_per_peak.max(1);
        self
    }

    /// Set the number of mipmap levels to generate
    ///
    /// More levels = better zoom range but more memory
    /// Typical range: 3-6
    pub fn num_levels(mut self, levels: usize) -> Self {
        self.num_levels = levels.max(1);
        self
    }

    /// Set the multiplier between levels
    ///
    /// Higher values = fewer levels needed but coarser steps
    /// Typical value: 4
    pub fn level_multiplier(mut self, multiplier: usize) -> Self {
        self.level_multiplier = multiplier.max(2);
        self
    }

    /// Generate thumbnail from mono samples
    pub fn generate_mono<T, F>(&self, samples: &[T], sample_rate: u32, amplitude_fn: F) -> AudioThumbnail
    where
        F: Fn(&T) -> f32,
    {
        let total_samples = samples.len();
        let mut thumbnail = AudioThumbnail::new(sample_rate, 1, total_samples);

        // Generate each level
        let mut current_resolution = self.base_resolution;
        for _ in 0..self.num_levels {
            let level = self.generate_level_mono(samples, current_resolution, &amplitude_fn);
            thumbnail.add_level(level);
            current_resolution *= self.level_multiplier;

            // Stop if resolution exceeds sample count
            if current_resolution > total_samples {
                break;
            }
        }

        thumbnail
    }

    /// Generate thumbnail from interleaved multi-channel samples
    ///
    /// Samples are expected in interleaved format: [L0, R0, L1, R1, ...]
    pub fn generate_interleaved<T, F>(
        &self,
        samples: &[T],
        sample_rate: u32,
        num_channels: u8,
        amplitude_fn: F,
    ) -> AudioThumbnail
    where
        F: Fn(&T) -> f32,
    {
        let samples_per_channel = samples.len() / num_channels as usize;
        let mut thumbnail = AudioThumbnail::new(sample_rate, num_channels, samples_per_channel);

        // Generate each level
        let mut current_resolution = self.base_resolution;
        for _ in 0..self.num_levels {
            let level = self.generate_level_interleaved(
                samples,
                num_channels as usize,
                current_resolution,
                &amplitude_fn,
            );
            thumbnail.add_level(level);
            current_resolution *= self.level_multiplier;

            // Stop if resolution exceeds sample count
            if current_resolution > samples_per_channel {
                break;
            }
        }

        thumbnail
    }

    /// Generate thumbnail from separate channel buffers
    pub fn generate_channels<T, F>(
        &self,
        channels: &[&[T]],
        sample_rate: u32,
        amplitude_fn: F,
    ) -> AudioThumbnail
    where
        F: Fn(&T) -> f32,
    {
        if channels.is_empty() {
            return AudioThumbnail::default();
        }

        let num_channels = channels.len() as u8;
        let samples_per_channel = channels[0].len();
        let mut thumbnail = AudioThumbnail::new(sample_rate, num_channels, samples_per_channel);

        // Generate each level
        let mut current_resolution = self.base_resolution;
        for _ in 0..self.num_levels {
            let level = self.generate_level_channels(channels, current_resolution, &amplitude_fn);
            thumbnail.add_level(level);
            current_resolution *= self.level_multiplier;

            // Stop if resolution exceeds sample count
            if current_resolution > samples_per_channel {
                break;
            }
        }

        thumbnail
    }

    // ========================================================================
    // Internal Methods
    // ========================================================================

    fn generate_level_mono<T, F>(
        &self,
        samples: &[T],
        samples_per_peak: usize,
        amplitude_fn: &F,
    ) -> ThumbnailLevel
    where
        F: Fn(&T) -> f32,
    {
        let mut level = ThumbnailLevel::new(samples_per_peak, 1);

        let num_peaks = (samples.len() + samples_per_peak - 1) / samples_per_peak;
        let mut peaks = Vec::with_capacity(num_peaks);

        for chunk in samples.chunks(samples_per_peak) {
            let (min, max) = Self::compute_peak(chunk, amplitude_fn);
            peaks.push((min, max));
        }

        level.peaks[0] = peaks;
        level
    }

    fn generate_level_interleaved<T, F>(
        &self,
        samples: &[T],
        num_channels: usize,
        samples_per_peak: usize,
        amplitude_fn: &F,
    ) -> ThumbnailLevel
    where
        F: Fn(&T) -> f32,
    {
        let mut level = ThumbnailLevel::new(samples_per_peak, num_channels);

        let samples_per_channel = samples.len() / num_channels;
        let num_peaks = (samples_per_channel + samples_per_peak - 1) / samples_per_peak;

        // Initialize peak vectors for each channel
        for ch in 0..num_channels {
            level.peaks[ch] = Vec::with_capacity(num_peaks);
        }

        // Process in chunks of samples_per_peak frames
        let frames_per_chunk = samples_per_peak;
        let samples_per_chunk = frames_per_chunk * num_channels;

        for chunk_start in (0..samples.len()).step_by(samples_per_chunk) {
            let chunk_end = (chunk_start + samples_per_chunk).min(samples.len());

            // Compute min/max for each channel in this chunk
            for ch in 0..num_channels {
                let mut min = f32::MAX;
                let mut max = f32::MIN;

                let mut sample_idx = chunk_start + ch;
                while sample_idx < chunk_end {
                    let amp = amplitude_fn(&samples[sample_idx]);
                    min = min.min(amp);
                    max = max.max(amp);
                    sample_idx += num_channels;
                }

                if min != f32::MAX {
                    level.peaks[ch].push((min, max));
                }
            }
        }

        level
    }

    fn generate_level_channels<T, F>(
        &self,
        channels: &[&[T]],
        samples_per_peak: usize,
        amplitude_fn: &F,
    ) -> ThumbnailLevel
    where
        F: Fn(&T) -> f32,
    {
        let num_channels = channels.len();
        let mut level = ThumbnailLevel::new(samples_per_peak, num_channels);

        for (ch, channel_data) in channels.iter().enumerate() {
            let num_peaks = (channel_data.len() + samples_per_peak - 1) / samples_per_peak;
            let mut peaks = Vec::with_capacity(num_peaks);

            for chunk in channel_data.chunks(samples_per_peak) {
                let (min, max) = Self::compute_peak(chunk, amplitude_fn);
                peaks.push((min, max));
            }

            level.peaks[ch] = peaks;
        }

        level
    }

    /// Compute min/max peak for a chunk of samples
    fn compute_peak<T, F>(samples: &[T], amplitude_fn: &F) -> (f32, f32)
    where
        F: Fn(&T) -> f32,
    {
        let mut min = f32::MAX;
        let mut max = f32::MIN;

        for sample in samples {
            let amp = amplitude_fn(sample);
            min = min.min(amp);
            max = max.max(amp);
        }

        if min == f32::MAX {
            (0.0, 0.0)
        } else {
            (min, max)
        }
    }
}

impl Default for ThumbnailGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Convenience Functions
// ============================================================================

/// Generate a thumbnail from f32 mono samples using default settings
pub fn generate_thumbnail_f32_mono(samples: &[f32], sample_rate: u32) -> AudioThumbnail {
    ThumbnailGenerator::new().generate_mono(samples, sample_rate, |s| *s)
}

/// Generate a thumbnail from f32 interleaved stereo samples using default settings
pub fn generate_thumbnail_f32_stereo(samples: &[f32], sample_rate: u32) -> AudioThumbnail {
    ThumbnailGenerator::new().generate_interleaved(samples, sample_rate, 2, |s| *s)
}

/// Generate a thumbnail from i16 mono samples using default settings
pub fn generate_thumbnail_i16_mono(samples: &[i16], sample_rate: u32) -> AudioThumbnail {
    ThumbnailGenerator::new().generate_mono(samples, sample_rate, |s| *s as f32 / 32768.0)
}

/// Generate a thumbnail from i16 interleaved stereo samples using default settings
pub fn generate_thumbnail_i16_stereo(samples: &[i16], sample_rate: u32) -> AudioThumbnail {
    ThumbnailGenerator::new().generate_interleaved(samples, sample_rate, 2, |s| *s as f32 / 32768.0)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine_wave(frequency: f32, sample_rate: u32, duration_secs: f32) -> Vec<f32> {
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        (0..num_samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                (2.0 * std::f32::consts::PI * frequency * t).sin()
            })
            .collect()
    }

    #[test]
    fn test_generator_mono() {
        let samples = generate_sine_wave(440.0, 44100, 1.0);
        let thumbnail = ThumbnailGenerator::new()
            .base_resolution(64)
            .num_levels(3)
            .generate_mono(&samples, 44100, |s| *s);

        assert_eq!(thumbnail.sample_rate(), 44100);
        assert_eq!(thumbnail.num_channels(), 1);
        assert_eq!(thumbnail.num_levels(), 3);
        assert!((thumbnail.duration_secs() - 1.0).abs() < 0.001);

        // Check level resolutions
        assert_eq!(thumbnail.level(0).unwrap().samples_per_peak, 64);
        assert_eq!(thumbnail.level(1).unwrap().samples_per_peak, 256);
        assert_eq!(thumbnail.level(2).unwrap().samples_per_peak, 1024);
    }

    #[test]
    fn test_generator_stereo_interleaved() {
        // Generate interleaved stereo (L, R, L, R, ...)
        let mono = generate_sine_wave(440.0, 44100, 0.5);
        let stereo: Vec<f32> = mono.iter().flat_map(|&s| [s, s * 0.5]).collect();

        let thumbnail = ThumbnailGenerator::new()
            .base_resolution(64)
            .num_levels(2)
            .generate_interleaved(&stereo, 44100, 2, |s| *s);

        assert_eq!(thumbnail.num_channels(), 2);
        assert_eq!(thumbnail.num_levels(), 2);

        // Check that left channel has higher peaks than right (right is scaled by 0.5)
        let level = thumbnail.level(0).unwrap();
        let left_peaks = level.channel_peaks(0).unwrap();
        let right_peaks = level.channel_peaks(1).unwrap();

        assert!(!left_peaks.is_empty());
        assert_eq!(left_peaks.len(), right_peaks.len());
    }

    #[test]
    fn test_generator_channels() {
        let left = generate_sine_wave(440.0, 44100, 0.5);
        let right = generate_sine_wave(880.0, 44100, 0.5);

        let thumbnail = ThumbnailGenerator::new()
            .base_resolution(64)
            .generate_channels(&[&left, &right], 44100, |s| *s);

        assert_eq!(thumbnail.num_channels(), 2);
    }

    #[test]
    fn test_convenience_functions() {
        let mono: Vec<f32> = (0..1000).map(|i| (i as f32 / 100.0).sin()).collect();
        let thumbnail = generate_thumbnail_f32_mono(&mono, 44100);
        assert_eq!(thumbnail.num_channels(), 1);

        let stereo: Vec<f32> = (0..2000).map(|i| (i as f32 / 100.0).sin()).collect();
        let thumbnail = generate_thumbnail_f32_stereo(&stereo, 44100);
        assert_eq!(thumbnail.num_channels(), 2);
    }

    #[test]
    fn test_get_peaks_for_width() {
        let samples = generate_sine_wave(440.0, 44100, 1.0);
        let thumbnail = ThumbnailGenerator::new()
            .base_resolution(64)
            .num_levels(4)
            .generate_mono(&samples, 44100, |s| *s);

        // Get peaks for full duration at 1000 pixels
        let peaks = thumbnail.get_peaks_for_width(1000, 0.0, 1.0);
        assert_eq!(peaks.len(), 1000);

        // Verify peaks are in valid range for sine wave
        for (min, max) in &peaks {
            assert!(*min >= -1.0 && *min <= 1.0);
            assert!(*max >= -1.0 && *max <= 1.0);
            assert!(min <= max);
        }

        // Get peaks for half duration
        let peaks = thumbnail.get_peaks_for_width(500, 0.0, 0.5);
        assert_eq!(peaks.len(), 500);
    }
}
