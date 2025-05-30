use crate::models::user::{CreateUser, User};
use actix_web::{web, HttpResponse};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

static COUNTER: AtomicI32 = AtomicI32::new(1);
static USERS: Mutex<BTreeMap<i32, User>> = Mutex::new(BTreeMap::new());

pub async fn create_user(user: web::Json<CreateUser>) -> HttpResponse {
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);

    let new_user = User {
        id: count,
        username: user.username.clone(),
        password: user.password.clone(),
    };
    USERS.lock().unwrap().insert(count, new_user.clone());
    HttpResponse::Created().json(new_user)
}

/*pub async fn get_user(user_id: web::Path<i32>) -> HttpResponse {
    match USERS.lock().unwrap().get(&user_id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}*/

pub async fn get_users() -> HttpResponse {
    HttpResponse::Ok().json(USERS.lock().unwrap().values().collect::<Vec<_>>())
}

/*pub async fn update_user(user: web::Json<User>) -> HttpResponse {
    let get_user = USERS.lock().unwrap().contains_key(&user.id);
    if get_user {
        HttpResponse::Ok().json(USERS.lock().unwrap().insert(user.id, user.clone()));
    }
    HttpResponse::NotFound().body("User not found")
}*/

pub async fn delete_user(user_id: web::Path<i32>) -> HttpResponse {
    match USERS.lock().unwrap().remove(&user_id) {
        Some(deleted_user) => HttpResponse::Ok().json(deleted_user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}
pub async fn fibonacci(number: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().body(fib_calc(number.clone()).to_string())
}

fn fib_calc(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_calc(n - 1) + fib_calc(n - 2)
    }
}
