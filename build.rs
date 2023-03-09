fn main() {
    #[cfg(windows)]
    {
        let cfg = match autocfg::AutoCfg::new() {
            Ok(cfg) => cfg,
            Err(e) => {
                println!(
                    "cargo:warning=wintheme: failed to detect compiler features: {}",
                    e
                );
                return;
            }
        };

        // We use "no_*" instead of "has_*" here. For non-Cargo
        // build tools that don't run build.rs, the negative
        // allows us to treat the current Rust version as the
        // latest stable version, for when version information
        // isn't available.
        if !cfg.probe_rustc_version(1, 63) {
            autocfg::emit("wintheme_no_io_safety");
        }
    }
}
