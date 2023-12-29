import { putJsonToJson } from './shared/safe-fetch.js'
import settings from './shared/settings.json' assert { type: 'json' }
const { vectorDbUrl, collection, dimension, distance } = settings

await putJsonToJson(`${vectorDbUrl}/collections/${collection}`, { dimension, distance })
