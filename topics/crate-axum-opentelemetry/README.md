# Axum and OpenTelemetry

This is a simple example of how to use Axum and OpenTelemetry together. And visualize the traces in Jaeger.

## Ressources

This code is masively inspired by this video: [Rust Axum and OpenTelemetry](https://www.youtube.com/watch?v=aVFzMog9FOQ):

<p align="center">
  <a href="https://www.youtube.com/watch?v=aVFzMog9FOQ">
    <img src="https://img.youtube.com/vi/aVFzMog9FOQ/0.jpg" width="300"/>
  </a>
</p>

## How to run

### 1. Start Jaeger

```bash
dockercompose up
```

### 2. Start the server

```bash
cargo run
```

### 3. Make a request

```bash
curl -X GET http://localhost:3000/health 
```

### 4. Open Jaeger

Open your browser and go to [http://localhost:16686/](http://localhost:16686/)
