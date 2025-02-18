// test import
import * as _1 from '@myriaddreamin/typst-ts-renderer';
import * as _2 from '@myriaddreamin/typst-ts-web-compiler';

import {
  createTypstCompiler,
  createTypstRenderer,
} from '@myriaddreamin/typst.ts/dist/cjs/index.cjs';
import { cachedFontInitOptions } from './cached-font-middleware';

async function main() {
  const compiler = createTypstCompiler();
  await compiler.init(await cachedFontInitOptions());

  compiler.addSource('/main.typ', 'Hello, typst!');
  const artifactData = await compiler.compile({
    mainFilePath: '/main.typ',
  });

  const renderer = createTypstRenderer();
  await renderer.init();
  const svg = await renderer.runWithSession(async session => {
    renderer.manipulateData({
      renderSession: session,
      action: 'reset',
      data: artifactData,
    });
    return renderer.renderSvg({
      renderSession: session,
    });
  });

  console.log('Renderer works exactly! The rendered SVG file:', svg.length);
}

main();
