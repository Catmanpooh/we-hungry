cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {
        use dotenv::dotenv;
        use leptos::server_fn::ServerFnError;
        use sqlx::PgPool;
        use std::env;
        use crate::types::{AddRecipeRequest, Recipe};

        pub async fn db() -> Result<PgPool, ServerFnError> {
            dotenv().ok();

            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

            Ok(PgPool::connect(&database_url).await?)
        }

        pub async fn add_new_recipe(recipe: AddRecipeRequest) -> Result<(), ServerFnError> {

            let pool = db()
            .await?;
        
            let _ = sqlx::query(
                "INSERT INTO recipe (id, title, description, ingredient, instruction, 
                prep_time, cook_time, servings)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
            )
            .bind(uuid::Uuid::new_v4())
            .bind(recipe.title)
            .bind(recipe.description)
            .bind(recipe.ingredient)
            .bind(recipe.instruction)
            .bind(recipe.prep_time)
            .bind(recipe.cook_time)
            .bind(recipe.servings)
            .execute(&pool)
            .await?;
            
            Ok(())
        }
        
        
        pub async fn all_recipes() -> Result<Vec<Recipe>, ServerFnError> {
        
            let pool = db()
                .await?;
        
            let results = sqlx::query_as::<_, Recipe>(
                "SELECT * FROM recipe;"
            )
            .fetch_all(&pool)
            .await;
            
            match results {
                Ok(recipes) => Ok(recipes),
                Err(e) => {
                    leptos::logging::log!("Error getting recipe: {:?}", e);
                    Err(ServerFnError::Args(String::from("Database error")))
                }
            }
        }
    }
}



