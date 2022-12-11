use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub type XBlockSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

async fn index(schema: web::Data<StarWarsSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000")
                .finish(),
        ))
}

fn app(schema: XBlockSchema) -> App {
    #[async_trait]
pub trait Chain {
    async fn create_block() -> Result<()>;
    fn supports_contracts() -> bool;
}

pub enum InterfaceEvent { 
    CreateBlock {
        nonce: u64,
        data: Vec<u8>,
    },
    TransferFunds {
        account: uuid::Uuid,
    },
    CreateContract {
        contract: Vec<u8>,
        responder: tokio::sync::oneshot::Sender<uuid::Uuid>,
    },
    InvokeContract {
        contract: uuid::Uuid,
        args: Vec<u8>,
        responder: tokio::sync::oneshot::Sender<Result<Vec<u8>>>
    }
}
    App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
}