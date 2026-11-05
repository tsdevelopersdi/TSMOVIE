//! Windows only: embed version info (VERSIONINFO) into `tsmovie-cli.exe`, so Explorer shows
//! the product and publisher. On every other target this does nothing.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=FILMCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    windows::embed_resources();
}

#[cfg(windows)]
mod windows {
    pub fn embed_resources() {
        let mut res = winresource::WindowsResource::new();
        res.set("ProductName", "TSMovie")
            .set("CompanyName", "Solusi Daya Indonesia")
            .set("FileDescription", "TSMovie command-line tool")
            .set("LegalCopyright", "Copyright (c) 2026 Solusi Daya Indonesia. MIT OR Apache-2.0.")
            .set("OriginalFilename", "tsmovie-cli.exe")
            .set("InternalName", "tsmovie-cli");
        if let Err(e) = res.compile() {
            if std::env::var_os("FILMCRAFT_REQUIRE_WINRES").is_some() {
                eprintln!("embedding Windows resources failed: {e}");
                std::process::exit(1);
            }
            println!("cargo:warning=tsmovie-cli.exe built without version resources: {e}");
        }
    }
}

/// Cross-compiling for Windows from macOS or Linux: `winresource` is not available on this host.
#[cfg(not(windows))]
mod windows {
    pub fn embed_resources() {
        println!("cargo:warning=tsmovie-cli.exe built without version resources (cross-compiled from a non-Windows host)");
    }
}
