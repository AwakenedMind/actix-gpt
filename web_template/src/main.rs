use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use reqwest::Client;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Analytics {
    page_views: u64,
    impressions: u64,
    clicks: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    analytics: HashMap<String, Analytics>,
}

impl Database {
    fn new() -> Self {
        Self {
            analytics: HashMap::new(),
        }
    }

    fn insert(&mut self, page: String, analytics: Analytics) {
        self.analytics.insert(page, analytics);
    }

    fn get(&self, page: &str) -> Option<&Analytics> {
        self.analytics.get(page)
    }

    fn get_all(&self) -> Vec<(&String, &Analytics)> {
        self.analytics.iter().collect()
    }

    fn update(&mut self, page: String, analytics: Analytics) {
        self.analytics.insert(page, analytics);
    }

    fn save_to_file(&self) -> std::io::Result<()> {
        let data: String = serde_json::to_string(&self)?;
        let mut file: fs::File = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content: String = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>
}

async fn update_analytics(app_state: web::Data<AppState>, page: web::Path<String>, analytics: web::Json<Analytics>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(page.into_inner(), analytics.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn get_analytics(app_state: web::Data<AppState>, page: web::Path<String>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get(&page.into_inner()) {
        Some(analytics) => HttpResponse::Ok().json(analytics),
        None => HttpResponse::NotFound().finish()
    }
}

async fn get_all_analytics(app_state: web::Data<AppState>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    let analytics = db.get_all();
    HttpResponse::Ok().json(analytics)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new()
    };

    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db)
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .route("/analytics/{page}", web::post().to(update_analytics))
            .route("/analytics/{page}", web::get().to(get_analytics))
            .route("/analytics", web::get().to(get_all_analytics))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}