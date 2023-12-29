import { postJson, postJsonToJson } from './safe-fetch.js'
import settings from './settings.json' assert { type: 'json' }
const { vectorDbUrl, collection, k, llmUrl, model } = settings

export async function vectorise(prompt) {
  const start = performance.now()
  const { embedding } = await postJsonToJson(`${llmUrl}/embeddings`, { model, prompt })
  const duration = Math.trunc(performance.now() - start)
  console.log(`embedding: [${embedding[0]} and ${embedding.length - 1} others] (in ${duration}ms)`)
  return embedding
}

export async function index(title, name, chapter, page, partext, parnum, vector) {
  const start = performance.now()
  const id = `${name}-${page}-${parnum}`
  page = String(page)
  parnum = String(parnum)
  await postJson(`${vectorDbUrl}/collections/${collection}/insert`, {
    id, metadata: { title, name, chapter, page, partext, parnum }, vector
  })
  const duration = Math.trunc(performance.now() - start)
  console.log(`${id} indexed (in ${duration}ms)`)
}

export async function search(query, filter) {
  const start = performance.now()
  const results = await postJsonToJson(`${vectorDbUrl}/collections/${collection}`, {
    k,
    query,
    filter
  })
  const duration = Math.trunc(performance.now() - start)
  console.log(`found ${results.length} embeddings (in ${duration}ms)`)
  for (const result of results) {
    const { score, embedding } = result
    const { id, metadata, vector } = embedding
    const { partext } = metadata
    console.log(`  ${id}, score ${score}, ${partext.length} characters, [${vector[0]} and ${vector.length - 1} others]`)
  }
  return results
}
