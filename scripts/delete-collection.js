import { drop } from './shared/safe-fetch.js'
import settings from './shared/settings.json' assert { type: 'json' }
const { vectorDbUrl, collection } = settings

await drop(`${vectorDbUrl}/collections/${collection}`)
