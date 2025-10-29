use dotenvy::dotenv;
use mongodb::{bson::doc, sync::Client};
use std::env;

const DEFAULT_MONGODB_HOST: &str = "cluster0.at4kkos.mongodb.net";
const DEFAULT_MONGODB_APPNAME: &str = "Cluster0";

pub struct ConnectionMongodb {
    link: String,
}

impl ConnectionMongodb {
    // Constructor con enlace por defecto
    pub fn new() -> Self {
        // Carga variables de entorno desde .env si existe
        let _ = dotenv();

        let username = env::var("MONGODB_USER").expect("MONGODB_USER no está definido en .env");
        let password =
            env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD no está definido en .env");

        let link = format!(
            "mongodb+srv://{}:{}@{}/?appName={}",
            username, password, DEFAULT_MONGODB_HOST, DEFAULT_MONGODB_APPNAME
        );

        ConnectionMongodb { link }
    }

    // Constructor personalizado por si se necesita otro enlace
    /*pub fn with_connection(link: String) -> Self {
        ConnectionMongodb { link }
    }*/

    // Método de instancia
    pub fn connection(&self) -> mongodb::error::Result<Client> {
        let uri = &self.link;
        // Create a new client and connect to the server
        let client = Client::with_uri_str(uri)?;
        // Send a ping to confirm a successful connection
        client
            .database("belador_db")
            .run_command(doc! { "ping": 1 })
            .run()?;
        println!("Pinged your deployment. You successfully connected to MongoDB!");
        Ok(client)
    }
}
