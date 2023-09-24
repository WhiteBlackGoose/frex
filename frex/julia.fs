#version 330

in vec2 fragTexCoord;
out vec4 finalColor;

uniform vec2 resolution;
uniform vec2 pos;
uniform vec2 size;
uniform vec2 julia_param;

uniform vec4 wk_colors[5];
// vec4 wk_colors[5] = {         
//    {1.0, 1.0, 1.0, 1.0}, 
//    {1.0, 1.0, 1.0, 1.0}, 
//    {1.0, 1.0, 1.0, 1.0}, 
//    {1.0, 1.0, 1.0, 1.0}, 
//    {1.0, 1.0, 1.0, 1.0}
//};                            
//vec4 wk_colors[5] = {
//    vec4(1.0, 1.0, 1.0, 1.0),
//    vec4(1.0, 1.0, 1.0, 1.0),
//    vec4(1.0, 1.0, 1.0, 1.0),
//    vec4(1.0, 1.0, 1.0, 1.0),
//    vec4(1.0, 1.0, 1.0, 1.0),
//};
uniform float wk_pos[5];

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

vec4 color2(int max_iter, int iter) {
    float h = float(iter) / float(max_iter);
    int hi = int(h * 6);
    float s = 1.0;
    float v = 0.7;
    float f = h * 6 - float(hi);

    float p = v * (1 - s);
    float q = v * (1 - s * f);
    float t = v * (1 - s * (1 - f));
    if (hi == 1) {
        return vec4(q, v, p, 1.0);
    } else if (hi == 2) {
        return vec4(p, v, t, 1.0);
    } else if (hi == 3) {
        return vec4(p, q, v, 1.0);
    } else if (hi == 4) {
        return vec4(t, v, p, 1.0);
    } else if (hi == 5) {
        return vec4(v, p, q, 1.0);
    } else {
        return vec4(v, t, p, 1.0);
    }
}

// https://stackoverflow.com/questions/16500656/which-color-gradient-is-used-to-color-mandelbrot-in-wikipedia
vec4 color_wikipedia(int max_iter, int iter) {
    float h = float(iter) / float(max_iter);
    h = h * 0.85;
    for (int i = 0; i < 4; i++) {
        if (h <= wk_pos[i]) {
            float t;
            if (i == 0)
                t = h / wk_pos[0];
            else 
                t = (h - wk_pos[i-1]) / (wk_pos[i] - wk_pos[i - 1]);
            t = 1 - t;
            return wk_colors[i]/255.0 * t + wk_colors[i + 1]/255.0 * (1 - t);
        }
    }
    return vec4(0.0,1.0, 0.0, 1.0);
}

void main()
{
    //finalColor = wk_colors[1] / 255.0;
    //finalColor = vec4(1.0);
    vec2 p = fragTexCoord;
    vec2 fc = p * size + pos;
    int max_iter = 150;
    int iter = count_iters(fc, max_iter);
    finalColor = color_wikipedia(max_iter, iter);
    //finalColor = color_wikipedia(150, int(150.0 * p.x));
}

