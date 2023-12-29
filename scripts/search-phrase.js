import { search, vectorise } from './shared/embeddings.js'

const vector = await vectorise('Who has the greatest physical strength?')
await search(vector, [
  { name: 'classes/barbarian' }, { name: 'classes/druid' }, { name: 'classes/fighter' },
  { name: 'classes/monk' }, { name: 'classes/paladin' }, { name: 'classes/warlock' }
])
