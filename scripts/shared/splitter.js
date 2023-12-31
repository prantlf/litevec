export function splitText(text) {
  const pars = []
  let title, chapter
  let page = 0
  const chunks = text.split(/(?:(?:\r?\n)|(?:\r)){2}/)
  for (let i = 0; i < chunks.length; ++i) {
    const partext = chunks[i].trim().replace(/\s*(?:\r?\n)?\s*(?:=|-)+$/, '')
    if (/^\w.*\w$/.test(partext)) {
      if (!title && !pars.len) {
        title = partext
      } else {
        chapter = partext
        ++page
      }
    } else {
      pars.push({ partext, page, chapter })
    }
  }
  return { title, pars }
}
