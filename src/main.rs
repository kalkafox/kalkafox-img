use bytes::BufMut;
use futures_util::{TryStreamExt, StreamExt};
use mongodb::{
    bson::doc,
    options::ClientOptions,
    Client,
};
use mongodb_gridfs::{options::GridFSBucketOptions, GridFSBucket};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use warp::{multipart::Part, Filter};

#[derive(Serialize, Deserialize)]
struct Post {
    id: String,
    bucket_id: mongodb::bson::oid::ObjectId,
    mime_type: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    url_prefix: String,
}

#[derive(Serialize, Deserialize)]
struct Key {
    key: String,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        println!("DATABASE_URL must be set");
        std::process::exit(1);
    });

    let port = std::env::var("PORT").unwrap_or_else(|_| "3030".to_string()).parse::<u16>().unwrap();


    // Upload endpoint that takes multipart/form-data
    let upload_route = warp::path!("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(12_000_000)) // max 12MB
        .and(warp::header::<String>("x-api-key"))
        .and_then(|form: warp::multipart::FormData, key: String| async move {
            let connection_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let mut mongodb_client_options = ClientOptions::parse(connection_env).await.unwrap();

            mongodb_client_options.app_name = Some("rust-mongodb".to_string());
            let mongodb_client = Client::with_options(mongodb_client_options).unwrap();
            let mongodb_database = mongodb_client.database("kalkafox");

            let collection = mongodb_database.collection("posts");
            let url_prefix = mongodb_database.collection::<Config>("config").find_one(None, None).await.unwrap().unwrap_or_else(|| Config { url_prefix: "https://i.kalkafox.dev".to_string() }).url_prefix;

            let keys = mongodb_database.collection::<Key>("keys");

            keys.find_one(doc! { "key": key }, None).await.unwrap().ok_or_else(|| {
                eprintln!("invalid key");
                warp::reject::reject()
            })?;


            let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
                eprintln!("form error: {}", e);
                warp::reject::reject()
            })?;

            if parts.len() > 1 {
                eprintln!("too many fields");
                return Err(warp::reject::reject());
            }

            for p in parts {
                match p.name() {
                    "data" => {
                        let mime_type = p.content_type().map(|ct| ct.to_string());

                        if mime_type.is_none() {
                            eprintln!("mime type error");
                            warp::reject::reject();
                        }

                        let mime_type = mime_type.unwrap();

                        let value = p
                            .stream()
                            .try_fold(Vec::new(), |mut vec, data| {
                                vec.put(data);
                                async move { Ok(vec) }
                            })
                            .await
                            .map_err(|e| {
                                eprintln!("reading file error: {}", e);
                                warp::reject::reject()
                            })?;

                        let rand_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

                        let mut bucket = GridFSBucket::new(mongodb_database.clone(), Some(GridFSBucketOptions::default()));
                        let id = bucket.upload_from_stream(rand_string.as_str(), value.as_slice(), None).await.unwrap();

                        let post = Post {
                            id: rand_string,
                            bucket_id: id,
                            mime_type,
                        };

                        let post_bson = mongodb::bson::to_bson(&post).unwrap();
                        let post_document = match post_bson {
                            mongodb::bson::Bson::Document(document) => document,
                            _ => panic!("Invalid BSON"),
                        };

                        collection.insert_one(post_document, None).await.unwrap();

                        return Ok::<_, warp::Rejection>(format!("{}/{}", url_prefix, post.id));
                    }
                    _ => {
                        eprintln!("unknown field");
                        return Err(warp::reject::not_found());
                    }
                }
            }

            Ok::<_, warp::Rejection>(format!("ok"))
        });

    let download_route = warp::path!(String)
        .and(warp::get())
        .and_then(|id| async move {
            let connection_env = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let mut mongodb_client_options = ClientOptions::parse(connection_env).await.unwrap();

            mongodb_client_options.app_name = Some("rust-mongodb".to_string());
            let mongodb_client = Client::with_options(mongodb_client_options).unwrap();
            let mongodb_database = mongodb_client.database("kalkafox");

            let collection = mongodb_database.collection::<Post>("posts");

            let document = collection.find_one(doc! { "id": id }, None).await.unwrap();

            if document.is_none() {
                return Err(warp::reject::not_found());
            }

            let document = document.unwrap();

            let bucket = GridFSBucket::new(mongodb_database.clone(), Some(GridFSBucketOptions::default()));

            let mut cursor = bucket.open_download_stream(document.bucket_id).await.unwrap();

            let mut buffer = Vec::new();

            while let Some(item) = cursor.next().await {
                buffer.extend_from_slice(&item);
            }

            let post_bson = mongodb::bson::to_bson(&document).unwrap();

            let post: Post = match post_bson {
                mongodb::bson::Bson::Document(document) => {
                    mongodb::bson::from_bson(mongodb::bson::Bson::Document(document)).unwrap()
                }
                _ => panic!("Invalid BSON"),
            };

            Ok::<_, warp::Rejection>(warp::reply::with_header(buffer, "Content-Type", post.mime_type))
        });

    // TODO: import react stuff, more fun with lua

    let not_found_route = warp::any()
        .map(|| warp::reply::with_status("Not Found", warp::http::StatusCode::NOT_FOUND));

    let routes = upload_route.or(download_route).or(not_found_route);

    println!("Listening on http://localhost:3030");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
