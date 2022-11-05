use session::{
    session_server::{Session, SessionServer},
    GetHealthRequest, GetHealthResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod session {
    tonic::include_proto!("server"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct SessionService {}

#[tonic::async_trait]
impl Session for SessionService {
    async fn get_health(
        &self,
        request: Request<GetHealthRequest>,
    ) -> Result<Response<GetHealthResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GetHealthResponse {};

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let session_service = SessionService::default();

    Server::builder()
        .add_service(SessionServer::new(session_service))
        .serve(addr)
        .await?;

    Ok(())
}
