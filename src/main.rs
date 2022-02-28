use axum::{
    // extract::Path,
    http::{header::LOCATION, HeaderMap, StatusCode},
    // response::{Redirect, IntoResponse},
    routing::{get, get_service, patch, post, MethodRouter},
    AddExtensionLayer,
    Router,
};
use redis::Client;
use sea_orm::Database;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::Level;

mod entity;
mod handler;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_test_writer()
        .init();
    // mysql数据库连接
    let mysql_url = "mysql://root:alex-gcx@47.102.114.90/ihome?ssl-mode=DISABLED";
    let db = Database::connect(mysql_url).await.unwrap();
    // redis连接
    let redis_url = "redis://:alex-gcx@47.102.114.90:6380/";
    let redis_client = Client::open(redis_url).unwrap();
    // 初始化数据库
    let init_area_route = Router::new().route("/", post(handler::init_db::insert_areas));
    let init_fac_route = Router::new().route("/", post(handler::init_db::insert_fac));
    let init_db_route = Router::new()
        .nest("/areas", init_area_route)
        .nest("/facilities", init_fac_route);
    // 区域路由
    let areas_route = Router::new().route("/", get(handler::houses::get_areas));
    // 房间路由
    let houses_route = Router::new()
        .route(
            "/",
            post(handler::houses::publish_house).put(handler::houses::update_house),
        )
        .route("/images", post(handler::houses::set_house_image))
        .route("/:house_id", get(handler::houses::get_house_info));
    // 用户路由
    let users_route = Router::new()
        .route("/", post(handler::users::register))
        .route("/info", get(handler::users::get_user_info))
        .route("/images", patch(handler::users::set_user_image))
        .route("/names", patch(handler::users::set_user_name))
        .route("/houses", get(handler::houses::get_user_house))
        .route(
            "/auth",
            get(handler::users::get_user_auth).patch(handler::users::authenticate),
        );
    // session路由-登录-登出
    let sessions_route = Router::new().route(
        "/",
        get(handler::users::get_user)
            .post(handler::users::login)
            .delete(handler::users::logout),
    );
    // index路由
    let index_route = Router::new().route("/houses", get(handler::houses::get_index_houses));
    // search路由
    let search_route = Router::new().route("/houses", get(handler::houses::get_search_houses));
    // booking路由
    let booking_route =
        Router::new().route("/houses/:house_id", get(handler::houses::get_order_house));
    // orders路由
    let orders_route = Router::new()
        .route(
            "/",
            post(handler::orders::create_order).get(handler::orders::get_orders),
        )
        .route("/accept/:order_id", patch(handler::orders::handle_order))
        .route("/comment/:order_id", patch(handler::orders::comment_order))
        .route("/cancel/:order_id", patch(handler::orders::cancel_order))
        .route("/alipay", post(handler::orders::pay_order));
    // api路由
    let api_route = Router::new()
        .nest("/init_db", init_db_route)
        .nest("/users", users_route)
        .nest("/houses", houses_route)
        .nest("/areas", areas_route)
        .nest("/index", index_route)
        .nest("/search", search_route)
        .nest("/booking", booking_route)
        .nest("/orders", orders_route)
        .nest("/sessions", sessions_route);
    // 根路由
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/:html", static_redirect("./static/html"))
        .nest("/api/v1.0", api_route)
        .nest("/static", static_redirect("./static"))
        .layer(AddExtensionLayer::new(db))
        .layer(AddExtensionLayer::new(redis_client));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 首页
async fn index_handler() -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, "/static/html/index.html".parse().unwrap());
    (StatusCode::FOUND, headers, ())
}

// 页面重定向
// async fn html_handler(Path(html): Path<String>) -> (StatusCode, HeaderMap, ()) {
//     let mut headers = HeaderMap::new();
//     headers.insert(LOCATION, format!("/static/html/{}", html).parse().unwrap());
//     (StatusCode::FOUND, headers, ())
// }

// 静态文件重定向
fn static_redirect(file: &str) -> MethodRouter {
    get_service(ServeDir::new(file)).handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error:{}", error),
        )
    })
}
// async fn static_redirect(file: &str) -> impl IntoResponse {
//     Redirect::found(file.parse().unwrap())
// }
