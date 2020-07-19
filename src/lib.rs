use async_std::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client,
};
use std::error;

pub async fn get(
    client: &Client,
    db_name: &str,
    collection_name: &str,
) -> Result<Vec<Document>, Box<dyn error::Error>> {
    let db = client.database(db_name);
    let coll = db.collection(collection_name);
    let mut cursor = coll.find(None, None).await?;
    let mut docs: Vec<Document> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                docs.push(document);
            }
            Err(e) => println!("Err {}", e),
        }
    }
    Ok(docs)
}

pub async fn get_by_id(
    client: &Client,
    db_name: &str,
    collection_name: &str,
    objid: &str,
) -> Result<Document, Box<dyn error::Error>> {
    let db = client.database(db_name);
    let coll = db.collection(collection_name);
    let object_id = ObjectId::with_string(objid).unwrap();
        
    let filter = doc! { "_id": object_id };
    let mut cursor = coll.find(filter, None).await?;

    let mut doc = Document::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                doc = document;
                println!("DAL: {}", doc);
            }
            Err(e) => println!("Err {}", e),
        }
    }
    Ok(doc)
}

pub async fn insert_one(
    client: &Client,
    db_name: &str,
    collection_name: &str,
    doc: Document,
) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
    let db = client.database(db_name);
    let coll = db.collection(collection_name);
    coll.insert_one(doc, None).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
