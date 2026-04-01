#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

module.exports.readVersion = function (contents) {
  const versionMatch = contents.match(/^version\s*=\s*"([^"]+)"/m);
  return versionMatch ? versionMatch[1] : '';
};

module.exports.writeVersion = function (contents, version) {
  return contents.replace(/^version\s*=\s*"[^"]+"/m, `version = "${version}"`);
};
