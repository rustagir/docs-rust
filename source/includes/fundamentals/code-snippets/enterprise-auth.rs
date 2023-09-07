use mongodb::{ bson::doc, options::{ ClientOptions, Credential, AuthMechanism }, Client };

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let uri = "<connection string>";
    let mut client_options = ClientOptions::parse(uri).await?;

    // start-ldap
    let scram_sha_256_cred = Credential::builder()
        .username("<username>".to_string())
        .password("<password>".to_string())
        .mechanism(AuthMechanism::Plain)
        .build();

    client_options.credential = Some(scram_sha_256_cred);
    let client = Client::with_options(client_options)?;
    // end-ldap

    Ok(())
}