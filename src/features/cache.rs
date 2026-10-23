use serde::{Deserialize, Serialize};

/// Cache file format version. Bump to invalidate previously saved caches.
const CACHE_VERSION: u32 = 1;

/// Persisted window size and position, restored on the next launch.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowCache {
    version: u32,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
}

impl WindowCache {
    /// Captures a window's current size and position in logical pixels.
    pub fn capture(width: f32, height: f32, x: f32, y: f32) -> Self {
        Self {
            version: CACHE_VERSION,
            width,
            height,
            x,
            y,
        }
    }

    pub fn size(&self) -> [f32; 2] {
        [self.width, self.height]
    }

    pub fn position(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

/// Loads the cached window geometry, returning `None` when the cache file is
/// missing, unreadable, unparsable, or from an older format version.
pub fn load() -> Option<WindowCache> {
    let path = crate::data::cache_path()?;
    let yaml = std::fs::read_to_string(path).ok()?;
    let cached: WindowCache = serde_saphyr::from_str(&yaml).ok()?;
    (cached.version == CACHE_VERSION).then_some(cached)
}

/// Saves the cache atomically: write to `cache.yaml.tmp` first, then rename
/// it over `cache.yaml`, so a crash mid-write can never leave a corrupt cache.
pub fn save(cached: &WindowCache) -> Result<(), std::io::Error> {
    // TODO: cleanup and extract to data.rs
    let path =
        crate::data::data_dir().ok_or_else(|| other_err("could not determine cache file path"))?;
    let dir = path
        .parent()
        .ok_or_else(|| other_err("could not determine cache directory"))?;
    std::fs::create_dir_all(dir)?;

    let tmp = dir.join("cache.yaml.tmp");
    let yaml = serde_saphyr::to_string(cached).map_err(other_err)?;
    std::fs::write(&tmp, yaml.as_bytes())?;

    // Windows won't rename over an existing file, so remove it first.
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    std::fs::rename(&tmp, &path).map_err(other_err)
}

/// Wraps non-io failures into an `io::Error::Other` so `save` can report
/// serialization and path errors through a single error type.
fn other_err(msg: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, msg)
}
