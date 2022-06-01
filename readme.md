# Actix-error-mapper-middleware

This simple crate allows you to remap actix-web errors to your own custom error type. You could for example return a html wrapped error. 

## Example

Your custom error trait has to implement the 
```std::convert::From<actix_web::Error>``` and the ```actix_web::ResponseError``` traits. 

```rust 
use actix_error_mapper_middleware::MapperService;
use actix_jwt_auth_middleware::{AuthService, Authority};
use actix_web::{web, App, Error as ActixError, HttpResponse, HttpServer, ResponseError};
use rusty_html::{html, HTMLify};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // this wrap will catch every error thrown in the Apps scope an convert it to the MyError type
            .wrap(MapperService::<MyError>::new())
            .service(
                web::scope("")
                    // this service will throw an error if you are not logged in
                    .wrap(AuthService::new(Authority::<u32>::default())),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Debug)]
struct MyError {
    error: ActixError,
}

impl std::convert::From<ActixError> for MyError {
    fn from(error: ActixError) -> Self {
        Self { error }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "An error occurred: \"{}\"",
            self.error.to_string()
        ))
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error.as_response_error().status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(html!(
        <html style="
                    margin: 0;
                    height: 100%;
                ">
            <head>
                <title>Error</title>
            </head>
            <body style="
                    display: grid;
                    margin: 0;
                    height: 100%;
                    width: 100%;
                    font-family: 'Raleway';
                    place-items: center;
                    color: #fff;
                    overflow-y: hidden;
                    background: #000;
                ">
                <section style="
                        padding: 2rem;
                        background: #289dcc;
                    ">
                    <h1>Ups . . .</h1>
                    <p>{self.to_string()}</p>
                </section>
            </body>
        </html>
        ))
    }
}
```