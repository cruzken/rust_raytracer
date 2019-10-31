import("../../pkg").then(wasm => {
  self.addEventListener("message", () => {
    try {
        const WIDTH = 100;
        const HEIGHT = 50;
      const imgData = wasm.gen_image(WIDTH, HEIGHT);
        const arr = new Uint8ClampedArray(WIDTH * HEIGHT * 4);

        for (let i = 0; i < arr.length; i += 4) {
            arr[i + 0] = imgData[i + 0];
            arr[i + 1] = imgData[i + 1];
            arr[i + 2] = imgData[i + 2];
            arr[i + 3] = imgData[i + 3];
        }
        let imageData = new ImageData(arr, WIDTH);
      self.postMessage({ allGood: true, imgData: imageData });
    } catch (err) {
      self.postMessage({ allGood: false, error: err.message });
    }
  });
});
