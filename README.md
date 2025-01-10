# Watch-n-Paint

[![GitHub](https://img.shields.io/github/license/elamribadrayour/watch-n-paint)](https://github.com/elamribadrayour/watch-n-paint/blob/main/LICENSE)

## Overview

**Watch-n-Paint** is a project aimed at creating a Generative Adversarial Network (GAN) that can reproduce the universe of a movie or a TV show. The first use case for this project is the TV show "The Wire".

## Author

- **Badr Ayour El Amri**  
  [GitHub Profile](https://github.com/elamribadrayour)

## Project Structure

- **src/main.rs**: The main entry point of the application, which initializes the configuration and processes the dataset.
- **src/config**: Contains modules related to configuration management.
  - **config_impl.rs**: Handles loading and parsing of configuration files.
  - **dataset.rs**: Defines the structure of the dataset configuration.
- **src/dataset**: Contains modules related to dataset management.
  - **dataset_impl.rs**: Manages the dataset, including loading items from a directory.
  - **item.rs**: Defines the `Item` structure and its associated methods for processing images.

## Configuration

The project uses a JSON configuration file (`Config.json`) to specify dataset parameters such as width, height, and path. Here is an example configuration:

```json

{
    "dataset": {
        "width": 500,
        "height": 500,
        "path": "data/the-wire"
    }
}

```

## Dependencies

The project is built using Rust and relies on several dependencies, as specified in `Cargo.toml`:

- `anyhow`: For error handling.
- `image`: For image processing.
- `serde` and `serde_json`: For serialization and deserialization of configuration files.
- `uuid`: For generating unique identifiers for image files.

## Usage

1. Clone the repository:
```bash
git clone https://github.com/elamribadrayour/watch-n-paint.git
cd watch-n-paint
```

2. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).

3. Build the project:
```bash
cargo build --release
```

4. Run the project:
```bash
cargo run --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an Issue for any bugs or feature requests.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/elamribadrayour/watch-n-paint/blob/main/LICENSE) file for details.

## Contact

For any inquiries, please contact Badr Ayour El Amri via [GitHub](https://github.com/elamribadrayour).
