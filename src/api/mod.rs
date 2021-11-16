use parking_lot::{RwLock};
use serde::{Deserialize, Serialize};

use rouille::{input, router, Request, Response};

use crate::{blockchain::Block, node::Node};

pub fn start(node: Node) {
    let node = RwLock::new(node);

    rouille::start_server("0.0.0.0:80", move |request| {
        router!(request,
            (GET) (/) => {
                Response::text("Hello")
            },
            (GET) (/blocks) => {
                req_blocks(request, &node)
            },
            (POST) (/mintBlock) => {
                req_mint_block(request, &node)
            },
            (GET) (/peers) => {
                req_peers(request, &node)
            },
            (POST) (/addPeer) => {
                req_add_peer(request, &node)
            },
            _ => Response::empty_404()
        )
    });
}

#[derive(Deserialize)]
struct ReqAddPeerBody {
    peer: String
}

fn req_add_peer(request: &Request, node: &RwLock<Node>) -> Response  {
    match input::json_input::<ReqAddPeerBody>(request) {
        Ok(body) => {
            node.write().connect_to_peer(body.peer);
            Response::text("")
        },
        Err(_) => Response::text("Invalid body").with_status_code(400),
    }
}

#[derive(Serialize)]
struct ReqPeersResp<'a> {
    peers: &'a Vec<String>
}

fn req_peers(_request: &Request, node: &RwLock<Node>) -> Response  {
    Response::json(&ReqPeersResp { peers: &node.read().peers })
}

#[derive(Serialize)]
struct ReqBlocksResp<'a> {
    blocks: &'a Vec<Block>
}

fn req_blocks(_request: &Request, node: &RwLock<Node>) -> Response  {
    Response::json(&ReqBlocksResp { blocks: &node.read().blockchain.blocks })
} 

#[derive(Deserialize)]
struct ReqMintBlockBody {
    data: String,
}

#[derive(Serialize)]
struct ReqMintBlockResp<'a> {
    block: &'a Block,
}

fn req_mint_block(request: &Request, node: &RwLock<Node>) -> Response {
    match input::json_input::<ReqMintBlockBody>(request) {
        Ok(body) => Response::json(&ReqMintBlockResp { block: node.write().mint(body.data) }),
        Err(_) => Response::text("Invalid body").with_status_code(400),
    }
}
