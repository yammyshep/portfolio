import init, { WebClient } from '@rsw/gl-test'

await init();
let client = new WebClient();
client.start();

let time = Date.now();
function render() {
    var newTime = Date.now()
    const dt = newTime - time;
    time = newTime;

    client.update(dt * 0.001);
    client.render();

    window.requestAnimationFrame(render);
}

render();
