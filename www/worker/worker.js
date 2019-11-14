import("../../pkg")
  .then(wasm => {
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
        self.postMessage({
          allGood: "ready"
        });
      } else if (msg.job) {
        let row = scene.image_row(msg.count);
        self.postMessage({
          row: msg.count,
          imgRow: row
        });
      }
    });
      self.postMessage("loaded");
  });
