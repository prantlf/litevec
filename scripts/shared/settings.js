const {
  LITEVEC_URL,
  LITEVEC_HOST,
  LITEVEC_PORT,
  LITEVEC_COLLECTION,
  LITEVEC_DIMENSION,
  LITEVEC_DISTANCE,
  LITEVEC_K,
  LITEVEC_LLM_URL,
  LITEVEC_LLM_MODEL
} = process.env

const host = LITEVEC_HOST === '0.0.0.0' && '127.0.0.1' || LITEVEC_HOST || '127.0.0.1'
const port = +(LITEVEC_PORT || 8000)

const vectorDbUrl = LITEVEC_URL || `http://${host}:${port}`
const collection = LITEVEC_COLLECTION || 'dnd-textembedding-gecko@003'
// phi: 2560, orca-mini: 3200, mistral 4096, textembedding-gecko@003 768
const dimension = +(LITEVEC_DIMENSION || 768)
// cosine, dot, euclidean
const distance = LITEVEC_DISTANCE || 'cosine'
const k = +(LITEVEC_K || 10)
const llmUrl = LITEVEC_LLM_URL || 'http://127.0.0.1:22434/api'
const model = LITEVEC_LLM_MODEL || 'textembedding-gecko@003'

export { vectorDbUrl, collection, dimension, distance, k, llmUrl, model }
