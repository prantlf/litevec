# Lightweight Vector Database

A tiny vector database for storing and querying embeddings in pure Rust. Read the explanation of [embeddings at OpenAI].

* Runs in memory for great speed.
* Serialises embeddings to one binary file for simplicity.
* Little code for sustainable  maintenance.
* Versatile REST API for various scenarios.
* Easy to integrate to prototypes or small products.

This is a fork of [tinyvector] with the following goals:

* Allow filtering by metadata in addition to the vector similarity search.
* Offer other algorithms for computing the vector similarity in addition to `cosine`, `dot` and `euclidean`.
* Focus only on embeddings. Do not integrate LLMs to split and vectorise text.
* Support CORS and other network features.

## Getting Started

Using Docker is easier than running the services built from the scratch. But building is easy. You need just the [Rust] compiler.

### Using Docker

For example, run a container for testing purposes exposing the port 8000 which will be deleted on exit:

    docker run -p 8000:8000 --rm -it ghcr.io/prantlf/litevec

For example, run a container named `litevec` in the background, persisting the data in `./litevec-storage` via the volume `/litevec/storage`:

    docker run -p 8000:8000 -v $PWD/litevec-storage:/litevec/storage \
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

| Name                  | Default    | Description                               |
|:----------------------|:-----------|:------------------------------------------|
| LITEVEC_HOST          | 0.0.0.0    | IP address to bind the server to          |
| LITEVEC_CORS_MAXAGE   | 86400      | how lon stays CORS preflighting valid [s] |
| LITEVEC_PORT          | 8000       | port address to bind the server to        |
| LITEVEC_PAYLOAD_LIMIT | 1073741824 | maximum size of request payload [bytes]   |
| LITEVEC_TIMEOUT       | 30         | maximum duration of a request [s]         |
| RUST_LOG              | info       | log level (`info`, `debug`, `trace`)      |

## API

Run `litevec` and open http://localhost:8000/docs to inspect and try the available REST API endpoints.

System endpoints:

| Method | Path      | Description           |
|:-------|:----------|:----------------------|
| GET    | /         | obtain API metadata   |
| POST   | /shutdown | shut the service down |

Documentation endpoints:

| Method | Path          | Description                                     |
|:-------|:--------------|:------------------------------------------------|
| GET    | /docs         | web page with the API documentation             |
| GET    | /openapi.json | API description according to the OpenAPI schema |

Endpoints for embedding collections and similarity search:

| Method | Path                          | Description                                                                     |
|:-------|:------------------------------|:--------------------------------------------------------------------------------|
| GET    | /collections                  | list collection names                                                           |
| POST   | /collections/:collection_name | search the collection for similar vectors while filtering with metadata too (1) |
| PUT    | /collections/:collection_name | create a collection (2)                                                         |
| GET    | /collections/:collection_name | get information about a collection (3)                                          |
| DELETE | /collections/:collection_name | delete a collection                                                             |


Similarity search input and output (1):

```ts
interface SearchInput {
	/// Vector to query with
	query: float[]
	/// Metadata to filter with
	filter?: Record<String, String>[]
	/// Number of results to return
	k?: integer
}

interface SearchOutput {
	/// Similarity score
	score: float
	/// Matching embedding
	embedding: Embedding
}
```

Collection input (2):

```ts
interface Collection {
	/// Dimension of the vectors in the collection
	dimension: integer
	/// Distance metric used for querying
	distance: 'cosine' | 'dot' | 'euclidean'
	/// Embeddings in the collection
	embeddings: Embedding[]
}
```

Collection output (3):

```ts
interface Collection {
	/// Name of the collection
	name: string
	/// Dimension of the vectors in the collection
	dimension: integer
	/// Distance metric used for querying
	distance: 'cosine' | 'dot' | 'euclidean'
	/// Number of embeddings in the collection
	embedding_count: integer
}
```

Endpoints for embeddings:

| Method | Path                                                   | Description                            |
|:-------|:-------------------------------------------------------|:---------------------------------------|
| GET    | /collections/:collection_name/embeddings               | list embedding identifiers             |
| POST   | /collections/:collection_name/embeddings               | filter embeddings with metadata (4)    |
| DELETE | /collections/:collection_name/embeddings               | delete embeddings by metadata          |
| PUT    | /collections/:collection_name/embeddings/:embedding_id | create an embedding (5)                |
| GET    | /collections/:collection_name/embeddings/:embedding_id | get information about an embedding (4) |
| DELETE | /collections/:collection_name/embeddings/:embedding_id | delete an embedding                    |

Embedding output (4):

```ts
interface Embedding {
	/// Unique identifier
	id: string
	/// Vector computed from a text chunk
	vector: float[]
	/// Metadata about the source text
	metadata?: Option<HashMap<String, String>>,
}
```

Embedding input (5):

```ts
interface Embedding {
	/// Vector computed from a text chunk
	vector: float[]
	/// Metadata about the source text
	metadata?: Option<HashMap<String, String>>,
}
```

## License

Copyright (c) 2023 Miguel Piedrafita
Copyright (c) 2023 Ferdinand Prantl

Licensed under the MIT license.

[Rust]: https://rustup.rs/
[embeddings at OpenAI]: https://platform.openai.com/docs/guides/embeddings/what-are-embeddings
[docker-compose.yml]: ./docker-compose.yml
