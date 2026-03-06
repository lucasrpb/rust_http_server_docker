use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();

    for request in server.incoming_requests() {
        let response = Response::from_string("Hello, world!");
        request.respond(response).unwrap();
    }
}