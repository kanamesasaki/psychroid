# Psychroid

A Rust library for psychrometric calculations and moist air properties.

## Overview

Psychroid is a Rust implementation of psychrometric functions for HVAC calculations and moist air analysis.
The library supports both SI and IP (Imperial) unit systems.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have suggestions for improvements, bug fixes, or additional features.

## WebAssembly Support

Psychroid can be compiled to WebAssembly (WASM), allowing the library to run in web browsers. To build the WebAssembly module:

```bash
wasm-pack build --target web
```

This will generate WASM bindings in the pkg directory, which can be imported in JavaScript applications.
The generated WASM is used in the following web application:

Psychrometric Chart Calculator (Psychroid-Web)
- GitHub: https://github.com/kanamesasaki/psychroid-web
- Web-site: https://psychroid.thermocraft.space/

## License

This project is licensed under the MIT License.