import index from "./index.svelte";

const app = new index({
  target: document.getElementById("app")!,
});

export default app;
