precision mediump float;

attribute vec4 position;
attribute vec3 normal;

varying vec3 frag_normal;

uniform mat4 mvp;
uniform mat4 normalMatrix;
uniform float time;

void main() {
    gl_Position = mvp * position;
    frag_normal = normalize(mat3(normalMatrix) * normal);
}