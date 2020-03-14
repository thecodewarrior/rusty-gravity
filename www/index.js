import { set_panic_hook, Game } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

set_panic_hook()

const game = Game.new();
const fpsElement = document.getElementById("fps");

let lastFrame = window.performance.now();
const renderLoop = () => {
    let start = window.performance.now();
    game.frame();
    let end = window.performance.now();
    let fps = 1000 / (end - lastFrame);
    let updateTime = end - start
    fpsElement.innerText = fps.toFixed(1) + " FPS (" + updateTime.toFixed(1) + "ms)";

    lastFrame = end;
    // setTimeout(() => {
        requestAnimationFrame(renderLoop);
    // }, 1000/30)
};

game.frame();
requestAnimationFrame(renderLoop);
