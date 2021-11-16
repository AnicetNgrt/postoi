use parking_lot::RwLock;

use crate::node::Node;

pub mod admin_controller;
pub mod websocket;
pub mod req_handler;

pub fn start_server(node: Node, address: &str) {
    let node = RwLock::new(node);

    rouille::start_server(address, move |request| req_handler::handle(request, &node));
}