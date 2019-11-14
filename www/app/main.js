const submitButton = document.querySelector("#submitButton");
const result = document.querySelector("#result");
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const WIDTH = 200;
const HEIGHT = 100;
canvas.width = WIDTH;
canvas.height = HEIGHT;

const IMG = new Uint8ClampedArray(WIDTH * HEIGHT * 4);
let count = HEIGHT;
let received = 0;
let t0;

submitButton.addEventListener("click", async () => {
  submitButton.disabled = true;
  const workerCount = Number(document.getElementById('workerNum').value);
  let workers = [];
  for (let i = 0; i < workerCount; i++) {
    workers[i] = new Worker("./worker.js");
  }

  await Promise.all(workers.map(x => loaded(x)));

  for (let i = 0; i < workers.length; i++) {
    let worker = workers[i];

    worker.addEventListener("message", ev => {
      const message = ev.data;
      if (message.allGood === true) {
        ctx.putImageData(message.imgData, 0, 0);
        result.textContent = message.time;
      } else if (message.allGood === "ready") {
        console.log(`worker ${i} is ready`);
        count--;
        worker.postMessage({
          job: true,
          count: count
        });
      } else if (message.allGood === false) {
        result.textContent = "Something went wrong! " + message.error;
      } else {
        received++;
        for (let i = 0; i < message.imgRow.length; i++) {
          IMG[((HEIGHT - message.row) * WIDTH * 4) + i] = message.imgRow[i];
        }
        if (received % 10 === 0) {
          let imageData = new ImageData(IMG, WIDTH, HEIGHT);
          ctx.putImageData(imageData, 0, 0);
        }
        count--;
        if (count >= 0) {
          worker.postMessage({
            job: true,
            count: count
          });
        }
        result.textContent = `${received * 100 / HEIGHT}% complete`
        if (received === HEIGHT) {
          let imageData = new ImageData(IMG, WIDTH, HEIGHT);
          ctx.putImageData(imageData, 0, 0);
          let t1 = performance.now();
          result.textContent = `done in ${t1 - t0} ms`;
        }
      }
    });
  }

  t0 = performance.now();

  import("../../pkg")
    .then(wasm => {
      const world = wasm.scene_gen_json();
      for (let i = 0; i < workers.length; i++) {
        workers[i].postMessage({
          init: true,
          width: WIDTH,
          height: HEIGHT,
          world: world
        });
      }
    });
});

// helper to ensure worker is loaded before taking any jobs
const loaded = w => new Promise(r => w.addEventListener("message", r, {
  once: true
}));
