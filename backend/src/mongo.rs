use mongodb::{ options::ClientOptions, Client };
use crate::generated_types::Node;

pub async fn get_nodes(db: &mongodb::Database) -> Vec<Node> {
    let node_collection = db.collection::<Node>("nodes");

    let mut node_cursor = match node_collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(err) => {
            eprintln!("Failed to execute find: {}", err);
            return vec![];
        }
    };

    let mut nodes = Vec::new();

    while let Some(result) = node_cursor.next().await {
        match result {
            Ok(node) => nodes.push(node),
            Err(err) => eprintln!("Failed to deserialize node: {}", err),
        }
    }

    nodes
}

pub async fn return_db(db_uri: String) -> mongodb::Database {
    let client_options = ClientOptions::parse(db_uri).await;

    match client_options {
        Ok(client_options) => {
            // Set the server_api field of the client_options object to Stable API version 1
            // let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
            // client_options.server_api = Some(server_api);
            // Get a handle to the cluster
            let client = Client::with_options(client_options).unwrap();

            client.database("admin")
        }
        Err(e) => panic!("Error connecting to MongoDB: {:?}", e),
    }
}
