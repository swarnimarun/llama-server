use std::process::Command;

use anyhow::{Context, Ok};
use clap::Parser;

#[derive(clap::Parser)]
struct BetterLlamaServer {
    #[arg(long, short, default_value_t = 8000)]
    /// port to be bound
    port: u16,
    #[arg(long, short = 'o', default_value_t = String::from("0.0.0.0"))]
    /// host to be bound
    host: String,
    #[arg(long, short, default_value_t = String::from("lmstudio-community/Meta-Llama-3-8B-Instruct-GGUF"))]
    /// Model path within the image, likely volume mounted.
    /// Or, model repo ID from Hugging Face
    model: String,

    #[arg(short, default_value_t = String::from("q8"))]
    /// you can add the key for your quantization eg. q4_0, q4_1, q5_0, q5_1, q5_k_s, q5_k_m or just use q2, q6, q8.
    quantization: String,

    #[arg(short, long, default_value_t = 0)]
    /// number of gpu layers to be used, zero means none
    gpu_layers: u16,
}

fn main() -> anyhow::Result<()> {
    ctrlc::set_handler(move || {
        panic!("SIGTERM requested, exiting without cleanup.");
    })?;

    let BetterLlamaServer {
        port,
        host,
        model,
        quantization,
        gpu_layers,
    } = BetterLlamaServer::parse();

    let quantization = quantization.to_lowercase();
    // download model if not found
    let model = if model.ends_with(".gguf") {
        model
    } else {
        // download from Hugging Face Hub
        let api = hf_hub::api::sync::Api::new()?.model(model);
        let gitattributes = api.get(".gitattributes")?;
        let files = std::fs::read_to_string(gitattributes)?;
        let model_files = files.lines().filter_map(|s| {
            // split the file names for all the large gguf files from gitattrs
            if s.contains(".gguf filter") {
                s.split(' ').next()
            } else {
                None
            }
        });
        let mut model = String::new();
        for model_filename in model_files {
            // download from Hugging Face Hub
            if model_filename.to_lowercase().contains(&quantization) {
                let model_path = api.get(model_filename)?;
                if model_path.is_file()
                    && model_path
                        .extension()
                        .map(|f| f.eq("gguf"))
                        .unwrap_or_default()
                {
                    model = model_path
                        .to_str()
                        .context("Failed to convert path os_str to str")?
                        .to_string();
                    // early exit if correct model is found.
                    break;
                }
            }
        }
        model
    };

    let mut command = Command::new("/server");
    command
        .args(["--port", format!("{port}").as_str()])
        .args(["--n-gpu-layers", format!("{gpu_layers}").as_str()])
        .args(["--host", format!("{host}").as_str()])
        .args(["-m", format!("{model}").as_str()]);
    println!("{:#?}", command);

    _ = command.spawn()?.wait()?;
    Ok(())
}
