import { drop } from './shared/safe-fetch.js'
import { vectorDbUrl, collection } from './shared/settings.js'

await drop(`${vectorDbUrl}/collections/${encodeURIComponent(collection)}`)
