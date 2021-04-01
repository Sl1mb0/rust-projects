use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct ModexpParameters {
    x: u64,
    y: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
	    .route("/", web::get().to(get_index))
	    .route("/modexp", web::post().to(post_modexp))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
	.run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        r#"
            <title>Modexp() Calculator</title>
	    <form action="/modexp" method="post">
	    <input type="text" name="x"/>
	    <input type="text" name="y"/>
	    <input type="text" name="m"/>
	    <button type="submit">Compute Modexp()</button>
	    </form>
	"#,
    )
}

fn post_modexp(form: web::Form<ModexpParameters>) -> HttpResponse {
    let response = 
        format!("<b>{}</b> to the power of <b>{}</b> modulo <b>{}</b> is <b>{}</b>\n", form.x, form.y, form.m, modexp(form.x, form.y, form.m));

	HttpResponse::Ok()
	    .content_type("text/html")
	    .body(response)
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if x == 0 {
        0
    } else if y == 0 {
        1
    }
    else {
        let mut z = modexp(x, y / 2, m); // z must be mutable
        z = (z * z) % m;

        if y % 2 == 1 {
            z = (z * x) % m;
        }

        z
    }
}
