use crate::api::{get_all_recipe, AddRecipe};
use crate::constants::*;
use crate::types::Recipe;
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // use gloo_storage::Storage;
    
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (theme, set_theme) = create_signal(String::new());

    // create_effect(move |_| {
    //     let storage = gloo_storage::LocalStorage::raw();
    //     let _ = storage.set("theme", "dark"); 
    //     let light_or_dark = match  storage.get("theme") {
    //         Ok(c) if c.is_some() => c,
    //         Ok(_) => Some("light".to_string()),
    //         Err(_) => Some("light".to_string())
    //     };
    //     set_theme.set(light_or_dark.unwrap());
    // });

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/we-hungry.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <body class=theme>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
        <div class="grid min-h-screen w-full md:grid-cols-[220px_1fr] lg:grid-cols-[280px_1fr]">
        <NavBar set_theme/>
            <main class="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
            <div class="flex items-center">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/create" view=RecipeCreatePage/>
                </Routes>
            </div>
            </main>
        </div>
        </Router>
        </body>
    }
}


#[component]
fn NavBar(set_theme: WriteSignal<String>) -> impl IntoView {

    use gloo_storage::Storage;
    use leptos_icons::*;
    use icondata::{BsMoonStars, BsSun};

    let (light_or_dark, set_light_or_dark) = create_signal(false);

    create_effect(move |x| {
        let l_or_d = light_or_dark();
        
        if x.is_none() {
          return gloo_storage::LocalStorage::set("theme", "light").unwrap();
        }
        match l_or_d {
            true =>  {
                set_theme.set("dark".to_string());
                gloo_storage::LocalStorage::set("theme", "dark").unwrap()
            },
            false =>  {
                set_theme.set("light".to_string());
                gloo_storage::LocalStorage::set("theme", "light").unwrap()
            } 
        } 
    });

    let icon = Signal::derive(move || {
        if light_or_dark.get() {
            icondata::BsSun
        } else {
            icondata::BsMoonStars
        }
    });

    view! {
            <div class="relative inset-y-0 left-0 h-full w-3/4 border-r data-[state=closed]:slide-out-to-left data-[state=open]:slide-in-from-left sm:max-w-sm">
                <div class="fixed flex flex-col">
                    <nav class="mt-10 grid items-start px-2 text-sm font-medium lg:px-4">
                        <a
                            href="/"
                            class="flex items-center gap-2 text-lg font-semibold"
                        >
                            "We Hungry"
                        </a>
                        <a
                            href="/create"
                            class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary"
                        >
                        "Create"
                        </a>
                        <button
                        class=BUTTON_CLASS
                        on:click=move |_| {
                            set_light_or_dark(!light_or_dark());
                        }
                        >
                            <Icon icon />
                        </button>
                    </nav>
                </div>
            </div>
    }
}



#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="m-auto mt-4 flex items-center justify-center">
        <RecipeList />
        </div>
    }
}


#[component]
fn RecipeCreatePage() -> impl IntoView {
    view! {
        <div class="m-auto mt-4 flex items-center justify-center">
        <RecipeForm />
        </div>
    }
}

#[component]
fn RecipeList() -> impl IntoView {
    let get_recipe_info = create_resource(|| (), |_| async move { get_all_recipe().await });
    view! {
        <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center">
            <Suspense fallback=move || {
                view! { <p>"Loading data..."</p> }
            }>
                {
                    move || {
                        get_recipe_info.get().map(|data| {
                            match data {
                                Ok(recipes_data) => {
                                    // leptos::logging::log!("Re data: {:?}", recipes_data);
                                    view! {
                                        <RecipeCard recipes_data />
                                    }.into_view()
                                },
                                Err(e) => view! {
                                    <div>{e.to_string()}</div>
                                }.into_view()
                            }
                        })
                    }
                }
            </Suspense>
        </div>
    }
}

#[component]
fn RecipeCard(recipes_data: Vec<Recipe>) -> impl IntoView {

    view! {
        <div class="grid grid-cols-3 gap-4">
            <For
            each=move || recipes_data.clone()
            key=|recipe| recipe.id
            let:recipe
            >
                <div class=CARD_GET_ALL_CLASS>
                    <div class=CARD_HEADER_CLASS>
                        <h1 class=CARD_TITLE_CLASS>{recipe.title}</h1>
                        <p class=CARD_DESCRIPTION_CLASS>{recipe.description}</p>
                    </div>
                    <div class=CARD_CONTENT_CLASS>
                        <div class="overflow-y-auto max-h-[175px]">
                            <h2>"Instructions:"</h2>
                            <p>{recipe.instruction}</p>
                            <br/>
                            <h2>"Ingredients:"</h2>
                            <p>{recipe.ingredient}</p>
                            <br/>
                        </div>
                        <div class="my-2">
                            <p>"Prep Time: " {recipe.prep_time} " min"</p>
                            <p>"Cook Time: " {recipe.cook_time} " min"</p>
                            <p>"Servings: " {recipe.servings}</p>
                        </div>
                    </div>
                </div>
            </For>
        </div>
    }
}

