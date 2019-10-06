import * as wand_app from "wand-example";

window.$mobile = /mobile/i.test(window.navigator.userAgent) || typeof window.orientation !== 'undefined'

window.requestAnimFrame = (function(callback) {
    return window.requestAnimationFrame || window.webkitRequestAnimationFrame || window.mozRequestAnimationFrame || window.oRequestAnimationFrame || window.msRequestAnimationFrame || function(callback) { window.setTimeout(callback, 1000 / 60); };
  })();

const app = wand_app.Application.new();
const canvas = document.getElementById("canvas");
const resize = () => {
    canvas.width = document.documentElement.clientWidth;
    canvas.height = document.documentElement.clientHeight;
    app.on_size_change();
    app.draw();
}

resize();

window.addEventListener("resize", resize);
window.addEventListener("mousemove", e => {
  let rec = canvas.getBoundingClientRect();
  app.on_mouse_move(e.clientX - rec.left, e.clientY - rec.top);
});

const renderer = () => {
  app.tick();
  window.requestAnimFrame(renderer);
}

window.requestAnimFrame(renderer);

window.addEventListener("keyup", e => {
  app.on_keyup(e.key);
});

window.addEventListener("keydown", e => {
  app.on_keydown(e.key);
});


