#[tokio::main]
async fn main() {
    niqin::db::init().await;
    niqin::db::schema::ensure_tables()
        .await
        .unwrap_or_else(|e| panic!("ensure tables: {e}"));
    niqin::db::products::seed_products()
        .await
        .unwrap_or_else(|e| panic!("seed products: {e}"));

    topcoat::start(niqin::app::router()).await.unwrap();
}