#[component]
fn RecipeForm() -> impl IntoView {
    let add_recipe = create_server_action::<AddRecipe>();

    let response = add_recipe.value();
    
    let error = move || match response.get() {
        Some(Err(e)) => Some(e.to_string()),
        _ => None,
    };

    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (ingredient, set_ingredient) = create_signal(String::new());
    let (instruction, set_instruction) = create_signal(String::new());
    let (prep_time, set_prep_time) = create_signal(String::new());
    let (cook_time, set_cook_time) = create_signal(String::new());
    let (servings, set_servings) = create_signal(String::new());

    create_effect(move |_| {
        if add_recipe.value().get().is_some_and(|val| val.is_ok()) {
            set_title.set(String::new());
            set_description.set(String::new());
            set_ingredient.set(String::new());
            set_instruction.set(String::new());
            set_prep_time.set(String::new());
            set_cook_time.set(String::new());
            set_servings.set(String::new());
        }
    });

    let title_element = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_title.set(value);
    };
    let description_element = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_description.set(value);
    };
    let ingredient_element = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_ingredient.set(value);
    };
    let instruction_element = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_instruction.set(value);
    };
    let prep_time_element = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_prep_time.set(value);
    };
    let cook_time_element = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_cook_time.set(value);
    };
    let servings_element = move |evt: ev::Event| {
        let value = event_target_value(&evt);
        set_servings.set(value);
    };

    view! {
        <div class=CARD_CLASS>
            <div class=CARD_HEADER_CLASS>
                    <h1 class=CARD_TITLE_CLASS>"Let's add a recipe."</h1>
                    <p class=CARD_DESCRIPTION_CLASS>"Add something special to the cookbook of the internet!"</p>
            </div>
            <div class=CARD_CONTENT_CLASS>
                <ActionForm action=add_recipe>
                    <div class="grid w-full items-center gap-4">

                        <div class=LABEL_INPUT_DIV_CLASS>
                        <label
                        htmlFor="title"
                        className=LABEL_CLASS
                        >
                        "Title"
                        </label>

                        <input type="text"
                        class=INPUT_CLASS
                        name="title"
                        prop:value=title
                        on:input=title_element
                        required
                        placeholder="What do you call this meal?"
                        />
                        </div>

                        <div class=LABEL_INPUT_DIV_CLASS>
                        <label
                        htmlFor="description"
                        className=LABEL_CLASS
                        >
                        "Description"
                        </label>
                        <textarea
                        class=TEXTAREA_CLASS
                        name="description"
                        prop:value=move || description.get()
                        on:input=description_element
                        required
                        placeholder="Tell us a little about what we are making today?"
                        />
                        </div>

                        <div class=LABEL_INPUT_DIV_CLASS>
                        <label
                        htmlFor="ingredient"
                        className=LABEL_CLASS
                        >
                        "Ingredient"
                        </label>
                        <textarea
                        class=TEXTAREA_CLASS
                        name="ingredient"
                        prop:value=move || ingredient.get()
                        on:input=ingredient_element
                        required
                        placeholder="Can you tell use your ingredient?"
                        />
                        </div>

                        <div class=LABEL_INPUT_DIV_CLASS>
                        <label
                        htmlFor="instruction"
                        className=LABEL_CLASS
                        >
                        "Instruction"
                        </label>
                        <textarea
                        class=TEXTAREA_CLASS
                        name="instruction"
                        prop:value=move || instruction.get()
                        on:input=instruction_element
                        required
                        placeholder="What are the instruction to make this?"
                        />
                        </div>
                        <div class="flex gap-4">
                            <div class=LABEL_INPUT_DIV_CLASS>
                            <label
                            htmlFor="prep_time"
                            className=LABEL_CLASS
                            >
                            "Prep Time"
                            </label>
                            <input type="number"
                            class=INPUT_CLASS
                            name="prep_time"
                            prop:value=prep_time
                            on:input=prep_time_element
                            min="0"
                            placeholder="What's the prep time?"
                            />
                            </div>

                            <div class=LABEL_INPUT_DIV_CLASS>
                            <label
                            htmlFor="cook_time"
                            className=LABEL_CLASS
                            >
                            "Cook Time"
                            </label>
                            <input type="number"
                            class=INPUT_CLASS
                            name="cook_time"
                            prop:value=cook_time
                            on:input=cook_time_element
                            min="0"
                            placeholder="How long to cook?"
                            />
                            </div>

                            <div class=LABEL_INPUT_DIV_CLASS>
                            <label
                            htmlFor="servings"
                            className=LABEL_CLASS
                            >
                            "Servings"
                            </label>
                            <input type="number"
                            class=INPUT_CLASS
                            name="servings"
                            prop:value=servings
                            on:input=servings_element
                            min="0"
                            placeholder="How many can we feed?"
                            />
                            </div>
                        </div>
                    </div>

                    <ErrorBoundary fallback=move |error| {
                        move || format!("{:#?}", error.get())
                    }>
                        <pre class=ERROR_CLASS>{error}</pre>
                    </ErrorBoundary>

                    <div class=CARD_FOOTER_CLASS>
                        <button class=BUTTON_CLASS type="submit" value="Add" >
                        "Add Your Recipe"
                        </button>
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}




