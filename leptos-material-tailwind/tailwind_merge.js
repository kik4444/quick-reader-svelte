import { twMerge } from "tailwind-merge";

// This file is bundled so it can be called from WASM
// ./node_modules/esbuild/bin/esbuild tailwind_merge.js --bundle --platform=neutral --minify --outfile=js_bundle/tailwind_merge.js

export function tailwind_merge(existingClasses, newClasses) {
    return twMerge(existingClasses, newClasses);
}
