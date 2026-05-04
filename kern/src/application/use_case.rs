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

#[cfg(test)]
mod test {
    use std::{borrow::Cow, sync::Arc};

    use crate::application::use_case::UseCase;

    struct TestUseCase;
    struct TestRequest {
        pub name: Cow<'static, str>,
    }
    struct TestResponse {
        pub name: Cow<'static, str>,
    }

    #[derive(Debug)]
    struct TestError {
        pub error: String,
    }

    #[async_trait::async_trait]
    impl UseCase for TestUseCase {
        type Request = TestRequest;
        type Response = Result<TestResponse, TestError>;

        async fn handle(&self, request: TestRequest) -> Result<TestResponse, TestError> {
            Ok(TestResponse { name: request.name })
        }
    }

    struct TestState {
        pub use_case: Arc<
            dyn UseCase<Request = TestRequest, Response = Result<TestResponse, TestError>>
                + Send
                + Sync,
        >,
    }

    #[tokio::test]
    async fn test() {
        let use_case = TestUseCase;
        let state = TestState {
            use_case: Arc::new(use_case),
        };

        let handle = tokio::spawn(async move {
            let request = TestRequest {
                name: Cow::Borrowed("test"),
            };
            state.use_case.handle(request).await
        });

        match handle.await.unwrap() {
            Ok(response) => assert_eq!(Cow::Borrowed("test"), response.name),
            Err(err) => {
                let error = err.error;
                let message = format!("Test failed: {}", error);
                panic!("{message}")
            }
        };
    }
}
