mod mongo_wrapper;
use mongo_wrapper::MongoWrapper;
use serde_json::from_str;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_name: &str = "test_Rust_database";
    let coll_name: &str = "test_Rust_collection";
    let mut mongo_client = MongoWrapper::create_mongo_connection().await?; // initiate the mongo client
    mongo_client.set_database_and_collection_names(db_name, coll_name); // set the struct variables
    mongo_client.initiate_mongo_database().await?; // create the database and collection

    let list_of_databases: Vec<String> = mongo_client.list_databases().await?; // get a list of all available databases
    let list_of_collections: Vec<String> = mongo_client.list_collections().await?; // get a list of all available collections for a given database

    // Redo this with your own understanding and style.
    let json_data = fs::read_to_string("test_json_data.json").expect("Failed to read JSON file");
    let data: serde_json::Value = from_str(&json_data).expect("Failed to deserialize JSON data");

    if let serde_json::Value::Array(groups) = data.get("groups").unwrap() {
        let mut bson_documents = Vec::new();

        for group in groups {
            for (_, group_data) in group.as_object().unwrap() {
                if let Ok(group_document) = mongodb::bson::to_document(group_data) {
                    bson_documents.push(group_document);
                } else {
                    println!("Failed to convert group data to BSON document");
                }
            }
        }

        mongo_client.insert_json_data(bson_documents).await.unwrap();

        println!("Data insertion complete");
    } else {
        println!("Invalid JSON format");
    }
    // Redo this with your own understanding and style.

    // debugging
    println!("{:?}", list_of_databases);
    println!("{:?}", list_of_collections);
    println!("Data insertion complete");
    // debugging

    Ok(())
}
