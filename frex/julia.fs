#version 330

in vec2 fragTexCoord;
out vec4 finalColor;

uniform vec2 resolution;
uniform vec2 pos;
uniform vec2 size;
uniform vec2 julia_param;

vec2 cx_sqr(vec2 z) {
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

float cx_magn_sq(vec2 z) {
    return z.x * z.x + z.y * z.y;
}

vec2 step(vec2 prev, vec2 c) {
    return cx_sqr(prev) + c;
}

int count_iters(vec2 coords, int max_iter) {
    vec2 z = coords;
    int iter = 0;
    while (cx_magn_sq(z) < 4.0 && iter < max_iter) {
        z = step(z, julia_param);
        iter += 1;
    }
    return iter;
}

vec4 color1(int max_iter, int iter) {
    if (iter == max_iter) {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }
    float frac = float(iter) / float(max_iter);
    return vec4(0.0, 1.0 - frac, frac / 2.0, 1.0);
}

void main()
{
    vec2 p = fragTexCoord;
    vec2 fc = p * size + pos;
    int max_iter = 150;
    int iter = count_iters(fc, max_iter);
    //if (pos.x < -1.6) {
    //    finalColor = vec4(1.0, 0.5, 0.0, 1.0);
    //} else {
    //    finalColor = vec4(pos.x, 1.0, 0.0, 1.0);
    //}
    finalColor = color1(max_iter, iter);
}

