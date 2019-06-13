
'use strict';

var VERSION = require('./package.json').version;

var path = require('path');

module.exports =
  process.platform === 'darwin'
    ? path.join(__dirname, 'tagger-osx64', 'tagger') :
    process.platform === 'linux' && process.arch === 'x64'
      ? path.join(__dirname, 'tagger-linux64', 'tagger') :
      process.platform === 'win32' && process.arch === 'x64'
        ? path.join(__dirname, 'tagger-win64', 'tagger.exe') :
        null;
