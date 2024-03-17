#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;

use core::{completion, env_var_exists, save_token};
use core::{SharedStream, Conversation};

use tauri::State;

/*---------MAIN FUNCTION------- */

fn main() {

    tauri::Builder::default()
        .manage(SharedStream::default())
        .invoke_handler(tauri::generate_handler![register_new_token, wrapper_answer, middleware_layer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*---------REGISTER TOKEN------- */

 #[tauri::command]
async fn register_new_token(token: String, state: State<'_, SharedStream>) -> Result<(), String> {
    
    println!("Registering a new API Key...");

    if let Err(e) = save_token(&token) {

        println!("⚠️ API Key");

        let error_news = format!("Couldn't register the token, please restart the application or open an issue @github/kinxyo/CooperAI\n{}", e);

        Err(error_news)

    } else {
        
        let mut config = state.0.lock().unwrap();
        config.header = format!("Bearer {}", token);
        
        println!("✅ API Key");
        Ok(())
    }
    
}

/*---------GENERATE RESPONSE------- */

#[tauri::command]
fn wrapper_answer(chats: Vec<Conversation>, state: State<'_, SharedStream>) -> String {
    
    /* Had to create a wrapper function to avoid this error:

        ```
        the trait bound `std::string::String: AsyncCommandMustReturnResult` 
        is not satisfied the trait `AsyncCommandMustReturnResult` 
        is implemented for `Result<A, B>` required for the 
        cast from `&std::string::String` to `&dyn AsyncCommandMustReturnResult`.
        ```

        will look into it later...
    */
    answer(chats, state)
}

#[tokio::main]
async fn answer(chats: Vec<Conversation>, state: State<'_, SharedStream>) -> String {
    
    /* fetching query */
    let mut query = state.0.lock().unwrap();

    /* generating response */
    let answer = completion(&mut query, chats).await;
    
    /* returning answer */
    match answer {
        Ok(v) => v,
        Err(e) => e.to_string(),
    }
}

/*---------MIDDLEWARE------- */

#[tauri::command]
fn middleware_layer() -> bool {
    env_var_exists()
}