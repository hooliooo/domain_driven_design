/// A UseCase is an object that organizes the business logic of core layer to fulfill
/// a business requirement
#[async_trait::async_trait]
pub trait UseCase: Send + Sync {
    /// The input of the UseCase
    type Request;

    /// The output of the UseCase
    type Response;

    /// Executes business logic within the core layer to fulfill a business requirement
    async fn handle(&self, request: Self::Request) -> Self::Response;
}

mod test {
    use std::sync::Arc;

    use crate::traits::use_case::UseCase;

    struct TestUseCase;
    struct TestRequest {
        pub name: String,
    }
    struct TestResponse {
        pub name: String,
    }
    struct TestError {
        pub error: String,
    }

    #[async_trait::async_trait]
    impl UseCase for TestUseCase {
        type Request = TestRequest;
        type Response = Result<TestResponse, TestError>;

        async fn handle(&self, request: TestRequest) -> Result<TestResponse, TestError> {
            Ok(TestResponse {
                name: "test".to_string(),
            })
        }
    }

    struct TestState {
        pub use_case: Arc<
            dyn UseCase<Request = TestRequest, Response = Result<TestResponse, TestError>>
                + Send
                + Sync,
        >,
    }
}
