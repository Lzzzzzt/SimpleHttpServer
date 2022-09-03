use std::collections::HashMap;
use std::fs;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

use log::{debug, error, info};
use request::Request;
use response::{BaseResponse, Response};
use thread_pool::ThreadPool;

use methods::Methods;

mod utils;

type RouteFn = Box<dyn Fn(Request) -> Response + Send + Sync + 'static>;

pub struct Server {
    address: String,
    listener: TcpListener,
    pool: ThreadPool,
    pub api: Api,
}

impl Server {
    pub fn new(addr: &str, thread_num: usize) -> Self {
        utils::init_logger();

        Self {
            address: addr.to_string(),
            listener: TcpListener::bind(addr).unwrap(),
            pool: ThreadPool::new(thread_num),
            api: Api::new(),
        }
    }

    pub fn run(&self) {
        info!("Simple HTTP Server start running");
        info!("Start listening on {}", self.address);

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            let routes = self.api.routes.clone();

            self.pool.execute(|| {
                Self::handle_connection(stream, routes);
            });
        }
    }

    pub fn mount(&mut self, static_dir_path: &str, mount_point: &str) {
        let path = static_dir_path.to_string();

        let result = fs::read_dir(static_dir_path);

        if result.is_ok() {
            let mut root = utils::make_root_path(mount_point);

            info!(
                "Mount Static Directory From '{}' To '{}'",
                static_dir_path, &root
            );

            self.api.get(&root, move |_| {
                BaseResponse::success()
                    .file(&format!("{}/index.html", path))
                    .unwrap_or_else(|_| BaseResponse::client_error().not_found())
            });

            root.pop();

            self.walk_dir(static_dir_path, mount_point);
        } else {
            error!("{}", result.err().unwrap());
        }
    }

    pub fn redirect(&mut self, method: Methods, origin: &str, target: &str) {
        let origin = origin.to_string();

        self.api.response(method, target, move |_| {
            BaseResponse::redirect().temporary(&origin)
        });
    }

    fn walk_dir(&mut self, dir_path: &str, root_path: &str) {
        let dir = fs::read_dir(dir_path).unwrap();

        dir.for_each(|d| {
            let file_or_dir = d.unwrap();

            if file_or_dir.file_type().unwrap().is_dir() {
                self.walk_dir(file_or_dir.path().to_str().unwrap(), root_path);
            } else {
                let file_path = file_or_dir.path().to_str().unwrap().to_string();

                let mut root = utils::make_root_path(root_path);

                root.push_str(&file_path.split('/').collect::<Vec<&str>>()[1..].join("/"));

                self.api.get(&root, move |_| {
                    BaseResponse::success()
                        .file(&file_path)
                        .unwrap_or_else(|_| BaseResponse::client_error().not_found())
                });
            }
        });
    }

    fn handle_connection(mut stream: TcpStream, routes: RouteTable) {
        let mut buffer = [0; 1024];

        let length = stream.read(&mut buffer).unwrap();

        let request = Request::parse(&buffer[0..length]);

        let request_method = &request.request_line.method;
        let request_url = &request.request_line.url;

        let response = match routes.get(request_method).read().unwrap().get(request_url) {
            None => Self::target_not_found(request_method, request_url)(),
            Some(res) => {
                let method = request.request_line.method.to_string();
                let url = request.request_line.url.clone();
                let response = res(request) as Response;
                info!("{} {} {}", method, url, response.message());
                response
            }
        } as Response;

        Self::send_response(stream, response).unwrap();
    }

    fn target_not_found(
        methods: &Methods,
        target: &str,
    ) -> Box<dyn Fn() -> Response + Send + Sync + 'static> {
        error!("{} {} 404 NOT FOUND", methods, target);
        Box::new(|| BaseResponse::client_error().not_found())
    }

    fn send_response(mut stream: TcpStream, mut response: Response) -> Result<(), Error> {
        stream.write_all(&response.as_bytes())?;
        stream.flush()?;

        Ok(())
    }
}

#[derive(Default)]
pub struct RouteTable {
    get: Arc<RwLock<HashMap<String, RouteFn>>>,
    post: Arc<RwLock<HashMap<String, RouteFn>>>,
}

impl RouteTable {
    pub fn new() -> Self {
        Self {
            get: Arc::new(RwLock::new(HashMap::new())),
            post: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, method: &Methods) -> Arc<RwLock<HashMap<String, RouteFn>>> {
        match method {
            Methods::Get => Arc::clone(&self.get),
            Methods::Post => Arc::clone(&self.post),
        }
    }
}

impl Clone for RouteTable {
    fn clone(&self) -> Self {
        Self {
            get: Arc::clone(&self.get),
            post: Arc::clone(&self.post),
        }
    }
}

#[derive(Default)]
pub struct Api {
    routes: RouteTable,
}

impl Api {
    pub fn new() -> Self {
        Self {
            routes: RouteTable::new(),
        }
    }

    pub fn get<F>(&mut self, route: &str, f: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.response(Methods::Get, route, f);
    }

    pub fn post<F>(&mut self, route: &str, f: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.response(Methods::Post, route, f);
    }

    pub fn response<F>(&mut self, method: Methods, route: &str, f: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.routes
            .get(&method)
            .write()
            .unwrap()
            .insert(route.to_string(), Box::new(f));

        debug!("{}: Add {} to Route Table", method, route);
    }
}
