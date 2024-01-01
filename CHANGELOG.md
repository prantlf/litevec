# Changes

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
