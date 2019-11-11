import("../../pkg").then(wasm => {
  self.addEventListener("message", () => {
    try {
        let t0 = new Date().getTime();
        const WIDTH = 200;
        const HEIGHT = 100;
        const arr = new Uint8ClampedArray(WIDTH * HEIGHT * 4);
        const scene = wasm.Scene.new(WIDTH, HEIGHT);

        for (let y = HEIGHT - 1; y >= 0; y--) {
            let row = scene.image_row(y);
            for (let i = 0; i < row.length; i++) {
                arr[((HEIGHT - y - 1) * WIDTH * 4) + i] = row[i];
            }
            let imageData = new ImageData(arr, WIDTH);
            self.postMessage({ allGood: "processing", status: `${(HEIGHT - y) * 100 / HEIGHT}% complete`, imgData: imageData });
        }

        let imageData = new ImageData(arr, WIDTH);
        let t1 = new Date().getTime();
      self.postMessage({ allGood: true, imgData: imageData, time: "Finished in " + (t1 - t0) + " ms" });
    } catch (err) {
      self.postMessage({ allGood: false, error: err.message });
    }
  });
});

