use axum_learn::run;
#[tokio::main] // untuk kebutuhan koneksi ke server (yang ada di file lib.rs)
async fn main() {
    run().await;
}