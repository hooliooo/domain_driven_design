/// A UseCase is an object that organizes the business logic of core layer to fulfill
/// a business requirement
pub trait UseCase: Send + Sync {
    /// The input of the UseCase
    type Request;

    /// The output of the UseCase
    type Response;

    /// Executes business logic within the core layer to fulfill a business requirement
    fn handle(&self, request: Self::Request) -> impl Future<Output = Self::Response> + Send;
}
