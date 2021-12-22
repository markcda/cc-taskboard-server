use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use hyper::Body;
use hyper::http::Request;
use hyper::body::to_bytes;

use crate::sec::auth::UserCredentials;
use crate::setup::AppConfig;

type PgClient = Arc<Mutex<tokio_postgres::Client>>;

/// Объединяет окружение в одну структуру данных.
pub struct Workspace {
  pub req: Request<Body>,
  pub cli: PgClient,
  pub cnf: AppConfig,
}

/// Набор цветов для раскраски компонента.
#[derive(Deserialize, Serialize)]
pub struct ColorSet {
  pub text_color: String,
  pub background_color: String,
}

/// Временные рамки для задач и подзадач.
#[derive(Deserialize, Serialize)]
pub struct Timelines {
  /// Предпочтительно закончить до X (даты и времени).
  pub preferred_time: String,
  /// Обязательно закончить до Y (даты и времени).
  pub max_time: String,
  /// Ожидаемое время выполнения задачи Z в минутах.
  pub expected_time: u32,
}

/// Тег.
#[derive(Deserialize, Serialize)]
pub struct Tag {
  pub title: String,
  pub color_set: ColorSet,
}

/// Подзадача.
#[derive(Deserialize, Serialize)]
pub struct Subtask {
  pub id: i64,
  pub author: i64,
  pub title: String,
  pub executors: Vec<i64>,
  pub exec: bool,
  pub tags: Vec<Tag>,
  pub timelines: Timelines,
  pub color_set: ColorSet,
}

/// Задача.
#[derive(Deserialize, Serialize)]
pub struct Task {
  pub id: i64,
  pub author: i64,
  pub title: String,
  pub executors: Vec<i64>,
  pub exec: bool,
  pub subtasks: Vec<Subtask>,
  pub notes: String,
  pub tags: Vec<Tag>,
  pub timelines: Timelines,
  pub color_set: ColorSet,
}

/// Карточка.
#[derive(Deserialize, Serialize)]
pub struct Card {
  pub id: i64,
  pub author: i64,
  pub title: String,
  pub tasks: Vec<Task>,
  pub color_set: ColorSet,
}

/// Доска.
#[derive(Deserialize, Serialize)]
pub struct Board {
  pub id: i64,
  pub author: i64,
  pub shared_with: Vec<i64>,
  pub title: String,
  pub cards: Vec<Card>,
  pub background_color: String,
}

/// Пользователь.
#[derive(Deserialize, Serialize)]
pub struct User {
  pub id: i64,
  pub shared_boards: Vec<i64>,
  pub user_creds: UserCredentials,
}

/// Извлекает данные из тела HTTP-запроса.
pub async fn extract<T>(req: Request<Body>) -> Result<T, ()>
  where
    T: DeserializeOwned,
{
  let body = req.into_body();
  let body = match to_bytes(body).await {
    Err(_) => return Err(()),
    Ok(v) => v,
  };
  let body = match String::from_utf8(body.to_vec()) {
    Err(_) => return Err(()),
    Ok(v) => v.clone(),
  };
  let obj = serde_json::from_str::<T>(&body);
  match obj {
    Err(_) => return Err(()),
    Ok(v) => Ok(v),
  }
}
