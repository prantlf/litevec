import { postJson } from './shared/safe-fetch.js'
import settings from './shared/settings.json' assert { type: 'json' }
const { vectorDbUrl } = settings

await postJson(`${vectorDbUrl}/shutdown`)
