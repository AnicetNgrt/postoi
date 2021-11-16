use parking_lot::RwLock;
use rouille::{Request, Response, router};

use crate::node::Node;
use super::admin_controller;

pub fn handle(request: &Request, node: &RwLock<Node>) -> Response {
    router!(request,
        (GET) (/) => {
            Response::text("Hello")
        },
        (GET) (/blocks) => {
            admin_controller::req_blocks(request, &node)
        },
        (POST) (/mintBlock) => {
            admin_controller::req_mint_block(request, &node)
        },
        (GET) (/peers) => {
            admin_controller::req_peers(request, &node)
        },
        (POST) (/addPeer) => {
            admin_controller::req_add_peer(request, &node)
        },
        _ => Response::empty_404()
    )
}