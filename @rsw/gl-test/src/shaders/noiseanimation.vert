precision mediump float;

attribute vec4 position;
attribute vec3 normal;
attribute vec4 color;

varying vec3 frag_normal;
varying vec4 frag_color;

uniform mat4 mvp;
uniform mat4 normalMatrix;
uniform float time;

#include "perlin.glsl"

void main() {
    vec4 pos = position;
    pos.z += cnoise(vec3(position.xy * 7.5, time * 0.2)) * 0.05;
    gl_Position = mvp * pos;
    frag_normal = normalize(mat3(normalMatrix) * normal);
    frag_color = color;
}