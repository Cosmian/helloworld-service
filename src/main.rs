#[cfg(feature = "confidential")]
use axum::{body::Bytes, http::StatusCode, routing::post, Json};
use axum::{routing::get, Router};

#[cfg(feature = "confidential")]
mod report {
    use serde::Serialize;
    mod base64 {
        use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
        use serde::{Serialize, Serializer};

        pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
            String::serialize(&BASE64_STANDARD.encode(v), s)
        }
    }

    #[derive(Serialize)]
    pub struct Report {
        /// ATTESTATION_REPORT Structure defined in Table 21 of SEV-SNP firmware ABI specification
        #[serde(with = "base64")]
        pub attestation: Vec<u8>,
        /// Concatenation of VCEK, ASK, and ARK certificates (PEM format, in that order).
        /// <https://www.amd.com/en/support/tech-docs/versioned-chip-endorsement-key-vcek-certificate-and-kds-interface-specification>
        pub platform_certificates: String,
        #[serde(with = "base64")]
        pub uvm_endorsements: Vec<u8>,
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let mut app = Router::new();

    // `GET /` goes to `root()`
    app = app.route("/", get(root));

    #[cfg(feature = "confidential")]
    {
        // `GET /report` goes to `report()`
        app = app.route("/report", post(report));
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    tracing::debug!("GET /");

    "Hello, World!"
}

#[cfg(feature = "confidential")]
async fn report(body: Bytes) -> Result<Json<report::Report>, StatusCode> {
    tracing::debug!("POST /report");

    if body.len() > 64 {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let report_data = if !body.is_empty() {
        Some(body.as_ref())
    } else {
        None
    };

    match az_cca::fetch_attestation(Some("/mnt/uds/attestation-container.sock"), report_data).await
    {
        Ok(quote) => Ok(Json(report::Report {
            attestation: quote.attestation,
            platform_certificates: String::from_utf8_lossy(&quote.platform_certificates)
                .to_string(),
            uvm_endorsements: quote.uvm_endorsements,
        })),
        Err(e) => {
            tracing::debug!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
