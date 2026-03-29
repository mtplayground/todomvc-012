use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

// ── DB helper functions (SSR only) ──────────────────────────────────────────
// These contain the actual business logic so they can be tested independently
// of the Leptos/Axum request context.

#[cfg(feature = "ssr")]
pub async fn db_add_todo(pool: &sqlx::SqlitePool, title: &str) -> Result<(), String> {
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    sqlx::query("INSERT INTO todos (title, completed) VALUES (?, 0)")
        .bind(&title)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn db_get_todos(pool: &sqlx::SqlitePool) -> Result<Vec<Todo>, String> {
    sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY id ASC")
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(feature = "ssr")]
pub async fn db_delete_todo(pool: &sqlx::SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn db_update_todo_title(
    pool: &sqlx::SqlitePool,
    id: i64,
    title: &str,
) -> Result<(), String> {
    let title = title.trim().to_string();
    if title.is_empty() {
        // Blank title deletes the todo
        sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
            .bind(&title)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn db_toggle_todo(pool: &sqlx::SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE todos SET completed = NOT completed WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn db_toggle_all(pool: &sqlx::SqlitePool, completed: bool) -> Result<(), String> {
    sqlx::query("UPDATE todos SET completed = ?")
        .bind(completed)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn db_clear_completed(pool: &sqlx::SqlitePool) -> Result<(), String> {
    sqlx::query("DELETE FROM todos WHERE completed = 1")
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ── Server functions ─────────────────────────────────────────────────────────

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;
    db_add_todo(&pool, &title)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e))
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;
    db_get_todos(&pool)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e))
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;
    db_delete_todo(&pool, id)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e))
}

#[server(UpdateTodoTitle, "/api")]
pub async fn update_todo_title(id: i64, title: String) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;
    db_update_todo_title(&pool, id, &title)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e))
}

#[server(ToggleTodo, "/api")]
pub async fn toggle_todo(id: i64) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;
    db_toggle_todo(&pool, id)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e))
}

#[server(ToggleAll, "/api")]
pub async fn toggle_all(completed: bool) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;
    db_toggle_all(&pool, completed)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e))
}

#[server(ClearCompleted, "/api")]
pub async fn clear_completed() -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;
    use sqlx::SqlitePool;

    let Extension(pool): Extension<SqlitePool> = extract().await?;
    db_clear_completed(&pool)
        .await
        .map_err(|e| ServerFnError::<server_fn::error::NoCustomError>::ServerError(e))
}
