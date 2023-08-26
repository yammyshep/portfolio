precision mediump float;

uniform float time;

void main() {
    gl_FragColor = vec4(abs(sin(time)), 0.07, 0.73, 1.0);
}
