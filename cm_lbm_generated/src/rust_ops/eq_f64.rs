pub fn eq_f64(ux: f64, uy: f64, uz: f64, density: f64) -> [f64; 27] {
    let mut result = [0.0; 27];
    result[0] = 0.296296296296296
        * density
        * (-3.0 * ux * ux - 3.0 * uy * uy - 3.0 * uz * uz + 1.0);

    result[1] = 0.0740740740740741
        * density
        * (6.0 * ux * ux + 3.0 * ux - 3.0 * uy * uy - 3.0 * uz * uz + 1.0);

    result[2] = 0.0740740740740741
        * density
        * (6.0 * ux * ux - 3.0 * ux - 3.0 * uy * uy - 3.0 * uz * uz + 1.0);

    result[3] = 0.0740740740740741
        * density
        * (-3.0 * ux * ux + 6.0 * uy * uy + 3.0 * uy - 3.0 * uz * uz + 1.0);

    result[4] = 0.0740740740740741
        * density
        * (-3.0 * ux * ux + 6.0 * uy * uy - 3.0 * uy - 3.0 * uz * uz + 1.0);

    result[5] = 0.0740740740740741
        * density
        * (-3.0 * ux * ux - 3.0 * uy * uy + 6.0 * uz * uz + 3.0 * uz + 1.0);

    result[6] = 0.0740740740740741
        * density
        * (-3.0 * ux * ux - 3.0 * uy * uy + 6.0 * uz * uz - 3.0 * uz + 1.0);

    result[7] = 0.0185185185185185
        * density
        * (6.0 * ux * ux
            + 18.0 * ux * uy
            + 3.0 * ux
            + 6.0 * uy * uy
            + 3.0 * uy
            - 3.0 * uz * uz
            + 1.0);

    result[8] = 0.0185185185185185
        * density
        * (6.0 * ux * ux - 18.0 * ux * uy + 3.0 * ux + 6.0 * uy * uy
            - 3.0 * uy
            - 3.0 * uz * uz
            + 1.0);

    result[9] = 0.0185185185185185
        * density
        * (6.0 * ux * ux - 18.0 * ux * uy - 3.0 * ux
            + 6.0 * uy * uy
            + 3.0 * uy
            - 3.0 * uz * uz
            + 1.0);

    result[10] = 0.0185185185185185
        * density
        * (6.0 * ux * ux + 18.0 * ux * uy - 3.0 * ux + 6.0 * uy * uy
            - 3.0 * uy
            - 3.0 * uz * uz
            + 1.0);

    result[11] = 0.0185185185185185
        * density
        * (6.0 * ux * ux + 18.0 * ux * uz + 3.0 * ux - 3.0 * uy * uy
            + 6.0 * uz * uz
            + 3.0 * uz
            + 1.0);

    result[12] = 0.0185185185185185
        * density
        * (6.0 * ux * ux - 18.0 * ux * uz + 3.0 * ux - 3.0 * uy * uy
            + 6.0 * uz * uz
            - 3.0 * uz
            + 1.0);

    result[13] = 0.0185185185185185
        * density
        * (6.0 * ux * ux - 18.0 * ux * uz - 3.0 * ux - 3.0 * uy * uy
            + 6.0 * uz * uz
            + 3.0 * uz
            + 1.0);

    result[14] = 0.0185185185185185
        * density
        * (6.0 * ux * ux + 18.0 * ux * uz - 3.0 * ux - 3.0 * uy * uy
            + 6.0 * uz * uz
            - 3.0 * uz
            + 1.0);

    result[15] = 0.0185185185185185
        * density
        * (-3.0 * ux * ux
            + 6.0 * uy * uy
            + 18.0 * uy * uz
            + 3.0 * uy
            + 6.0 * uz * uz
            + 3.0 * uz
            + 1.0);

    result[16] = 0.0185185185185185
        * density
        * (-3.0 * ux * ux + 6.0 * uy * uy - 18.0 * uy * uz
            + 3.0 * uy
            + 6.0 * uz * uz
            - 3.0 * uz
            + 1.0);

    result[17] = 0.0185185185185185
        * density
        * (-3.0 * ux * ux + 6.0 * uy * uy - 18.0 * uy * uz - 3.0 * uy
            + 6.0 * uz * uz
            + 3.0 * uz
            + 1.0);

    result[18] = 0.0185185185185185
        * density
        * (-3.0 * ux * ux + 6.0 * uy * uy + 18.0 * uy * uz - 3.0 * uy
            + 6.0 * uz * uz
            - 3.0 * uz
            + 1.0);

    result[19] = 0.00462962962962963
        * density
        * (-3.0 * ux * ux + 3.0 * ux - 3.0 * uy * uy + 3.0 * uy
            - 3.0 * uz * uz
            + 3.0 * uz
            + 9.0 * (ux + uy + uz) * (ux + uy + uz)
            + 1.0);

    result[20] = -0.00462962962962963
        * density
        * (3.0 * ux * ux - 3.0 * ux + 3.0 * uy * uy - 3.0 * uy
            + 3.0 * uz * uz
            + 3.0 * uz
            - 9.0 * (ux + uy - uz) * (ux + uy - uz)
            - 1.0);

    result[21] = -0.00462962962962963
        * density
        * (3.0 * ux * ux - 3.0 * ux + 3.0 * uy * uy + 3.0 * uy + 3.0 * uz * uz
            - 3.0 * uz
            - 9.0 * (ux - uy + uz) * (ux - uy + uz)
            - 1.0);

    result[22] = -0.00462962962962963
        * density
        * (3.0 * ux * ux + 3.0 * ux + 3.0 * uy * uy - 3.0 * uy + 3.0 * uz * uz
            - 3.0 * uz
            - 9.0 * (-ux + uy + uz) * (-ux + uy + uz)
            - 1.0);

    result[23] = -0.00462962962962963
        * density
        * (3.0 * ux * ux - 3.0 * ux
            + 3.0 * uy * uy
            + 3.0 * uy
            + 3.0 * uz * uz
            + 3.0 * uz
            - 9.0 * (-ux + uy + uz) * (-ux + uy + uz)
            - 1.0);

    result[24] = -0.00462962962962963
        * density
        * (3.0 * ux * ux + 3.0 * ux + 3.0 * uy * uy + 3.0 * uy + 3.0 * uz * uz
            - 3.0 * uz
            - 9.0 * (ux + uy - uz) * (ux + uy - uz)
            - 1.0);

    result[25] = -0.00462962962962963
        * density
        * (3.0 * ux * ux + 3.0 * ux + 3.0 * uy * uy - 3.0 * uy
            + 3.0 * uz * uz
            + 3.0 * uz
            - 9.0 * (ux - uy + uz) * (ux - uy + uz)
            - 1.0);

    result[26] = -0.00462962962962963
        * density
        * (3.0 * ux * ux
            + 3.0 * ux
            + 3.0 * uy * uy
            + 3.0 * uy
            + 3.0 * uz * uz
            + 3.0 * uz
            - 9.0 * (ux + uy + uz) * (ux + uy + uz)
            - 1.0);

    result
}
