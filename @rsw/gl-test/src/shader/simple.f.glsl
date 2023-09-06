precision mediump float;

varying vec3 frag_normal;

//uniform float time;
uniform vec4 ambientLightColor;
uniform vec4 directionalLightColor;
uniform vec3 directionalLightDir;

void main() {
    //gl_FragColor = vec4(abs(sin(time)), 0.07, 0.73, 1.0);

    float shadeFactor = max(0.0, dot(normalize(frag_normal), directionalLightDir));
    vec3 irradiance = (ambientLightColor.rgb * ambientLightColor.a) + 
        (directionalLightColor.rgb * directionalLightColor.a) * shadeFactor;

    gl_FragColor = vec4(pow(irradiance, vec3(1.0/2.2)), 1.0);
}
