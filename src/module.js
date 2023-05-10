import init, { greet, thing } from '@rsw/game-of-life';

await init();
console.log(thing());
greet("Someone!");
