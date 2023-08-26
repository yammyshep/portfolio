precision mediump float;

attribute vec4 position;

uniform mat4 mvp;
uniform float time;

void main() {
    gl_Position = mvp * position;
}
