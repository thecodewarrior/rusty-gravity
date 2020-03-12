import { set_panic_hook, Game } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

set_panic_hook()

const game = Game.new();

const renderLoop = () => {
    game.frame();

    setTimeout(() => {
        requestAnimationFrame(renderLoop);
    }, 15)
};

game.frame();
requestAnimationFrame(renderLoop);
