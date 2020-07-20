const wasm = import("../pkg");
import TestImage from "./testImage.jpg";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const image = document.getElementById("img");
image.src = TestImage;

image.addEventListener("load", (_) => {
  wasm
    .then((m) => {
      ctx.drawImage(image, 0, 0, 200, 200);
      m.apply_filter(ctx);
    })
    .catch(console.error);
});
