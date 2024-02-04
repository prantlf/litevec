# REST API Endpoints

- [System](#system) - control the server
- [Collections](#collections) - manage collections of embeddings and search by similarity and metadata
- [Embeddings](#embeddings) - manage embeddings in a collections and filter by metadata

## System

| Method | Path | Description         |
|:-------|:-----|:--------------------|
| GET    | /    | obtain API metadata |

Example:

    curl -X GET -s http://localhost:8000

    { "docs_url": "/docs", "openapi_url": "/openapi.json",
      "version": { "semver": "0.0.1", "rev": "7c20d76",
                   "compile_time": "2023-12-28T20:16:35.888568+00:00" } }

| Method | Path      | Description           |
|:-------|:----------|:----------------------|
| POST   | /shutdown | shut the service down |

Example:

    curl -X POST -s -w "%{http_code}" http://localhost:8000/shutdown

    204

## Collections

| Method | Path         | Description           |
|:-------|:-------------|:----------------------|
| GET    | /collections | list collection names |

Example:

    curl -X GET -s http://localhost:8000/collections

    ["dnd"]

| Method | Path                          | Description                                                                 |
|:-------|:------------------------------|:----------------------------------------------------------------------------|
| POST   | /collections/:collection_name | search the collection for similar vectors while filtering with metadata too |

```ts
interface EmbeddingParams {
  /// Omits the vector from the embedding data in the response
  novector: Option<bool>
}

interface SearchInput {
  /// Vector to query with
  query: float[]
  /// Metadata to filter with
  filter?: Record<String, String>
  /// Number of results to return
  k?: integer
}

interface SearchOutput {
  /// Similarity score
  score: float
  /// Matching embedding
  embedding: EmbeddingOutput
}

interface EmbeddingOutput {
  /// Unique identifier
  id: string
  /// Vector computed from a text chunk
  vector: float[]
  /// Metadata about the source text
  metadata?: Record<String, String>
}
```

Example:

    curl -X POST -s http://localhost:8000/collections/dnd \
      -d '{ "query": [ 0070150318, 0.008992326, ..., -0.002473238, 0.00245696 ], "k": 5 }' \
      -H "Content-Type: application/json"

    [ { "id": "classes/barbarian-0-0", "vector": [ 0.0033867622, 0.008273851, ..., 0.017800305, -0.01118711 ],
        "metadata": { "parnum": "0", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } },
      ...,
      { "id": "classes/barbarian-0-4", "vector": [ 0.01261057, 0.003335859, ..., 0.0024617626,-0.0025066733 ],
        "metadata": { "parnum": "4", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } } ]

The URL parameter `?novector=true` will omit the vectors from the response. Vectors are usually used for querying the embeddings, but they are usually not needed once the embedding has been found and its metadata obtained.

| Method | Path                          | Description         |
|:-------|:------------------------------|:--------------------|
| PUT    | /collections/:collection_name | create a collection |

```ts
interface CollectionInput {
  /// Dimension of the vectors in the collection
  dimension: integer
  /// Distance metric used for querying
  distance: 'cosine' | 'dot' | 'euclidean'
}
```

Example:

    curl -X PUT -s -w "%{http_code}" http://localhost:8000/collections/dnd \
      -d '{ "dimension": 4096, "distance": "cosine" }' \
      -H "Content-Type: application/json"

    201


| Method | Path                          | Description         |
|:-------|:------------------------------|:--------------------|
| PATCH  | /collections/:collection_name | rename a collection |

```ts
interface CollectionUpdate {
  /// New name
  name: string
}
```

Example:

    curl -X PATCH -s -w "%{http_code}" http://localhost:8000/collections/dnd \
      -d '{ "name": "dnd-mistral" }' \
      -H "Content-Type: application/json"

    204

| Method | Path                          | Description                        |
|:-------|:------------------------------|:-----------------------------------|
| GET    | /collections/:collection_name | get information about a collection |

```ts
interface CollectionOutput {
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

Example:

    curl -X GET -s http://localhost:8000/collections/dnd

    { "name": "dnd", "dimension": 4096, "distance": "cosine", "embedding_count": 109 }

| Method | Path                          | Description         |
|:-------|:------------------------------|:--------------------|
| DELETE | /collections/:collection_name | delete a collection |

Example:

    curl -X DELETE -s -w "%{http_code}" http://localhost:8000/collections/dnd

    204

## Embeddings

| Method | Path                                     | Description                |
|:-------|:-----------------------------------------|:---------------------------|
| GET    | /collections/:collection_name/embeddings | list embedding identifiers |

Example:

    curl -X GET -s http://localhost:8000/collections/dnd/embeddings

    [ "classes/barbarian-0-0", "classes/barbarian-0-1", ...,
      "guilds/travelers-0-1", "guilds/travelers-0-2" ]

| Method | Path                                     | Description                     |
|:-------|:-----------------------------------------|:--------------------------------|
| POST   | /collections/:collection_name/embeddings | filter embeddings with metadata |

```ts
interface EmbeddingOutput {
  /// Unique identifier
  id: string
  /// Vector computed from a text chunk
  vector: float[]
  /// Metadata about the source text
  metadata?: Record<String, String>
}
```

Example:

    curl -X POST -s http://localhost:8000/collections/dnd/embeddings \
      -d '{ "filter": [ { "name": "classes/barbarian" } ], "k": 5 }' \
      -H "Content-Type: application/json"

    [ { "id": "classes/barbarian-0-0", "vector": [ 0.0033867622, 0.008273851, ..., 0.017800305, -0.01118711 ],
        "metadata": { "parnum": "0", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } },
      ...,
      { "id": "classes/barbarian-0-4", "vector": [ 0.01261057, 0.003335859, ..., 0.0024617626,-0.0025066733 ],
        "metadata": { "parnum": "4", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } } ]

| Method | Path                                     | Description                   |
|:-------|:-----------------------------------------|:------------------------------|
| DELETE | /collections/:collection_name/embeddings | delete embeddings by metadata |

Example:

    curl -X DELETE -s -w "%{http_code}" http://localhost:8000/collections/dnd/embeddings \
      -d '{ "filter": [ { "name": "classes/barbarian" } ] }' \
      -H "Content-Type: application/json"

    204

| Method | Path                                                   | Description         |
|:-------|:-------------------------------------------------------|:--------------------|
| PUT    | /collections/:collection_name/embeddings/:embedding_id | create an embedding |

```ts
interface EmbeddingInput {
  /// Vector computed from a text chunk
  vector: float[]
  /// Metadata about the source text
  metadata?: Record<String, String>
}
```

Example:

    curl -X PUT -s -w "%{http_code}" http://localhost:8000/collections/dnd/embeddings/classes%2Fbarbarian-0-0 \
    -d '{ "vector": [ 0.0033867622, 0.008273851, ..., 0.017800305, -0.01118711 ], \
          "metadata": { "parnum": "0", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } }' \
    -H "Content-Type: application/json"

    201

| Method | Path                                                   | Description         |
|:-------|:-------------------------------------------------------|:--------------------|
| PATCH  | /collections/:collection_name/embeddings/:embedding_id | update an embedding |

```ts
interface EmbeddingUpdate {
  /// Metadata about the source text
  metadata?: Record<String, String>
}
```

Example:

    curl -X PATCH -s -w "%{http_code}" http://localhost:8000/collections/dnd/embeddings/classes%2Fbarbarian-0-0 \
    -d '{ "metadata": { "parnum": "0", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } }' \
    -H "Content-Type: application/json"

    204

| Method | Path                                                   | Description                        |
|:-------|:-------------------------------------------------------|:-----------------------------------|
| GET    | /collections/:collection_name/embeddings/:embedding_id | get information about an embedding |


```ts
interface EmbeddingParams {
  /// Omits the vector from the embedding data in the response
  novector: Option<bool>
}

interface EmbeddingOutput {
  /// Unique identifier
  id: string
  /// Vector computed from a text chunk
  vector: float[]
  /// Metadata about the source text
  metadata?: Record<String, String>
}
```

Example:

    curl -X GET -s http://localhost:8000/collections/dnd/embeddings/classes%2Fbarbarian-0-0

    { "id": "classes/barbarian-0-0", "vector": [ 0.0033867622, 0.008273851, ..., 0.017800305, -0.01118711 ],
      "metadata": { "parnum": "0", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } }

The URL parameter `?novector=true` will omit the vector from the response. Vectors are usually used for querying the embeddings, but they are usually not needed once the embedding has been found and its metadata obtained.

| Method | Path                                                   | Description         |
|:-------|:-------------------------------------------------------|:--------------------|
| DELETE | /collections/:collection_name/embeddings/:embedding_id | delete an embedding |

Example:

    curl -X DELETE -s -w "%{http_code}" \
    http://localhost:8000/collections/dnd/embeddings/classes%2Fbarbarian-0-0

    204
