use poem::http::StatusCode;
use poem::test::TestClient;
use poem::Route;
use std::fs;
use tempdir::TempDir;

use crate::SPAEndpoint;

#[tokio::test]
async fn test() {
    let tmp_dir = TempDir::new("example").expect("Temp dir required");

    fs::create_dir(tmp_dir.path().join("assets")).expect("Shoud make dir");
    fs::create_dir(tmp_dir.path().join("subdir")).expect("Shoud make dir");

    fs::write(tmp_dir.path().join("index"), "index").expect("Should create file");
    fs::write(tmp_dir.path().join("top_level"), "top_level").expect("Should create file");
    fs::write(tmp_dir.path().join("subdir/inner"), "inner").expect("Should create file");
    fs::write(tmp_dir.path().join("assets/test"), "test asset").expect("Should create file");

    let app = Route::new().nest(
        "/",
        SPAEndpoint::new(tmp_dir.path(), "index").with_assets("assets"),
    );

    TestClient::new(&app)
        .get("/")
        .send()
        .await
        .assert_text("index")
        .await;

    TestClient::new(&app)
        .get("/index")
        .send()
        .await
        .assert_text("index")
        .await;

    TestClient::new(&app)
        .get("/top_level")
        .send()
        .await
        .assert_text("top_level")
        .await;

    TestClient::new(&app)
        .get("/top_level")
        .send()
        .await
        .assert_text("top_level")
        .await;

    TestClient::new(&app)
        .get("/subdir")
        .send()
        .await
        .assert_text("index")
        .await;

    TestClient::new(&app)
        .get("/subdir/")
        .send()
        .await
        .assert_text("index")
        .await;

    TestClient::new(&app)
        .get("/subdir/inner")
        .send()
        .await
        .assert_text("inner")
        .await;

    TestClient::new(&app)
        .get("/subdir2/inner")
        .send()
        .await
        .assert_text("index")
        .await;

    TestClient::new(&app)
        .get("/assets/test")
        .send()
        .await
        .assert_text("test asset")
        .await;

    TestClient::new(&app)
        .get("/assets/notfound")
        .send()
        .await
        .assert_status(StatusCode::NOT_FOUND);

    TestClient::new(&app)
        .get("/assets")
        .send()
        .await
        .assert_status(StatusCode::FORBIDDEN);

    TestClient::new(&app)
        .get("/assets/")
        .send()
        .await
        .assert_status(StatusCode::FORBIDDEN);

    TestClient::new(&app)
        .get("/..")
        .send()
        .await
        .assert_status(StatusCode::FORBIDDEN);

    TestClient::new(&app)
        .get("/../")
        .send()
        .await
        .assert_status(StatusCode::FORBIDDEN);

    TestClient::new(&app)
        .get("/../a")
        .send()
        .await
        .assert_status(StatusCode::FORBIDDEN);
}
