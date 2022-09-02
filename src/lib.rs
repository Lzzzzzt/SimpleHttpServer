mod utils;

use log::{error, info};
use methods::Methods;
use request::Request;
use response::{BaseResponse, Response};
use std::collections::HashMap;
use std::fs;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use thread_pool::ThreadPool;

type RouteFn = Box<dyn Fn(Request) -> Response + Send + Sync + 'static>;
type Routes = Arc<RwLock<HashMap<String, RouteFn>>>;

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
        info!("Simple HTTP Server start running\n");
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

    pub fn redirect(&self, _target: &str, origin: &str) {
        let routes = self.api.routes.read().unwrap();

        if routes.contains_key(origin) {
        } else {
            error!("origin url {} is not in route table", origin);
        }
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

    fn handle_connection(mut stream: TcpStream, routes: Routes) {
        let mut buffer = [0; 1024];

        let _ = stream.read(&mut buffer).unwrap();

        let request = Request::parse(&buffer);

        let response = match request.request_line.method {
            Methods::Get => match routes.read().unwrap().get(&request.request_line.url) {
                None => Self::target_not_found(Methods::Get, &request.request_line.url)(),
                Some(res) => {
                    info!("GET {} 200 OK", request.request_line.url);
                    res(request)
                }
            },
            // TODO
            Methods::Post => {
                println!("{:#?}", request);
                Self::target_not_found(request.request_line.method, &request.request_line.url)()
            }
        } as Response;

        Self::send_response(stream, response).unwrap();
    }

    fn target_not_found(
        methods: Methods,
        target: &str,
    ) -> Box<dyn Fn() -> Response + Send + Sync + 'static> {
        error!("{} {} 404 NOT FOUND", methods, target);
        Box::new(|| BaseResponse::client_error().not_found())
    }

    fn send_response(mut stream: TcpStream, mut response: Response) -> Result<(), Error> {
        let _ = stream.write(&response.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Api {
    routes: Arc<RwLock<HashMap<String, RouteFn>>>,
}

impl Api {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get<F>(&mut self, route: &str, f: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.routes
            .write()
            .unwrap()
            .insert(route.to_string(), Box::new(f));

        info!("Add '{}' to Route Table", route);
    }
}
