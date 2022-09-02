use response::BaseResponse;
use simple_http_server::Server;

fn main() {
    let mut server = Server::new("0.0.0.0:7878", 16);

    server.mount("dist", "/");

    server.redirect("/test2", "/111");

    server.api.get("/test", |_| {
        BaseResponse::success().file("dist/index.html").unwrap()
    });

    server.api.get("/hello", |_| {
        BaseResponse::success()
            .string("Hello, world")
            .set_content_type("text/html")
    });

    server.run();
}
