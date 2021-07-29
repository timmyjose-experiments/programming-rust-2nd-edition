use actix_gcd;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    m: u64,
    n: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding on localhost:3000")
        .run()
        .expect("error while running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="m"/>
        <input type="text" name="n"/>
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("GCD cannot be calculated for 0 and 0");
    }

    let response = format!(
        "The GCD of {} and {} is <b>{}</b>\n",
        form.m,
        form.n,
        actix_gcd::gcd(form.m, form.n)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}
