const port = Number(Bun.env.PORT || 8000);

Bun.serve({
  hostname: "0.0.0.0",
  port,
  fetch() {
    return new Response("frameworkless bun ok\n");
  },
});
