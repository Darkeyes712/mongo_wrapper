mod mongo_wrapper;
use bson::to_document;
use mongo_wrapper::MongoWrapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestMongoSingleData {
    id_: i32,
    title: String,
    author: String,
    age: i32,
    is_faggot: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestMongoData {
    id_: i32,
    title: String,
    author: String,
    age: i32,
    is_faggot: bool,
}

pub fn convert_struct_data_to_bson() -> Vec<bson::Document> {
    let struct_data: Vec<TestMongoData> = vec![
        TestMongoData {
            id_: 22,
            title: "Kurec4".to_string(),
            author: "Pesho".to_string(),
            age: 31,
            is_faggot: true,
        },
        TestMongoData {
            id_: 23,
            title: "Kurec5".to_string(),
            author: "Pesho".to_string(),
            age: 33,
            is_faggot: true,
        },
        TestMongoData {
            id_: 24,
            title: "Kurec6".to_string(),
            author: "Pesho".to_string(),
            age: 35,
            is_faggot: false,
        },
    ];
    let mut bson_documents: Vec<_> = Vec::new();
    for dat in &struct_data {
        if let Ok(document) = to_document(dat) {
            bson_documents.push(document);
        } else {
            println!("Failed to convert book data to BSON document");
        }
    }

    bson_documents
}

pub fn convert_struct_data_to_single_bson() -> Result<bson::Document, bson::ser::Error> {
    let single_doc_data: TestMongoSingleData = TestMongoSingleData {
        id_: 13,
        title: "Kurec26".to_string(),
        author: "Pesho".to_string(),
        age: 41,
        is_faggot: true,
    };

    let bson_doc = to_document(&single_doc_data)?;

    Ok(bson_doc)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_name: &str = "test_Rust_database";
    let coll_name: &str = "test_Rust_collection";
    let mut mongo_client = MongoWrapper::create_mongo_connection().await?; // initiate the mongo client
    mongo_client.set_database_and_collection_names(db_name, coll_name); // set the struct variables
    mongo_client.initiate_mongo_database().await?; // create the database and collection

    // let list_of_databases: Vec<String> = mongo_client.list_databases().await?; // get a list of all available databases
    // let list_of_collections: Vec<String> = mongo_client.list_collections().await?; // get a list of all available collections for a given database

    // Redo this with your own understanding and style.

    // mongo_client
    //     .insert_multiple_documents(convert_struct_data_to_bson())
    //     .await?; // - insert a multiple documents

    // mongo_client
    //     .insert_single_document(convert_struct_data_to_single_bson().unwrap())
    //     .await?; // - insert a single document

    // mongo_client.search_for_single_document("age", 33).await?; // find a single document using search parameters
    // mongo_client.delete_single_document("age", 33).await?; // delete a single document using search parameters
    // mongo_client
    //     .update_single_document("age", 33, "age", 40)
    //     .await?; // update a single document using search parameters

    // Redo this with your own understanding and style.

    // debugging
    // println!("{:?}", list_of_databases);
    // println!("{:?}", list_of_collections);
    println!("Data insertion complete");
    // debugging

    Ok(())
}
