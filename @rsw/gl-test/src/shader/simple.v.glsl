attribute vec4 position;

uniform float rotation;

void main() {
    mat2 rotator = mat2(cos(rotation),sin(rotation),
                        -sin(rotation),cos(rotation));
    gl_Position = vec4(position.xy * rotator, position.zw);
}
