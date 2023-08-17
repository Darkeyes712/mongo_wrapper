use bson::Document;
use mongodb::{
    bson::doc, options::ClientOptions, options::CreateCollectionOptions, options::FindOneOptions,
    Client, Collection,
};
use std::error::Error;

pub struct MongoWrapper {
    client: Client,
    database_name: String,
    collection_name: String,
}

impl MongoWrapper {
    pub async fn create_mongo_connection() -> Result<Self, Box<dyn std::error::Error>> {
        let uri_string = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(uri_string)?;

        Ok(Self {
            client,
            database_name: String::new(),
            collection_name: String::new(),
        })
    }

    pub fn set_database_and_collection_names(&mut self, db_name: &str, coll_name: &str) {
        self.database_name = db_name.to_string();
        self.collection_name = coll_name.to_string();
    }

    pub async fn initiate_mongo_database(&self) -> Result<(), Box<dyn std::error::Error>> {
        let database_exists = self.database_exists().await;

        if !database_exists {
            self.create_database().await?;
            println!("Database '{}' created", &self.database_name);
        }

        if !self.collection_exists(&self.collection_name).await {
            self.create_collection().await?;
            println!("Collection '{}' created", &self.collection_name);
        }

        if database_exists {
            println!(
                "Database '{}' and collection '{}' already exist",
                &self.database_name, &self.collection_name
            );
        }

        Ok(())
    }

    async fn create_database(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db_exists = self.database_exists().await;

        if !db_exists {
            let db_options = CreateCollectionOptions::default();
            self.client
                .database(&self.database_name)
                .create_collection(&self.collection_name, db_options)
                .await?;
        }

        Ok(())
    }

    async fn create_collection(&self) -> Result<(), Box<dyn std::error::Error>> {
        let database = self.client.database(&self.database_name);
        let coll_options = CreateCollectionOptions::default();

        match database
            .create_collection(&self.collection_name, coll_options)
            .await
        {
            Ok(_) => {
                println!("Collection '{}' created", &self.collection_name);
                println!(
                    "Database '{}' and collection '{}' created",
                    &self.database_name, &self.collection_name
                );
            }
            Err(err) if err.to_string().contains("Collection already exists") => {
                println!("Collection '{}' already exists", &self.collection_name);
            }
            Err(err) => return Err(Box::new(err)),
        }

        Ok(())
    }

    async fn database_exists(&self) -> bool {
        let db_names = self
            .client
            .list_database_names(None, None)
            .await
            .unwrap_or_default();
        db_names.contains(&self.database_name)
    }

    async fn collection_exists(&self, coll_name: &str) -> bool {
        let database = self.client.database(&self.database_name);
        let collections = database
            .list_collection_names(None)
            .await
            .unwrap_or_default();
        collections.contains(&coll_name.to_string())
    }

    pub async fn list_databases(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let databases = self.client.list_database_names(None, None).await?;
        Ok(databases)
    }

    pub async fn list_collections(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let database = self.client.database(&self.database_name);
        let collections = database.list_collection_names(None).await?;
        Ok(collections)
    }

    pub async fn insert_multiple_documents(
        &self,
        data: Vec<bson::Document>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let collection_handle = self
            .client
            .database(&self.database_name)
            .collection(&self.collection_name);

        collection_handle.insert_many(data, None).await?;

        Ok(())
    }

    // we can change that here so that we pass the data s a parameter and call the function in the main.rs file..
    pub async fn insert_single_document(
        &self,
        data: Document,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // let bson_doc = self.convert_struct_data_to_single_bson()?;
        let collection_handle = self
            .client
            .database(&self.database_name)
            .collection(&self.collection_name);

        collection_handle.insert_one(data, None).await?;

        Ok(())
    }

    pub async fn search_for_single_document(
        &self,
        key: &str,
        value: i32,
    ) -> Result<Document, Box<dyn Error>> {
        let collection_handle: Collection<Document> = self
            .client
            .database(&self.database_name)
            .collection(&self.collection_name);

        let find_options = FindOneOptions::default();
        if let Some(document) = collection_handle
            .find_one(doc! { key: value }, find_options)
            .await?
        {
            println!("Found document: {:?}", document); // Print the document
            Ok(document)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No matching document found",
            )))
        }
    }

    pub async fn delete_single_document(
        &self,
        key: &str,
        value: i32,
    ) -> Result<(), Box<dyn Error>> {
        let collection_handle: Collection<Document> = self
            .client
            .database(&self.database_name)
            .collection(&self.collection_name);

        if let Ok(document_to_delete) = self.search_for_single_document(key, value).await {
            let filter = doc! { key: value };
            collection_handle.delete_one(filter, None).await?;
            println!("Deleted document: {:?}", document_to_delete);
        } else {
            println!("No document found matching that criteria.");
        }

        Ok(())
    }

    pub async fn update_single_document(
        &self,
        key: &str,
        value: i32,
        new_key: &str,
        new_value: i32,
    ) -> Result<(), Box<dyn Error>> {
        let collection_handle: Collection<Document> = self
            .client
            .database(&self.database_name)
            .collection(&self.collection_name);

        if let Ok(document_to_update) = self.search_for_single_document(key, value).await {
            let filter = doc! { key: value };
            let update_value = doc! {"$set": {new_key: new_value}};

            collection_handle
                .update_one(filter, update_value, None)
                .await?;
            println!("Updated document: {:?}", document_to_update);
        } else {
            println!("No document found matching that criteria.");
        }

        Ok(())
    }
}
