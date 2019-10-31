const worker = new Worker("./worker.js");

const submitButton = document.querySelector("#submitButton");
const result = document.querySelector("#result");

worker.addEventListener("message", ev => {
  const message = ev.data;
  if (message.allGood) {
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');
        ctx.putImageData(message.imgData, 0, 0);
  } else {
    result.textContent = "Something went wrong! " + message.error;
  }
});

submitButton.addEventListener("click", () => worker.postMessage({}));
