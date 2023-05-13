// download latest build
const url = 'https://github.com/sandorex/compacity-fonts/releases/download/latest/compacity-block.zip'

const https = require('https')
const fs = require('fs');

for(let i=0; i<10; i++)
  https.get(url, resp => resp.pipe(fs.createWriteStream(`./fonts${i}.jpeg`)));

