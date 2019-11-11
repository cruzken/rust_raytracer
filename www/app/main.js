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
          ctx.putImageData(message.imgData, 0, 0);
          result.textContent = message.status;
  } else {
    result.textContent = "Something went wrong! " + message.error;
  }
});

submitButton.addEventListener("click", () => worker.postMessage({}));
