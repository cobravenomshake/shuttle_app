use actix_web::{get, web::{self, ServiceConfig}, App, HttpResponse, HttpServer};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> HttpResponse {
    // Define the HTML content with a green background and centered text
    let html_content = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Hello World</title>
            <style>
                body {
                    background-color: green;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                }
                .content {
                    font-size: 24px;
                    color: white;
                }
            </style>
        </head>
        <body>
            <div class="content">
                Hello World!
            </div>
        </body>
        </html>
    "#;

    // Return the HTML content as the response
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html_content)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}

