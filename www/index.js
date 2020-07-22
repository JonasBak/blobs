let selectedFilter = 0;
let selectedInput = "image";

const selectFilter = document.getElementById("selectFilter");
selectFilter.addEventListener("change", (e) => {
  const v = parseInt(e.target.value);
  if (!isNaN(v)) {
    selectedFilter = v;
  }
});

const wasmPromise = import("../pkg");
let wasm;
wasmPromise.then((m) => {
  wasm = m;
  m.get_filters().forEach((filter, i) => {
    const node = document.createElement("option");
    node.value = String(i);
    node.innerText = filter;
    selectFilter.appendChild(node);
  });
});

const selectInput = document.getElementById("selectInput");
selectInput.addEventListener("change", (e) => {
  selectedInput = e.target.value;
});

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const video = document.getElementById("webcamVideo");

import testImageSrc from "./testImage.jpg";
const testImage = document.getElementById("testImage");
testImage.src = testImageSrc;
const imagePromise = new Promise((resolve) =>
  testImage.addEventListener("load", resolve)
);

const render = async () => {
  if (selectedInput === "image") {
    ctx.drawImage(testImage, 0, 0, canvas.width, canvas.height);
  } else if (selectedInput === "webcam") {
    if (navigator.mediaDevices.getUserMedia && !video.srcObject) {
      const stream = await navigator.mediaDevices.getUserMedia({
        video: true,
      });
      video.srcObject = stream;
    }
    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
  }
  wasm.apply_filter(ctx, selectedFilter);
  requestAnimationFrame(render);
};

const start = async () => {
  await wasmPromise;
  await imagePromise;
  requestAnimationFrame(render);
};

start();
