import { postJson } from './shared/safe-fetch.js'
import { vectorDbUrl } from './shared/settings.js'

await postJson(`${vectorDbUrl}/shutdown`)
