const wasmPromise = import("../pkg");
let wasm;
wasmPromise.then((m) => {
  wasm = m;
});

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const video = document.getElementById("webcamVideo");
let renderVideo = false;

import TestImage from "./testImage.jpg";
const image = document.getElementById("testImage");
image.src = TestImage;
const imagePromise = new Promise((resolve) =>
  image.addEventListener("load", resolve)
);

const setInputType = async (t) => {
  renderVideo = false;
  await wasmPromise;
  if (t === "image") {
    await imagePromise;
    ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
    wasm.apply_filter(ctx);
  } else if (t === "webcam") {
    if (!navigator.mediaDevices.getUserMedia) {
      return;
    }
    const stream = await navigator.mediaDevices.getUserMedia({
      video: { width: { exact: 640 }, height: { exact: 480 } },
    });
    video.srcObject = stream;
    const renderWebcam = () => {
      if (!renderVideo) return;
      ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
      wasm.apply_filter(ctx);
      requestAnimationFrame(renderWebcam);
    };
    renderVideo = true;
    requestAnimationFrame(renderWebcam);
  }
};

const selectInput = document.getElementById("selectInput");
setInputType("image");
selectInput.addEventListener("change", (e) => setInputType(e.target.value));
