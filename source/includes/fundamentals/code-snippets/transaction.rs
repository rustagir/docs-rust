use std::env;
use bson::{ Document, doc };
use futures::FutureExt;
use mongodb::{ Client, ClientSession };
use mongodb::error::Error;

// begin-callback
async fn insert_media(session: &mut ClientSession) -> Result<(), Error> {
    let books_coll = session.client().database("db").collection::<Document>("books");
    let films_coll = session.client().database("db").collection::<Document>("films");

    books_coll.insert_one_with_session(
        doc! { "name": "The Bluest Eye".to_string(), "author": "Toni Morrison".to_string() },
        None,
        session
    ).await?;

    films_coll.insert_one_with_session(
        doc! { "name": "Nostalgia".to_string(), "year": 1983 },
        None,
        session
    ).await?;

    println!("Successfully committed transaction!");
    Ok(())
}
// end-callback

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let client = Client::with_uri_str(uri).await?;

    // begin-session
    let mut session = client.start_session(None).await?;
    session.with_transaction((), |session, _| insert_media(session).boxed(), None).await?;
    // end-session

    Ok(())
}
