import { twMerge } from "tailwind-merge";

// This file is bundled so it can be called from WASM

export function tailwind_merge(existingClasses, newClasses) {
    return twMerge(existingClasses, newClasses);
}
