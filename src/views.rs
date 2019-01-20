use actix_web::{ HttpRequest, HttpResponse, middleware::session::{ RequestSession },
                 AsyncResponder, HttpMessage, error, Error as HttpResponseErr, FutureResponse };
use futures::{ Future, future::result as FutResult, Stream };
use serde_derive::{ Deserialize, Serialize };
use bytes::BytesMut;

use crate::block_types::{ Block, BlockChain, Transaction };
use crate::utils::Block_Chain;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

pub fn new_transaction(req: HttpRequest) -> FutureResponse<HttpResponse, HttpResponseErr> {
    req.json().from_err()
        .and_then(move |tran: Transaction| {
            //let new_tran = Transaction::new(&res);
            println!("transaction: {:?}", tran);
            Ok(HttpResponse::Ok().json(tran))
    }).responder()
}


pub fn mine(req: HttpRequest) -> FutureResponse<HttpResponse, HttpResponseErr> {
    unimplemented!();
}


pub fn full_chain(req: HttpRequest) -> FutureResponse<HttpResponse, HttpResponseErr> {
    let ch = req.session().get::<BlockChain>("block_chain");
    if let Ok(Some(chains)) = ch {
        FutResult(Ok(HttpResponse::Ok().json(chains))).responder()
    } else {
        req.payload().from_err()
            .fold(BytesMut::new(), move |mut body, chunk| {
                // limit max size of in-memory payload
                if (body.len() + chunk.len()) > MAX_SIZE {
                    Err(error::ErrorBadRequest("overflow"))
                } else {
                    body.extend_from_slice(&chunk);
                    Ok(body)
                }
            })
            .and_then(move |body| {
                println!("returning full block!");
                let block = BlockChain::default();
                let _ = req.session().set("block_chain", &block);
                Ok(HttpResponse::Ok().json(block))  // <- send block chain
            })
            .responder()
    }
} 