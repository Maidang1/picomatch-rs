'use strict';

const basename = (str, options = {}) => {
  const sep = options.windows ? /[\\/]/ : /\//;
  const parts = str.split(sep).filter(part => part !== '');
  return parts.length > 0 ? parts[parts.length - 1] : '';
};

module.exports = { basename };
