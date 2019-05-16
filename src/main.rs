#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::Json;
use std::sync::Mutex;
use rocket::State;
use rocket::http::Status;


struct Transaction {
    amount: f32,
    timestamp: i64
}

#[derive(Serialize)]
struct Statistics {
    avg: f32,
    count: i64,
    min: f32,
    max: f32,
    sum: f32
}

struct StatsManager {
    position: i32,
    stats_record: Vec<Statistics>
}

struct SafeStatsManager {
    manager: Mutex<StatsManager>
}

impl StatsManager {
    pub fn get_summary(&self) -> Statistics {
        Statistics {
            avg: 0.0,
            count: 0,
            min: 0.0,
            max: 0.0,
            sum: 0.0
        }
    }
}

impl SafeStatsManager {
    pub fn new() -> SafeStatsManager {
        SafeStatsManager {
            manager: Mutex::new(StatsManager {
                position: 0,
                stats_record: Vec::with_capacity(60)
            })
        }
    }

    pub fn get_summary(&self) -> Statistics {
        let data = self.manager.lock().unwrap();

        data.get_summary()
    }
}


#[post("/transaction")]
fn transaction(manager: State<SafeStatsManager>) -> Status {
    Status::raw(200)
}

#[get("/statistics")]
fn stats(manager: State<SafeStatsManager>) -> Json<Statistics> {
    Json(manager.get_summary())
}

fn main() {
    let mut min_max_bucket : Vec<(f32, f32)> = Vec::with_capacity(60);


    rocket::ignite().mount("/", routes![stats, transaction])
        .manage(SafeStatsManager::new())
        .launch();
}
