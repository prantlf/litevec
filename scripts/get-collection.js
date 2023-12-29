import { getJson } from './shared/safe-fetch.js'
import settings from './shared/settings.json' assert { type: 'json' }
const { vectorDbUrl, collection } = settings

console.log(await getJson(`${vectorDbUrl}/collections/${collection}`))
