use postoi::{node::Node, web};

fn main() {
    web::start_server(Node::init(), "0.0.0.0:8000");
}
