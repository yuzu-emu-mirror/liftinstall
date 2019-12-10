#!/bin/env node
const fs = require('fs')
const merge = require('deepmerge')
const glob = require('glob')

glob('src/locales/!(messages).json', {}, (e, files) => {
  let messages = []
  for (const file of files) {
    console.log(`Loading ${file}...`)
    const locale_messages = require(`./${file}`)
    messages.push(locale_messages)
  }
  console.log('Merging messages...')
  messages = merge.all(messages)
  fs.writeFileSync('src/locales/messages.json', JSON.stringify(messages), {})
})
