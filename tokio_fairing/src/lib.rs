extern crate rocket;
extern crate tokio_core;

use rocket::fairing;

struct TokioFairing {
    core: tokio_core::reactor::Remote,
}

impl TokioFairing {
    fn new() -> Self {
        let core = tokio_core::reactor::Core::new().unwrap();

        TokioFairing {
            core: core.remote(),
        }
    }
}

impl fairing::Fairing for TokioFairing {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "tokio core eventloop fairing",
            kind: fairing::Kind::Request | fairing::Kind::Response,
        }
    }

    fn on_request(&self, request: &mut rocket::Request, data: &rocket::Data) {
        let handle = self.core.handle().unwrap();
    }
    fn on_response(&self, request: &rocket::Request, response: &mut rocket::Response) {
    }
}
