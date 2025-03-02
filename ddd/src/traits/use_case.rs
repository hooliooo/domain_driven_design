pub trait UseCase<Request, Response> {
    fn handle(&self, request: Request) -> Response;
}
