//! HTTP client utilities.

/// Creates a `reqwest::ClientBuilder` configured with appropriate root certificates.
///
/// On Android and musl-based standalone binaries, the platform verifier may not have
/// any usable system trust store available, which can lead to errors such as
/// `No CA certificates were loaded from the system`. In those environments, fall back
/// to the Mozilla root certificates bundled by `webpki-root-certs`.
pub fn client_builder() -> reqwest::ClientBuilder {
    // reqwest is built with `rustls-no-provider`; install the ring provider (the
    // one the rest of the workspace uses) before any client is constructed.
    let _ = rustls::crypto::ring::default_provider().install_default();
    let builder = reqwest::Client::builder();
    #[cfg(any(target_os = "android", target_env = "musl"))]
    let builder = {
        let certs = webpki_root_certs::TLS_SERVER_ROOT_CERTS
            .iter()
            .filter_map(|c| reqwest::Certificate::from_der(c.as_ref()).ok());
        certs.fold(builder, |builder, cert| builder.add_root_certificate(cert))
    };
    builder
}

/// Creates a default `reqwest::Client`.
#[must_use]
pub fn client() -> reqwest::Client {
    client_builder().build().unwrap_or_default()
}
