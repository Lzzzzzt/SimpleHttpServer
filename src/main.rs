use response::Response;
use simple_http_server::Server;

fn main() {
    let mut server = Server::new("0.0.0.0:7878", 16);

    server.mount("dist", "/");

    server.redirect("/test2", "/111");

    server.api.get("/test", |req| {
        println!("{}", req);
        Response::file_response("dist/index.html").unwrap()
    });

    server.run();
}


