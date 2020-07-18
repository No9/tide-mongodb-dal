use async_std::stream::StreamExt;
use mongodb::{
    bson::{Document},
    Client,
};
use std::error;

pub async fn get(client : &Client, db_name: &str, collection_name : &str) -> Result<Vec<Document>, Box<dyn error::Error>> {
    
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}