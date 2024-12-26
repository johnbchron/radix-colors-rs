import * as colors from '@radix-ui/colors';
import fs from 'fs';

// Remove any function properties and stringify
const cleanObj = obj => JSON.parse(JSON.stringify(obj));

// Convert each color scale to JSON
const colorScales = {};
for (const [name, scale] of Object.entries(colors)) {
  if (typeof scale === 'object') {
    colorScales[name] = cleanObj(scale);
  }
}

fs.writeFileSync('colors.json', JSON.stringify(colorScales, null, 2));
