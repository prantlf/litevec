export function breakDownText(text) {
  const lines = text.split(/(?:\r?\n)/)
  const [title] = lines
  const pars = []
  let page = 0
  let chapter, subtitle
  let sentences = []
  for (let i = 3; i < lines.length; ++i) {
    const line = lines[i].trim()
    if (line && /\w/.test(line)) {
      if (/^\w.*\w$/.test(line)) {
        if (!title) {
          title = line
        } else {
          subtitle = line
        }
      } else if (/^[^\w]+\w/.test(line)) {
        sentences.push(line)
      } else {
        if (subtitle) {
          ++page
          chapter = subtitle
          subtitle = undefined
        }
        let partext
        if (sentences.length) {
          partext = sentences.join('\n')
          sentences = []
        } else {
          partext = line
        }
        pars.push({ partext, page, chapter })
      }
    }
  }
  return { title, pars }
}
