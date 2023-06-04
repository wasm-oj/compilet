use async_compression::tokio::bufread::GzipEncoder;
use rocket::{fairing::AdHoc, http::Header, tokio::io};
use std::io::Cursor;

pub fn fairing() -> AdHoc {
    AdHoc::on_response("Gzip Compression", |req, res| {
        Box::pin(async move {
            // Check if the client accepts gzip encoding.
            if !req
                .headers()
                .get_one("Accept-Encoding")
                .map(|encodings| encodings.split(',').any(|x| x.trim() == "gzip"))
                .unwrap_or(false)
            {
                return;
            }

            // Check if the response is already encoded.
            if res.headers().get_one("Content-Encoding").is_some() {
                return;
            }

            // Encode the response.
            let body = res.body_mut().take();
            let compressed: Vec<u8> = {
                let mut compressor = GzipEncoder::with_quality(
                    io::BufReader::new(body),
                    async_compression::Level::Best,
                );

                let mut out = Vec::new();
                io::copy(&mut compressor, &mut out)
                    .await
                    .expect("Failed to compress response");

                out
            };

            // Set the response's body and headers.
            res.set_header(Header::new("Content-Encoding", "gzip"));
            res.set_sized_body(compressed.len(), Cursor::new(compressed));
        })
    })
}
