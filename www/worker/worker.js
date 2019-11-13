import("../../pkg").then(wasm => {
    let WIDTH;
    let HEIGHT;
    let world;
    let scene;

    self.addEventListener("message", ev => {
        const msg = ev.data;
        if (msg.init) {
            WIDTH = msg.width;
            HEIGHT = msg.height;
            world = msg.world;
            scene = wasm.Scene.new(WIDTH, HEIGHT, world);
            const arr = new Uint8ClampedArray(WIDTH * HEIGHT * 4);

            for (let y = HEIGHT - 1; y >= 0; y--) {
                let row = scene.image_row(y);
                for (let i = 0; i < row.length; i++) {
                    arr[((HEIGHT - y - 1) * WIDTH * 4) + i] = row[i]
                }
                self.postMessage({ allGood: "processing", status: `${(HEIGHT - y) * 100 / HEIGHT}% complete` });
                self.postMessage(arr);
            }

        }
    });
  });


