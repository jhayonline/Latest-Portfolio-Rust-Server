use salvo::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, QueryOrder};
use serde::Deserialize;
use tracing::error;

use crate::entity::wall_message::{ActiveModel, Column, Entity as WallMessage, Model};

fn database_from_depot(depot: &mut Depot) -> &DatabaseConnection {
    depot
        .obtain::<DatabaseConnection>()
        .expect("DatabaseConnection not found in Depot, maybe in main.rs? loll")
}

fn send_json(e: &str) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "error": e
    }))
}

#[handler]
pub async fn get_messages(depot: &mut Depot, response: &mut Response) {
    let database = database_from_depot(depot);

    match WallMessage::find()
        .order_by_desc(Column::CreatedAt)
        .all(database)
        .await
    {
        Ok(messages) => {
            response.render(Json(messages));
        }
        Err(e) => {
            error!("Failed to fetch messages: {e}");

            response.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            response.render(send_json("Failed to fetch messages"));
        }
    }
}

#[derive(Deserialize)]
struct AddMessageBody {
    name: String,
    message: String,
}

#[handler]
pub async fn add_message(request: &mut Request, depot: &mut Depot, response: &mut Response) {
    let body = match request.parse_json::<AddMessageBody>().await {
        Ok(b) => b,
        Err(_) => {
            response.status_code(StatusCode::BAD_REQUEST);
            response.render(send_json("Invalid request body"));

            return;
        }
    };

    if body.name.trim().is_empty() || body.message.trim().is_empty() {
        response.status_code(StatusCode::BAD_REQUEST);
        response.render(send_json("Name and message are required"));

        return;
    }

    let database = database_from_depot(depot);

    let new_message = ActiveModel {
        name: Set(body.name.trim().to_owned()),
        message: Set(body.message.trim().to_owned()),
        // likes default to 0
        ..Default::default()
    };

    match new_message.insert(database).await {
        Ok(saved) => {
            response.status_code(StatusCode::CREATED);
            response.render(Json(saved));
        }
        Err(e) => {
            error!("Failed to insert message: {e}");

            response.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            response.render(send_json("Failed to create message"));
        }
    }
}

#[handler]
pub async fn add_like(request: &mut Request, depot: &mut Depot, response: &mut Response) {
    let id: i32 = match request.param::<i32>("id") {
        Some(id) => id,
        None => {
            response.status_code(StatusCode::BAD_REQUEST);
            response.render(send_json("Invalid id"));

            return;
        }
    };

    let database = database_from_depot(depot);

    let message: Model = match WallMessage::find_by_id(id).one(database).await {
        Ok(Some(m)) => m,
        Ok(None) => {
            response.status_code(StatusCode::NOT_FOUND);
            response.render(send_json("Message not found"));

            return;
        }
        Err(e) => {
            error!("Database error finding message {id}: {e}");

            response.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            response.render(send_json("Database error"));

            return;
        }
    };

    let mut active: ActiveModel = message.into();

    let current_likes = active.likes.as_ref().to_owned();

    active.likes = Set(current_likes + 1);

    match active.update(database).await {
        Ok(updated) => {
            response.render(Json(updated));
        }
        Err(e) => {
            error!("Failed to update likes for message {id}: {e}");

            response.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            response.render(send_json("Failed to update likes"));
        }
    }
}
