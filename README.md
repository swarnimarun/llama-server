# LLama.cpp Server with HF

This project provides a Rust binary for docker images to handle things like automatic model download from HF, using HF cache and saving the model to disk.

The goal of the project is to be as minimal as possible, this project adds a 6 MB uncompressed binary to the image, with no additional dependencies to the image.

This leads to extremely small docker images for testing and production use.

The CPU only image is: ~227MB uncompressed.

The CUDA image is: ~3.4GB uncompressed.

The Intel image is: ~5.2GB uncompressed. (untested)

The Rocm image is: ~10GB uncompressed. (untested)

## Build the image using the repo

CPU Only Image

```bash
docker build -t swarnimarun/llama-server:latest . --no-cache
```

Other Images,

```bash
docker build -t swarnimarun/llama-server:cuda . --build-args BASE_IMAGE="ghcr.io/ggerganov/llama.cpp:server-cuda"
docker build -t swarnimarun/llama-server:intel . --build-args BASE_IMAGE="ghcr.io/ggerganov/llama.cpp:server-intel"
docker build -t swarnimarun/llama-server:rocm . --build-args BASE_IMAGE="ghcr.io/ggerganov/llama.cpp:server-rocm"
```

## Roadmap

None. For me it's complete but the CLI should expose all/most of the features from llama-server if someone wants to help port that, you are welcome.

## License => MIT

Same as `llama.cpp`, we use MIT for the Rust source code provided in the repository.
