
mod shared;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use shared::{*, consts::*};

#[tokio::main]
async fn main() -> Result<()> {
    let text_model = get_text_model();
    println!("Going to use text model: {}", text_model);

    // By default localhost:11434
    let ollama = Ollama::default();

    let prompt = "Give me a YouTube shorts script written as Italian brainrot".to_string();

    let gen_req = GenerationRequest::new(text_model, prompt)
                  .system(DEFAULT_SYSTEM_MOCK.to_string());

    let gen_res = ollama.generate(gen_req).await;

    let response = match gen_res {
        Ok(val) => val.response,
        Err(e) => format!("Failed to get a proper response, error:\n{}", e)
    };

    println!("{}", response);

    Ok(())
}
