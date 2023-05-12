import init, { WebClient } from '@rsw/gl-test'

await init();
let client = new WebClient();
client.start();

let time = Date.now();
function render() {
    const dt = Date.now() - time;

    client.update(dt * 0.001);
    client.render();

    window.requestAnimationFrame(render);
    time = Date.now();
}

render();
