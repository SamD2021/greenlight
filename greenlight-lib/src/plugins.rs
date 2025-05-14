#[cfg(feature = "plugins")]
use extism::*;
#[cfg(feature = "plugins")]
use serde_json::Value;
#[cfg(feature = "plugins")]
use std::path::Path;

#[cfg(feature = "plugins")]
pub fn run_plugins_from(dir: &Path) -> Vec<Value> {
    let mut results = vec![];
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.path().extension().is_some_and(|e| e == "wasm") {
                let wasm = std::fs::read(entry.path()).unwrap();
                let manifest = Manifest::new([wasm]);
                let mut plugin = PluginBuilder::new(manifest)
                    .with_wasi(true)
                    .build()
                    .unwrap();
                let output: Value = plugin.call("check", "{}").unwrap();
                results.push(output);
            }
        }
    }
    results
}
