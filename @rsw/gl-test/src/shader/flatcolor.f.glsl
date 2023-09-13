precision mediump float;

uniform vec4 flatColor;

void main() {
    gl_FragColor = vec4(pow(flatColor.rgb, vec3(1.0/2.2)), flatColor.a);
}
