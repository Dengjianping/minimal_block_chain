use actix_web::{ Error, HttpResponse, FutureResponse, AsyncResponder };
use diesel::{ prelude::*, pg::PgConnection, r2d2::{ ConnectionManager, Pool }};
use dotenv::dotenv;
use futures::future::result;
use openssl::ssl::{ SslMethod, SslAcceptor, SslFiletype, SslAcceptorBuilder };
use lazy_static::lazy_static;
use std::{ fs::File, path::Path, io::{ BufReader, prelude::* } };

use crate::block_types::BlockChain;

static DATABASE_URL: &'static str = env!("DATABASE_URL");
type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref Block_Chain: BlockChain = BlockChain::default();
}

pub struct DBPool {
    pub conn: PgConnection,
}

pub struct DBState {
    pub db: actix::Addr<DBPool>,
}

impl actix::Actor for DBPool {
    type Context = actix::SyncContext<Self>;
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = DATABASE_URL;
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


// create a db pool
pub fn db_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("failed to create a db pool")
}


pub fn load_ssl() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("ssl_keys/server.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("ssl_keys/crt.pem").unwrap();
    builder
}


// read project config file
pub fn server_config() -> Option<toml::Value> {
    let config = File::open(concat!(env!("CARGO_MANIFEST_DIR"),"/block_chain.toml")).unwrap();
    let mut buff = BufReader::new(config);
    let mut contents = String::new();
    buff.read_to_string(&mut contents).unwrap();
    
    let value = contents.parse::<toml::Value>().unwrap();
    Some(value["production"].clone())
}