use response::BaseResponse;

use methods::Methods;
use simple_http_server::Server;

fn main() {
    let mut server = Server::new("0.0.0.0:7878", 32);

    server.mount("dist", "/");

    server.api.post("/echo", |request| {
        let body = request.body.to_string();

        BaseResponse::success()
            .string(body.as_str())
            .set_content_type("application/json")
    });

    server.redirect(Methods::Get, "/", "/red");

    server.run();
}
