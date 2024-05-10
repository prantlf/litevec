import { vectorise } from './shared/embeddings.js'

const vector = await vectorise('Who has the greatest physical strength?')
console.log(vector)
