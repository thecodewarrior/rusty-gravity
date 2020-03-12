import { Universe, Cell, set_panic_hook } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

set_panic_hook()

const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');

const getIndex = (index, field) => {
    return index * 3 + field;
};

const drawBodies = () => {
    const bodiesPtr = universe.bodies();
    const bodyCount = universe.bodies_count();
    const bodies = new Float64Array(memory.buffer, bodiesPtr, bodyCount * 3);


    ctx.beginPath();

    ctx.fillStyle = ALIVE_COLOR;

    for (let bodyIndex = 0; bodyIndex < bodyCount; bodyIndex++) {
        const x = bodies[getIndex(bodyIndex, 0)];
        const y = bodies[getIndex(bodyIndex, 1)];
        const mass = bodies[getIndex(bodyIndex, 2)];

        ctx.arc(x, y, mass, 0, Math.PI*2, true);
        ctx.closePath();
    }

    ctx.fill();
};

const renderLoop = () => {
    universe.tick();

    drawBodies();

    setTimeout(() => {
        requestAnimationFrame(renderLoop);
    }, 15)
};

drawBodies();
requestAnimationFrame(renderLoop);
