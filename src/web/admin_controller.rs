use parking_lot::{RwLock};
use serde::{Deserialize, Serialize};

use rouille::{input, Request, Response};

use crate::{blockchain::Block, node::Node};

#[derive(Deserialize)]
struct ReqAddPeerBody {
    peer: String
}

pub fn req_add_peer(request: &Request, node: &RwLock<Node>) -> Response  {
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

pub fn req_peers(_request: &Request, node: &RwLock<Node>) -> Response  {
    Response::json(&ReqPeersResp { peers: &node.read().peers })
}

#[derive(Serialize)]
struct ReqBlocksResp<'a> {
    blocks: &'a Vec<Block>
}

pub fn req_blocks(_request: &Request, node: &RwLock<Node>) -> Response  {
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

pub fn req_mint_block(request: &Request, node: &RwLock<Node>) -> Response {
    match input::json_input::<ReqMintBlockBody>(request) {
        Ok(body) => Response::json(&ReqMintBlockResp { block: node.write().mint(body.data) }),
        Err(_) => Response::text("Invalid body").with_status_code(400),
    }
}
