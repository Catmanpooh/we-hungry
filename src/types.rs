use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Recipe {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub ingredient: String,
    pub instruction: String,
    pub prep_time: Option<i16>,
    pub cook_time: Option<i16>,
    pub servings: Option<i16>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Recipe {
    pub fn new_default() -> Recipe {
        Default::default()
    }

    pub fn new(
        id: uuid::Uuid,
        title: String,
        description: String,
        ingredient: String,
        instruction: String,
        prep_time: Option<i16>,
        cook_time: Option<i16>,
        servings: Option<i16>,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Recipe {
        Recipe {
            id,
            title,
            description,
            ingredient,
            instruction,
            prep_time,
            cook_time,
            servings,
            created_at,
            updated_at,
        }
    }

    
pub fn create_dummy_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            id: Uuid::new_v4(),
            title: String::from("Spaghetti Bolognese"),
            description: String::from("A classic Italian pasta dish with rich meat sauce."),
            ingredient: String::from("Spaghetti, Ground Beef, Tomato Sauce, Onions, Garlic"),
            instruction: String::from("1. Boil the spaghetti. 2. Cook the beef with onions and garlic. 3. Add tomato sauce and simmer."),
            prep_time: Some(15),
            cook_time: Some(45),
            servings: Some(4),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Recipe {
            id: Uuid::new_v4(),
            title: String::from("Chicken Curry"),
            description: String::from("A flavorful chicken curry with a blend of spices."),
            ingredient: String::from("Chicken, Curry Powder, Coconut Milk, Onions, Garlic, Ginger"),
            instruction: String::from("1. Cook onions, garlic, and ginger. 2. Add chicken and spices. 3. Stir in coconut milk and simmer."),
            prep_time: Some(20),
            cook_time: Some(30),
            servings: Some(4),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Recipe {
            id: Uuid::new_v4(),
            title: String::from("Beef Tacos"),
            description: String::from("Delicious beef tacos with fresh toppings."),
            ingredient: String::from("Ground Beef, Taco Seasoning, Tortillas, Lettuce, Cheese, Salsa"),
            instruction: String::from("1. Cook beef with seasoning. 2. Assemble tacos with toppings."),
            prep_time: Some(10),
            cook_time: Some(20),
            servings: Some(6),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Recipe {
            id: Uuid::new_v4(),
            title: String::from("Pancakes"),
            description: String::from("Fluffy pancakes perfect for breakfast."),
            ingredient: String::from("Flour, Eggs, Milk, Sugar, Baking Powder"),
            instruction: String::from("1. Mix dry and wet ingredients. 2. Cook on a hot griddle until golden brown."),
            prep_time: Some(10),
            cook_time: Some(15),
            servings: Some(4),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Recipe {
            id: Uuid::new_v4(),
            title: String::from("Grilled Salmon"),
            description: String::from("Simple and healthy grilled salmon."),
            ingredient: String::from("Salmon Fillets, Lemon, Olive Oil, Salt, Pepper"),
            instruction: String::from("1. Season salmon with olive oil, lemon, salt, and pepper. 2. Grill until cooked through."),
            prep_time: Some(5),
            cook_time: Some(15),
            servings: Some(2),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Recipe {
            id: Uuid::new_v4(),
            title: String::from("Chocolate Cake"),
            description: String::from("Rich and moist chocolate cake."),
            ingredient: String::from("Flour, Cocoa Powder, Sugar, Eggs, Butter, Baking Soda, Milk"),
            instruction: String::from("1. Mix dry and wet ingredients. 2. Bake at 350°F for 30 minutes."),
            prep_time: Some(15),
            cook_time: Some(30),
            servings: Some(8),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    ]
}
}

#[derive(Debug, Default, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct AddRecipeRequest {
    #[validate(length(min = 5, message = "title is required"))]
    pub title: String,
    #[validate(length(min = 20, message = "description is required"))]
    pub description: String,
    #[validate(length(min = 20, message = "ingredient is required"))]
    pub ingredient: String,
    #[validate(length(min = 20, message = "instruction is required"))]
    pub instruction: String,
    #[validate(range(min = 1, max = 255))]
    pub prep_time: Option<i16>,
    #[validate(range(min = 1, max = 255))]
    pub cook_time: Option<i16>,
    #[validate(range(min = 1, max = 255))]
    pub servings: Option<i16>,
}

impl AddRecipeRequest {
    pub fn new(
        title: String,
        description: String,
        ingredient: String,
        instruction: String,
        prep_time: Option<i16>,
        cook_time: Option<i16>,
        servings: Option<i16>,
    ) -> AddRecipeRequest {
        AddRecipeRequest {
            title,
            description,
            ingredient,
            instruction,
            prep_time,
            cook_time,
            servings,
        }
    }
}