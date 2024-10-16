use crate::recpie_errors::*;
use crate::types::*;
use crate::db::*;
use leptos::*;

// #[server(GetRecipe, "/api")]
// pub async fn get_recipe(recipe_id: Option<uuid::Uuid>, title: Option<String>) -> Result<Recipe, ServerFnError<GetOneRecipeError>> {
//     let id_or_title = if let Some(id) = recipe_id {
//         id.to_string()
//     } else if let Some(t) = title {
//         t
//     } else {
//         return Err(GetOneRecipeError::TitleOrIdError.into())
//     };

//     let response = recipe(id_or_title).await;

//     match response {
//         Ok(Some(recipe)) => Ok(recipe),
//         Ok(None) => Err(GetOneRecipeError::FoundNothingError.into()),
//         Err(_) => Err(GetOneRecipeError::TryAgainError.into()),
//     }
// }


#[server(GetAllRecipe, "/api")]
pub async fn get_all_recipe() -> Result<Vec<Recipe>, ServerFnError<GetAllRecipeError>> {
    let response = all_recipes().await;


    match response {
        Ok(recipe) => Ok(recipe),
        Err(_) => Err(GetAllRecipeError::TryAllAgainError.into()),
    }
}


#[server(AddRecipe, "/api")]
pub async fn add_recipe(
    title: String,
    description: String,
    ingredient: String,
    instruction: String,
    prep_time: Option<i16>,
    cook_time: Option<i16>,
    servings: Option<i16>,
) -> Result<(), ServerFnError<AddRecipeError>> {
    use validator::Validate;

    let new_recipe = AddRecipeRequest::new(
        title,
        description,
        ingredient,
        instruction,
        prep_time,
        cook_time,
        servings,
    );
    

    if let Err(e) = new_recipe.validate() {
        for (field, _err) in e.field_errors() {
            if field == "title" {
                return Err(AddRecipeError::InvalidTitle.into());
            }
            if field == "description" {
                return Err(AddRecipeError::InvalidDescription.into());
            }
            if field == "ingredient" {
                return Err(AddRecipeError::InvalidIngredient.into());
            }
            if field == "instruction" {
                return Err(AddRecipeError::InvalidInstruction.into());
            }
            if field == "prep_time" {
                return Err(AddRecipeError::InvalidPrepTime.into());
            }
            if field == "cook_time" {
                return Err(AddRecipeError::InvalidCookTime.into());
            }
            if field == "servings" {
                return Err(AddRecipeError::InvalidServings.into());
            }
        }
        return Err(AddRecipeError::UnknownError.into());
    }

    let response = add_new_recipe(new_recipe).await;
    leptos::logging::log!("REsponse Add recipe: {:?}", response);
    match response {
        Ok(_) => Ok(()),
        Err(_) => Err(AddRecipeError::UnknownError.into()),
    }
}
