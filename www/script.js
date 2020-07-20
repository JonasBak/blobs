const rust = import("../pkg");

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

rust
  .then((m) => {
    m.test_canvas(ctx);
    m.flip_canvas(ctx);
  })
  .catch(console.error);
