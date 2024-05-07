import { getJson } from './shared/safe-fetch.js'
import { vectorDbUrl } from './shared/settings.js'

await getJson(`${vectorDbUrl}/ping`)
