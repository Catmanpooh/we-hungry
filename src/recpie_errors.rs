use strum::{Display, EnumString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
pub enum AddRecipeError {
    #[strum(to_string = "Thats the best title you can come up with.")]
    InvalidTitle,
    #[strum(to_string = "Give use a good description.")]
    InvalidDescription,
    #[strum(to_string = "There has to be more ingredients.")]
    InvalidIngredient,
    #[strum(to_string = "Clear instructions I want to make this.")]
    InvalidInstruction,
    #[strum(to_string = "Prep time seems kind of low.")]
    InvalidPrepTime,
    #[strum(to_string = "Is that the real cook time.")]
    InvalidCookTime,
    #[strum(to_string = "With that amount of servings I will be hungry.")]
    InvalidServings,
    #[strum(to_string = "Please try adding the recipe again; we had an issue.")]
    CreatingRecipeError,
    #[strum(to_string = "Seems like somthing is wrong on my end; just try again.")]
    UnknownError,
    #[strum(to_string = "Yea I will need to get my boss for this.")]
    InternalServerError,
}


#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
pub enum GetOneRecipeError {
    #[strum(to_string = "Not sure what im searching for no id or title provided.")]
    TitleOrIdError,
    #[strum(to_string = "I searched hard but came up empty handed.")]
    FoundNothingError,
    #[strum(to_string = "Try again I think well get something next time.")]
    TryAgainError,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Eq, EnumString, Display)]
pub enum GetAllRecipeError {
    #[strum(to_string = "All that to return you nothing.")]
    FoundAllOrNothingError,
    #[strum(to_string = "We didn't get anything.")]
    TryAllAgainError,
    #[strum(to_string = "I think i'll be working late tonight.")]
    InternalServerError,
}

