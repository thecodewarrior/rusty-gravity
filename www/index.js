import { set_panic_hook, Renderer } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

set_panic_hook()

// Construct the universe, and get its width and height.
const renderer = Renderer.new();

const renderLoop = () => {
    renderer.draw();

    setTimeout(() => {
        requestAnimationFrame(renderLoop);
    }, 15)
};

renderer.draw();
requestAnimationFrame(renderLoop);
