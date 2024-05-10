# Changes

## [0.8.0](https://github.com/prantlf/litevec/compare/v0.7.0...v0.8.0) (2024-05-10)

### Features

* Move mappable storage volume from /litevec/storage to /storage ([5f60051](https://github.com/prantlf/litevec/commit/5f60051a52a4a709627bb85ada7194b265b1fb38))
* Add /api/ping for healthcheck ([40b4145](https://github.com/prantlf/litevec/commit/40b4145c7fe13f73569f9d178e9f5534ecd72869))
* Support method HEAD for /ping ([fd627a5](https://github.com/prantlf/litevec/commit/fd627a5c1a6389eeca0f41126d2b2b1a733fee3c))
* Use hash map and set instead of arrays to improve embedding and collection lookups ([ac24883](https://github.com/prantlf/litevec/commit/ac248835fd8bddafaad407e191a62e9b6b6deb19))
* Add healthcheck to docker-compose example ([7a64cfb](https://github.com/prantlf/litevec/commit/7a64cfbebc44e16112b8567540e7e66ad2eb3e27))

### Bug Fixes

* Do not try migrating old database from empty directory ([bdd606b](https://github.com/prantlf/litevec/commit/bdd606bca703b919f1a9d3fccf72798898cef24f))
* Upgrade dependencies ([3fc959e](https://github.com/prantlf/litevec/commit/3fc959ed47b4a6727c8ac6cc8420fe35dd3dc959))

### BREAKING CHANGES

Replace the target volume mapping `/litevec/storage`
with the new value `/storage`. (Internally, the executable moved from
`/litevec/litevec` to `/litevec`.

## [0.7.0](https://github.com/prantlf/litevec/compare/v0.6.2...v0.7.0) (2024-05-06)

### Features

* Let storage directory set by LITEVEC_STORAGE ([67acf87](https://github.com/prantlf/litevec/commit/67acf872f680f80c00d35f0c08f14d8efa7612a1))

### Bug Fixes

* Upgrade dependencies ([4ed2e02](https://github.com/prantlf/litevec/commit/4ed2e02b4f0dc0886148e36b1d979265b3e53461))

## [0.6.2](https://github.com/prantlf/litevec/compare/v0.6.1...v0.6.2) (2024-05-06)

### Bug Fixes

* Publish docker image in the same workflow ([f3888c3](https://github.com/prantlf/litevec/commit/f3888c3ac2eaf228c800709652d3a5f569aefda9))

## [0.6.1](https://github.com/prantlf/litevec/compare/v0.6.0...v0.6.1) (2024-05-06)

### Bug Fixes

* Build linux arm64 binary ([9faf09d](https://github.com/prantlf/litevec/commit/9faf09d722bd1d0737f9f9a191190b762030a44e))

## [0.6.0](https://github.com/prantlf/litevec/compare/v0.5.0...v0.6.0) (2024-05-06)

### Features

* Store collections in separate files ([778f1a7](https://github.com/prantlf/litevec/commit/778f1a7cbd76d422a51504d78a4be14b8fcbd846))

### Bug Fixes

* Upgrade dependencies ([ad18130](https://github.com/prantlf/litevec/commit/ad181302c3e0a69dc15df4fe459d059f987ed908))

### BREAKING CHANGES

There is no one single file `storage/db` any more.
Every collection is stored with its name: `storage/<encoded-name>`.
The `encoded-name` is the collection name encoded as a URI component.
The previous storage file `storage/db` will be loaded when the service
starts, separate files will be written and the old `db` file deleted.
An empty file `storage/._collections` will be created to mark the new
storage version. Do not delete it.

## [0.5.0](https://github.com/prantlf/litevec/compare/v0.4.0...v0.5.0) (2024-02-05)

### Features

* Add URL parameter novector ([f3cce2e](https://github.com/prantlf/litevec/commit/f3cce2e7009be7148450af43d38b0da254576ac7))

### Bug Fixes

* Upgrade dependencies ([d6c2a6c](https://github.com/prantlf/litevec/commit/d6c2a6c7001556981fb474318131104ae60c57c0))

## [0.4.0](https://github.com/prantlf/litevec/compare/v0.3.0...v0.4.0) (2024-01-07)

### Features

* Support renaming of a collection ([d4fda4b](https://github.com/prantlf/litevec/commit/d4fda4b9db02546cba4fd1b2cc09fb38d8a73eef))
* Support updating metadata of an embedding ([19b39d5](https://github.com/prantlf/litevec/commit/19b39d5cc24399678829b04059e42ec098037a6d))

### Bug Fixes

* Upgrade dependencies ([f1c8250](https://github.com/prantlf/litevec/commit/f1c82509cb028eee286c7fdfc1315d8747ce8264))

## [0.3.0](https://github.com/prantlf/litevec/compare/v0.2.0...v0.3.0) (2024-01-01)

### Features

* Ensure the database saved every 10 seconds ([99b66c0](https://github.com/prantlf/litevec/commit/99b66c03d43bd86908f7d836e41030032eb49472))
* Enable debug logging by default ([d329e19](https://github.com/prantlf/litevec/commit/d329e190f48dfead1ec254097bb280597dd778be))

## [0.2.0](https://github.com/prantlf/litevec/compare/v0.1.0...v0.2.0) (2023-12-31)

### Features

* Load .env files automatically ([0ab1300](https://github.com/prantlf/litevec/commit/0ab1300966ab4e486450de9bf460663404c21ad1))

### Bug Fixes

* Fix the image name in the docker-compose configuration ([2c903e3](https://github.com/prantlf/litevec/commit/2c903e3b528689eea651e6843069b081299d489e))

## [0.1.0](https://github.com/prantlf/litevec/compare/v0.0.1...v0.1.0) (2023-12-29)

### Features

* Implement graceful shutdown ([4bc9256](https://github.com/prantlf/litevec/commit/4bc9256e914811c1d09009d74300905567cd5c59))
* Support request compression, cors, timeout and validation ([33dc8d6](https://github.com/prantlf/litevec/commit/33dc8d6341809bfea6f1b8f32006d00a5913b68e))
* Add configuration for docker-compose ([77fc3fa](https://github.com/prantlf/litevec/commit/77fc3faec59a612bff321967b3e45337cd1c0b35))
* Compress only responses larger than 1KB by default ([d413fef](https://github.com/prantlf/litevec/commit/d413fef7a4ed067fb2a44484324185f578d1b5aa))

### Bug Fixes

* Create a new embedding by PUT (#3) ([ca35af4](https://github.com/prantlf/litevec/commit/ca35af436091e068f5b2c957b1d0e9d8b348bb3d))
* Upgrade dependencies ([388a68a](https://github.com/prantlf/litevec/commit/388a68a8cae3eedc839191d439ee8848594744d4))
* Wait in /shutdown until database is written in other handlers ([8ce6a7a](https://github.com/prantlf/litevec/commit/8ce6a7adf5c82a9fba8445c11c21cab037c201a9))
* Remove logging duplicating axum request entries ([47c0e3f](https://github.com/prantlf/litevec/commit/47c0e3fe12eda5db006581a67ca9bcafa1ad677d))
* Add libss3 to the Docker image ([77b6e41](https://github.com/prantlf/litevec/commit/77b6e41a5cbfbb09ce06c3cf87c2b199843b5d29))
* Remove unused k from the deletion query payload ([45493c2](https://github.com/prantlf/litevec/commit/45493c2503d1a14debeb7fb36bf22bd9ffb047ff))
* Remove unused embeddings from the collection creation payload ([188dc66](https://github.com/prantlf/litevec/commit/188dc6652aa954a2c903ee3bf29baec881d5a7e0))

### Chores

* Rename the package to litevec ([26b82f3](https://github.com/prantlf/litevec/commit/26b82f31c2859f4819ed2e2a58fb9667b4717046))

### BREAKING CHANGES

* Instead of POST /collections/:collection_name/insert, use PUT /collections/:collection_name/embeddings/:embedding_id.
* Binary executable, Docker image name and logging identifier were renamed from `tinyvector` to `litevec`.

## 0.0.1 (2023-12-27)

Forking the original project.
