use std::error::Error;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::net::SocketAddr;
use std::fs::OpenOptions;

use clap::Parser;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
use log::info;

use rust_graph::apis::health::serve_health_apis;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Address to start the database on
    #[arg(short, long)]
    addr : u16,
    /// Path to log to, if left empty logs to stdout
    #[arg(short,long)]
    log_path : String,
    /// Path to database storage, if empty defaults to $(PWD)/data
    #[arg(short, long)]
    store : String
}

use slog::Drain;

#[derive(Debug)]
struct Server{
    addr : u16,
    _root_logger : slog::Logger
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let s = Server{
        addr : args.addr,
        _root_logger : setup_logger(Some(args.log_path.as_str())).unwrap(),
    };

    info!("starting server on port : {}", s.addr);

    let app = serve_health_apis(
            Router::new()
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], s.addr));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn root() -> &'static str {
    "Rust Graph"
}

fn setup_logger(filename : Option<&str>) -> Option<slog::Logger>{
    match filename {
        Some(log_path) => {
            let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(log_path)
            .unwrap();

            let decorator = slog_term::PlainDecorator::new(file);
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = slog_async::Async::new(drain).build().fuse();

            Some(slog::Logger::root(drain, o!()))
        },
        None => {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = slog_async::Async::new(drain).build().fuse();

            Some(slog::Logger::root(drain, o!()))
        }
    }
}