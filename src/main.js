const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function plot() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  var plotData = await invoke("plot");
  Plotly.newPlot("gd", {
    "data": [plotData],
    "layout": { "width": 600, "height": 400 }
  });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});
