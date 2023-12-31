import { readdir, readFile } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'
import { splitText } from './shared/splitter.js'
import { index, vectorise } from './shared/embeddings.js'

const __dirname = dirname(fileURLToPath(import.meta.url))
const datadir = join(__dirname, '../data')

async function indexParagraph(title, name, chapter, page, partext, parnum) {
  const vector = await vectorise(partext)
  await index(title, name, chapter, page, partext, parnum, vector)
}

async function indexFile(group, file) {
  const name = `${group}/${file.slice(0, -4)}`
  console.log(name)
  const text = await readFile(join(datadir, group, file), 'utf8')
  const { title, pars } = splitText(text)
  // await Promise.all(text.map(({ chapter, page, partext }, parnum) =>
  //   postParagraph(title, name, chapter, String(page), partext, String(parnum))))
  for (let parnum = 0; parnum < pars.length; ++parnum) {
    const { chapter, page, partext } = pars[parnum]
    await indexParagraph(title, name, chapter, page, partext, parnum)
  }
}

async function indexGroup(group) {
  console.log(group)
  const files = await readdir(join(datadir, group))
  // await Promise.all(files
  //   .filter(file => file.endsWith('.txt'))
  //   .map(file => postFile(group, file)))
  for (const file of files) {
    if (file.endsWith('.txt')) await indexFile(group, file)
  }
}

const groups = await readdir(datadir)
// await Promise.all(groups
//   .filter(dir => dir !== 'extra' && !dir.includes('.'))
//   .map(postGroup))
for (const dir of groups) {
  if (dir !== 'extra' && !dir.includes('.')) await indexGroup(dir)
}
