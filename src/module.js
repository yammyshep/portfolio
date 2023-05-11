import init, { WebClient } from '@rsw/gl-test'

await init();
let client = new WebClient().constructor;
client.start();

let time = Date.now();
function render() {
    const dt = Date.now() - time;

    client.update(dt);
    client.render();

    window.requestAnimationFrame(render);
    time = Date.now();
}

render();
