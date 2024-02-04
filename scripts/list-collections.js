import { getJson } from './shared/safe-fetch.js'
import { vectorDbUrl } from './shared/settings.js'

console.log(await getJson(`${vectorDbUrl}/collections`))
