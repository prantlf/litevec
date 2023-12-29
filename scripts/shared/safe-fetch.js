async function fetchSafely(url, options) {
  const res = await fetch(url, options)
  if (!res.ok) {
    let text
    try {
      text = await res.text()
    } catch {
      text = res.statusText
    }
    throw new Error(`${options.method} ${new URL(url).pathname} failed with ${res.status}: ${text}`)
  }
  return res
}

async function getOptionalJson(res) {
  const text = await res.text()
  return text && JSON.parse(text)
}

export async function getJson(url, method = 'GET') {
  const res = await fetchSafely(url, {
    method,
    headers: { Accept: 'application/json' }
  })
  return res.json()
}

export async function drop(url) {
  return fetchSafely(url, { method: 'DELETE' })
}

export async function postJson(url, body, method = 'POST') {
  return fetchSafely(url, {
    method,
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json'
    },
    body: body && JSON.stringify(body)
  })
}

export async function postJsonToJson(url, body, method = 'POST') {
  const res = await postJson(url, body, method)
  return getOptionalJson(res)
}

export async function putJsonToJson(url, body) {
  return postJsonToJson(url, body, 'PUT')
}
