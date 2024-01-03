use std::io::Write;
use llm::Model;


fn main() {
    println!("load a GGML model from disk...");
    // load a GGML model from disk
    let llama = llm::load::<llm::models::Llama>(
        // path to GGML file
        std::path::Path::new("ML/models/open_llama_3b-f16.bin"),
        // llm::ModelParameters
        Default::default(),
        // load progress callback
        llm::load_progress_callback_stdout).unwrap_or_else(|err| panic!("Failed to load model: {err}"));

    // use the model to generate text from a prompt
    let mut session = llama.start_session(Default::default());
    let res = session.infer::<std::convert::Infallible>(
        // model to use for text generation
        &llama,
        // randomness provider
        &mut rand::thread_rng(),
        // the prompt to use for text generation, as well as other
        // inference parameters
        &llm::InferenceRequest {
            prompt: "Number of English alphabets",
            ..Default::default()
        },
        // llm::OutputRequest
        &mut Default::default(),
        // output callback
        |t| {
            print!("{t}");
            std::io::stdout().flush().unwrap();
            Ok(())
        },
    );

    match res {
        Ok(result) => println!("\n\nInference stats:\n{result}"),
        Err(err) => println!("\n{err}"),
    }
}
