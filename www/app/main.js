const worker = new Worker("./worker.js");

const submitButton = document.querySelector("#submitButton");
const result = document.querySelector("#result");
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

worker.addEventListener("message", ev => {
  const message = ev.data;
  if (message.allGood === true) {
        ctx.putImageData(message.imgData, 0, 0);
        result.textContent = message.time;
  } else if (message.allGood === "processing") {
        result.textContent = message.status;
  } else if (message.allGood === false) {
        result.textContent = "Something went wrong! " + message.error;
  } else {
        let imageData = new ImageData(message, 200);
        ctx.putImageData(imageData, 0, 0);
  }
});

submitButton.addEventListener("click", () => {
    import("../../pkg").then(wasm => {
        const WIDTH = 200;
        const HEIGHT = 100;
        const world = wasm.scene_gen_json();
        worker.postMessage({init: true, width: WIDTH, height: HEIGHT, world: world});
    });
} );
