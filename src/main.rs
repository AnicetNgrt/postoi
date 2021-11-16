use postoi::{node::Node, user_api::start_server};

fn main() {
    start_server(Node::init(), "0.0.0.0:8000");
}
