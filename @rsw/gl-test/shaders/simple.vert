precision mediump float;

attribute vec4 position;
attribute vec3 normal;
attribute vec4 color;

varying vec3 frag_normal;
varying vec4 frag_color;

uniform mat4 mvp;
uniform mat4 normalMatrix;
uniform float time;

void main() {
    gl_Position = mvp * position;
    frag_normal = normalize(mat3(normalMatrix) * normal);
    frag_color = color;
}