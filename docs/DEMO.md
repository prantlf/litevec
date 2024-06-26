# Demo Example

This example shows how to index and search through several topics from [Dungeons and Dragons]. Selected 12 character classes ([data/classes]) and 4 guilds ([data/guilds]) are used as testing data. Helpful scripts runnable with [Node.js] or [Bun] ([scripts]) can be used instead of `curl`. Vectors are computed using [ollama], which offers a REST API for integration to a bigger application, similarly to this vector database.

- [Create a Collection](#create-a-collection)
- [Upload Documents](#upload-documents)
- [Search by a Phrase](#search-by-a-phrase)
- [Other Scripts](#other-scripts)

## Create a Collection

A collection is needed to be able to index and search through any content. The vector dimension has to be set according to the algorithm, which you use to compute the vectors and it cannot be changed later. Examples of LLMs, which can be used with [ollama] to compute vectors:

| Model     | Parameters | Dimension |
|:----------|:-----------|:----------|
| phi       | 2.7B       | 2560      |
| orca-mini | 3B         | 3200      |
| mistral   | 7B         | 4096      |

Perform:

    node scripts/create-collection.js

Behind:

    curl -X PUT -s -w "%{http_code}" http://localhost:8000/collections/dnd \
      -d '{ "dimension": 4096, "distance": "cosine" }' \
      -H "Content-Type: application/json"

    201

## Upload Documents

Documents are indexed in a form of vectorised chunks - embeddings. This is performed by:

1. splitting the text to reasonable parts, usually paragraphs consisting of sentences about one topic.
2. converting the parts to their vector representation.
3. storing the vectors with optional metadata in a vector database.

This demo expects nicely prepared textual documents, for example:

    Barbarian
    =========

    A tall human tribesman strides through a blizzard, ... who dared poach his people’s elk herd.

    ...

    Primal Instinct
    ---------------

    People of towns and cities take pride in their settled ways, ... where their tribes live and hunt.

* Title, subtitles and paragraphs are separated by two line breaks.
* A line starting and ending with a letter is considered a title. The first one is a document title, the others are chapter titles.
* Each chapter is considered to start on a new page.

Perform:

    node scripts/index-documents.js

Behind:

This script loads filed `data/classes/*.txt` and `data/guilds/*.txt` and splits them to paragraphs according to rules above. Vectors are computed using [ollama] like this:

    curl -X POST http://localhost:11434/embeddings \
    -d '{ "model": "mistral", "prompt": { "A tall human tribesman strides ..." } }'

    { "embedding": [ 0.0033867622, 0.008273851, ..., 0.017800305, -0.01118711 ] }

An embedding is uploaded to the vector database like this:

    curl -X PUT http://localhost:8000/collections/dnd/embeddings/classes%2Fbarbarian-0-0 \
    -d '{ "vector": [ 0.0033867622, 0.008273851, ..., 0.017800305, -0.01118711 ], \
          "metadata": { "title": "Barbarian", "name": "classes/barbarian", "page": "0", \
                        "parnum": "0", "partext": "A tall human tribesman strides ..." } }' \
    -H "Content-Type: application/json"

    201

Identifier and metadata of an embedding are inferred from the file name and content:

| Property | Pattern                   | Example                            | Note              |
|:---------|:--------------------------|:-----------------------------------|:------------------|
| id       | {path}-{page}-{paragraph} | classes/barbarian-0-0              |                   |
| title    | {title}                   | Barbarian                          |                   |
| name     | {path}                    | classes/barbarian                  | without extension |
| page     | {page}                    | 0                                  | zero-based        |
| parnum   | {paragraph}               | 0                                  | zero-based        |
| partext  | {paragraph content}       | A tall human tribesman strides ... |                   |

## Search by a Phrase

Perform:

    node scripts/search-phrase.js

Behind:

A phrase needs to be vectorised at first, to be able to perform a similarity search among the vectors in the database. This is done in he same way as computing a vector for a paragraph of text:

    curl -X POST http://localhost:11434/embeddings \
    -d '{ "model": "mistral", "prompt": { "Who has the greatest physical strength?" } }'

    { "embedding": [ 0070150318, 0.008992326, ..., -0.002473238, 0.00245696 ] }

Then the search can be conducted:

    curl -X POST -s http://localhost:8000/collections/dnd \
      -d '{ "query": [ 0070150318, 0.008992326, ..., -0.002473238, 0.00245696 ], "k": 5 }' \
      -H "Content-Type: application/json"

    [ { "id": "classes/barbarian-0-0", "vector": [ 0.0033867622, 0.008273851, ..., 0.017800305, -0.01118711 ],
        "metadata": { "parnum": "0", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } },
      ...,
      { "id": "classes/barbarian-0-4", "vector": [ 0.01261057, 0.003335859, ..., 0.0024617626,-0.0025066733 ],
        "metadata": { "parnum": "4", "title": "Barbarian", "name": "classes/barbarian", "page": "0" } } ]

## Other Scripts

Remaining [scripts] demonstrate other use cases for the REST API. They have no parameters. They work with the embedding collection `dnd` and the LLM model `mistral`. The common configuration is in [scripts/shared/settings.js].

| Path                 | Description                                            |
|:---------------------|:-------------------------------------------------------|
| create-collection.js | create a new collection                                |
| delete-collection.js | create the collection                                  |
| get-collection.js    | prints information about the collection                |
| get-embedding.js     | prints information about an embedding                  |
| index-documents.js   | indexes sample  documents in the collection            |
| list-embeddings.js   | prints identifiers of all embeddings in the collection |
| search-phrase.js     | searches for embeddings similar to a phrase            |
| ping.js              | checks that the server is running                      |
| shutdown.js          | shuts down the service                                 |
| shared/embeddings.js | common functions to manage embeddings                  |
| shared/splitter.js   | text-splitting function                                |
| shared/safe-fetch.js | wrappers for network requests                          |
| shared/settings/json | common parameters for all scripts                      |

### Configuration

The scripts recognise the following environment variables:

| Name               | Default                    | Description                                                            |
|:-------------------|:---------------------------|:-----------------------------------------------------------------------|
| LITEVEC_URL        | http://127.0.0.1:8000      | base URL of the vector database service                                |
| LITEVEC_COLLECTION | dnd-phi                    | name of the collection to work with                                    |
| LITEVEC_DIMENSION  | 2560                       | dimension of the collection (phi: 2560, orca-mini: 3200, mistral 4096) |
| LITEVEC_DISTANCE   | cosine                     | algorithm for computing the vector distance (cosine, dot or euclidean) |
| LITEVEC_K          | 10                         | maximum number of embeddings returned by the similarity search         |
| LITEVEC_LLM_URL    | http://127.0.0.1:11434/api | base URL of the LLM service                                            |
| LITEVEC_LLM_MODEL  | phi                        | name of the LLM model (phi, orca-mini, mistral)                        |

For example, use different models:

    # phi
    node scripts/create-collection.js
    node scripts/index-documents.js
    # orca-mini
    LITEVEC_COLLECTION=dnd-orca-mini LITEVEC_DIMENSION=3200 node scripts/create-collection.js
    LITEVEC_COLLECTION=dnd-orca-mini LITEVEC_LLM_MODEL=orca-mini node scripts/index-documents.js
    # mistral
    LITEVEC_COLLECTION=dnd-mistral LITEVEC_DIMENSION=3200 node scripts/create-collection.js
    LITEVEC_COLLECTION=dnd-mistral LITEVEC_LLM_MODEL=mistral node scripts/index-documents.js

[Dungeons and Dragons]: https://www.dndbeyond.com
[data/classes]: ../data/classes
[data/guilds]: ../data/guilds
[scripts]:  ../scripts
[Node.js]: https://nodejs.org
[Bun]: https://bun.sh
[ollama]: https://ollama.ai
[scripts/shared/settings.js]: ../scripts/shared/settings.js
