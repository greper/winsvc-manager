#!/usr/bin/env node

module.exports.readVersion = function (contents) {
  const versionMatch = contents.match(/^version\s*=\s*"([^"]+)"/m);
  return versionMatch ? versionMatch[1] : '';
};

module.exports.writeVersion = function (contents, version) {
  return contents.replace(/^version\s*=\s*"[^"]+"/m, `version = "${version}"`);
};
