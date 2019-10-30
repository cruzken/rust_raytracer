import * as rt from "rust_raytracer";


const test = rt.gen_image();

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const arr = new Uint8ClampedArray(80000);

for (let i = 0; i < arr.length; i += 4) {
    arr[i + 0] = test[i + 0];
    arr[i + 1] = test[i + 1];
    arr[i + 2] = test[i + 2];
    arr[i + 3] = test[i + 3];
}

let imageData = new ImageData(arr, 200);

ctx.putImageData(imageData, 0, 0);
