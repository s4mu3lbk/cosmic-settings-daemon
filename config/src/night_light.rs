// SPDX-License-Identifier: MPL-2.0

use cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic_config::CosmicConfigEntry;

pub const ID: &str = "com.system76.CosmicSettings.NightLight";

/// Gets a cosmic-config [Config] context.
pub fn context() -> Result<cosmic_config::Config, cosmic_config::Error> {
    Config::context()
}

/// Default color temperature in Kelvin when night light is active.
pub const DEFAULT_TEMPERATURE: f32 = 4000.0;

/// Minimum and maximum configurable color temperature in Kelvin.
pub const MIN_TEMPERATURE: f32 = 1000.0;
pub const MAX_TEMPERATURE: f32 = 6500.0;

/// Cosmic-config configuration state for `com.system76.CosmicSettings.NightLight`
#[derive(Clone, Debug, PartialEq, CosmicConfigEntry)]
#[version = 1]
pub struct Config {
    /// Whether night light is enabled.
    pub enabled: bool,
    /// Color temperature in Kelvin applied while night light is active.
    pub temperature: f32,
    /// Whether to schedule night light from sunset to sunrise, using the
    /// system location via GeoClue. Falls back to the manual schedule when
    /// no location is available.
    pub auto_schedule: bool,
    /// Manual schedule start time, local time in minutes after midnight.
    pub schedule_start_minutes: u16,
    /// Manual schedule end time, local time in minutes after midnight.
    pub schedule_end_minutes: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            temperature: DEFAULT_TEMPERATURE,
            auto_schedule: true,
            schedule_start_minutes: 19 * 60,
            schedule_end_minutes: 7 * 60,
        }
    }
}

impl Config {
    pub fn context() -> Result<cosmic_config::Config, cosmic_config::Error> {
        cosmic_config::Config::new(ID, Self::VERSION)
    }
}
