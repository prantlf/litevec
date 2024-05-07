# Lightweight Vector Database

A tiny vector database for storing and querying embeddings in pure Rust. Read the explanation of [embeddings at OpenAI].

* Runs in memory for great speed.
* Serialises embeddings to binary files per collection for speed and simplicity.
* Little code for sustainable  maintenance.
* Versatile REST API for various scenarios.
* Easy to integrate to prototypes or small products.

This is a fork of [tinyvector] with the following goals:

* Allow filtering by metadata in addition to the vector similarity search.
* Offer other algorithms for computing the vector similarity in addition to `cosine`, `dot` and `euclidean`.
* Focus only on embeddings. Do not integrate LLMs to split and vectorise text.
* Support CORS and other network features.

There's a [demo example] included. This database is integrated in the lightweight AI RAG solution [literag].

## Getting Started

Using Docker is easier than running the services built from the scratch. But building is easy too. You need just the [Rust] compiler.

### Using Docker

For example, run a container for testing purposes exposing the port 8000 which will be deleted on exit:

    docker run -p 8000:8000 --rm -it ghcr.io/prantlf/litevec

For example, run a container named `litevec` in the background, persisting the data in `./litevec-storage` via the volume `/storage`:

    docker run -p 8000:8000 -v $PWD/litevec-storage:/storage \
      -dt --name litevec ghcr.io/prantlf/litevec

And the same task as above, only using Docker Compose (place [docker-compose.yml] to the current directory) to make it easier:

    docker-compose up -d

### Building from Scratch

Make sure that you have [Rust] installed before you continue. Clone this repository, build the binary executable and run it:

    git clone https://github.com/prantlf/litevec.git
    cd litevec
    cargo build --release
    target/release/litevec

The `storage` directory will be created in the current directory as needed.

### Configuration

Runtime parameters of the service can be customised using the process environment variables below:

| Name                      | Default    | Description                                 |
|:--------------------------|:-----------|:--------------------------------------------|
| LITEVEC_AUTOSAVE_INTERVAL | 10         | how often to ensure the database saved [s]  |
| LITEVEC_COMPRESSION_LIMIT | 1024       | minimum response size to get compressed [b] |
| LITEVEC_CORS_MAXAGE       | 86400      | how long stays CORS preflighting valid [s]  |
| LITEVEC_HOST              | 0.0.0.0    | IP address to bind the server to            |
| LITEVEC_PORT              | 8000       | port number to bind the server to           |
| LITEVEC_PAYLOAD_LIMIT     | 1073741824 | maximum size of request payload [b]         |
| LITEVEC_STORAGE           | ./storage  | directory to store the collections to       |
| LITEVEC_TIMEOUT           | 30         | maximum duration of a request [s]           |
| RUST_LOG                  | info       | log level (`info`, `debug`, `trace`)        |

A `.env` file with environment variables will be loaded and processed automatically.

## API

See the summary of the endpoints below, [API details] on a separate page. Run `litevec` and open http://localhost:8000/docs to inspect and try the available REST API endpoints live.

System endpoints:

| Method | Path      | Description                                                 |
|:-------|:----------|:------------------------------------------------------------|
| GET    | /         | obtain API metadata                                         |
| GET    | /ping     | checks that the server is running                           |
| POST   | /shutdown | shut the service down (sending SIGTERM or SIGINT works too) |

Documentation endpoints:

| Method | Path          | Description                                     |
|:-------|:--------------|:------------------------------------------------|
| GET    | /docs         | web page with the API documentation             |
| GET    | /openapi.json | API description according to the OpenAPI schema |

Endpoints for embedding collections and similarity search:

| Method | Path                          | Description                                                                 |
|:-------|:------------------------------|:----------------------------------------------------------------------------|
| GET    | /collections                  | list collection names                                                       |
| POST   | /collections/:collection_name | search the collection for similar vectors while filtering with metadata too |
| PUT    | /collections/:collection_name | create a collection                                                         |
| PATCH  | /collections/:collection_name | rename a collection                                                         |
| GET    | /collections/:collection_name | get information about a collection                                          |
| DELETE | /collections/:collection_name | delete a collection                                                         |

Endpoints for embeddings:

| Method | Path                                                   | Description                        |
|:-------|:-------------------------------------------------------|:-----------------------------------|
| GET    | /collections/:collection_name/embeddings               | list embedding identifiers         |
| POST   | /collections/:collection_name/embeddings               | filter embeddings with metadata    |
| DELETE | /collections/:collection_name/embeddings               | delete embeddings by metadata      |
| PUT    | /collections/:collection_name/embeddings/:embedding_id | create an embedding                |
| PATCH  | /collections/:collection_name/embeddings/:embedding_id | update an embedding                |
| GET    | /collections/:collection_name/embeddings/:embedding_id | get information about an embedding |
| DELETE | /collections/:collection_name/embeddings/:embedding_id | delete an embedding                |

## License

Copyright (c) 2023 Miguel Piedrafita<br>
Copyright (c) 2023-2024 Ferdinand Prantl

Licensed under the MIT license.

[embeddings at OpenAI]: https://platform.openai.com/docs/guides/embeddings/what-are-embeddings
[tinyvector]: https://github.com/m1guelpf/tinyvector
[literag]: https://github.com/prantlf/literag
[demo example]: ./docs/DEMO.md
[Rust]: https://rustup.rs
[docker-compose.yml]: ./docker-compose.yml
[API details]: ./docs/API.md
