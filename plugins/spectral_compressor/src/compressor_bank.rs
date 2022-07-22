// Spectral Compressor: an FFT based compressor
// Copyright (C) 2021-2022 Robbert van der Helm
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use nih_plug::prelude::*;

#[derive(Params)]
pub struct CompressorBankParams {
    /// The downwards compression threshold relative to the target curve.
    #[id = "thresh_down_off"]
    downwards_threshold_offset_db: FloatParam,
    #[id = "thresh_up_off"]
    /// The upwards compression threshold relative to the target curve.
    upwards_threshold_offset_db: FloatParam,
    /// The downwards compression ratio. At 1.0 the downwards compressor is disengaged.
    #[id = "ratio_down"]
    downwards_ratio: FloatParam,
    /// The upwards compression ratio. At 1.0 the upwards compressor is disengaged.
    #[id = "ratio_up"]
    upwards_ratio: FloatParam,
    // TODO: High frequency ratio falloff, make the compression milder for higher frequencies to make it less piercing
    /// The compressor's attack time in milliseconds. Controls both upwards and downwards
    /// compression.
    #[id = "attack"]
    compressor_attack_ms: FloatParam,
    /// The compressor's release time in milliseconds. Controls both upwards and downwards
    /// compression.
    #[id = "release"]
    compressor_release_ms: FloatParam,
}

impl Default for CompressorBankParams {
    fn default() -> Self {
        Self {
            // TODO: Set nicer default values for these things
            // As explained above, these offsets are relative to the target curve
            downwards_threshold_offset_db: FloatParam::new(
                "Downwards Offset",
                0.0,
                FloatRange::Linear {
                    min: -50.0,
                    max: 50.0,
                },
            )
            .with_unit(" dB")
            .with_step_size(0.1),
            upwards_threshold_offset_db: FloatParam::new(
                "Upwards Offset",
                0.0,
                FloatRange::Linear {
                    min: -50.0,
                    max: 50.0,
                },
            )
            .with_unit(" dB")
            .with_step_size(0.1),
            downwards_ratio: FloatParam::new(
                "Downwards Ratio",
                1.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 300.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_step_size(0.1)
            .with_value_to_string(formatters::v2s_compression_ratio(1))
            .with_string_to_value(formatters::s2v_compression_ratio()),
            upwards_ratio: FloatParam::new(
                "Upwards Ratio",
                1.0,
                FloatRange::Skewed {
                    min: 1.0,
                    max: 300.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_step_size(0.1)
            .with_value_to_string(formatters::v2s_compression_ratio(1))
            .with_string_to_value(formatters::s2v_compression_ratio()),
            compressor_attack_ms: FloatParam::new(
                "Attack",
                150.0,
                FloatRange::Skewed {
                    // TODO: Make sure to handle 0 attack and release times in the compressor
                    min: 0.0,
                    max: 10_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" ms")
            .with_step_size(0.1),
            compressor_release_ms: FloatParam::new(
                "Release",
                300.0,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 10_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" ms")
            .with_step_size(0.1),
        }
    }
}