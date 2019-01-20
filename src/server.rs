use actix_web::{ server, App, middleware, 
                 http::{ self, NormalizePath },
                 middleware::session::SessionStorage
};
use actix_redis::RedisSessionBackend;

use crate::{ block_types::*,
    views,
    utils::{ DBPool, DBState, establish_connection, server_config, load_ssl },
};

use std::{ env, fs::File, path::Path };

pub fn block_chain_server() {
    let config = server_config().unwrap();
    
    let address = config["address"].as_str().unwrap();
    let port = config["port"].as_integer().unwrap();
    let mut workers = config["workers"].as_integer().unwrap() as usize;
    let log_level = config["log"].as_str().unwrap();

    env::set_var("RUST_LOG", format!("actix_web={}", log_level)); // log level
    env_logger::init(); // init a log
    
    let sys = actix::System::new("block_chain"); // start a system
    workers = num_cpus::get();
    
    let chain_server = server::new(move || {
        vec![
            App::new()
                .middleware(SessionStorage::new(
                    // redis is running on local server with port 6379
                    RedisSessionBackend::new("127.0.0.1:6379", &[0;32])
                    .cookie_secure(false)
                    .cookie_max_age(chrono::Duration::days(30)) // session will expire after 30 days
                ))
                .scope("/", |scope| {
                    scope.default_resource(|r| r.h(NormalizePath::default())) // normalize the path
                    .resource("/new_transaction/", |r| {
                        r.method(http::Method::POST).with(views::new_transaction);
                    })
                    .resource("/mine/", |r| {
                        r.method(http::Method::POST).with(views::mine);
                    })
                    .resource("/chain/", |r| {
                        r.method(http::Method::GET).with(views::full_chain);
                    })
                })
        ]
    });
    
    chain_server.bind_ssl(format!("{}:{}", &address, &port), load_ssl()).unwrap().start();
    println!("Started http server: {}", format!("{}:{}", &address, &port));
    let _ = sys.run();
}
