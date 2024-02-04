import { putJsonToJson } from './shared/safe-fetch.js'
import { vectorDbUrl, collection, dimension, distance } from './shared/settings.js'

await putJsonToJson(`${vectorDbUrl}/collections/${collection}`, { dimension, distance })
