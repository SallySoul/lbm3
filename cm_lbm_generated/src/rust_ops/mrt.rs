pub fn mrt(
    f: [f32; 27],
    ux: f32,
    uy: f32,
    uz: f32,
    density: f32,
    riv: f32,
) -> [f32; 27] {
    let mut result = [0.0; 27];
    let q0 = f[0];
    let q1 = f[1];
    let q2 = f[2];
    let q3 = f[3];
    let q4 = f[4];
    let q5 = f[5];
    let q6 = f[6];
    let q7 = f[7];
    let q8 = f[8];
    let q9 = f[9];
    let q10 = f[10];
    let q11 = f[11];
    let q12 = f[12];
    let q13 = f[13];
    let q14 = f[14];
    let q15 = f[15];
    let q16 = f[16];
    let q17 = f[17];
    let q18 = f[18];
    let q19 = f[19];
    let q20 = f[20];
    let q21 = f[21];
    let q22 = f[22];
    let q23 = f[23];
    let q24 = f[24];
    let q25 = f[25];
    let q26 = f[26];
    result[0] = -0.871800647655485 * density * ux * ux
        - 0.871800647655485 * density * uy * uy
        - 0.871800647655484 * density * uz * uz
        - 1.37183350925717 * density
        + 1.0 * q0
        + 1.94174757281553 * q1
        + 1.96410934908596 * q10
        + 1.96410934908596 * q11
        + 1.96410934908596 * q12
        + 1.96410934908596 * q13
        + 1.96410934908596 * q14
        + 1.96410934908596 * q15
        + 1.96410934908596 * q16
        + 1.96410934908596 * q17
        + 1.96410934908596 * q18
        + 1.95387778164147 * q19
        + 1.94174757281553 * q2
        + 1.95387778164147 * q20
        + 1.95387778164147 * q21
        + 1.95387778164147 * q22
        + 1.95387778164147 * q23
        + 1.95387778164147 * q24
        + 1.95387778164147 * q25
        + 1.95387778164147 * q26
        + 1.94174757281553 * q3
        + 1.94174757281553 * q4
        + 1.94174757281553 * q5
        + 1.94174757281553 * q6
        + 1.96410934908596 * q7
        + 1.96410934908596 * q8
        + 1.96410934908596 * q9;

    result[1] = 0.333333333333333 * density * riv * ux * ux
        - 0.166666666666667 * density * riv * uy * uy
        - 0.166666666666667 * density * riv * uz * uz
        - 0.211348867110769 * density * ux * ux
        + 1.14307253812719e-18 * density * ux * uy
        + 1.21451457176014e-18 * density * ux * uz
        + 0.458169320552106 * density * ux
        + 0.108548765646749 * density * uy * uy
        + 1.21451457176014e-18 * density * uy * uz
        + 0.108548765646749 * density * uz * uz
        + 0.145300107942581 * density
        - 0.333333333333333 * q1 * riv
        - 0.323624595469256 * q1
        - 0.166666666666667 * q10 * riv
        + 0.341569920926274 * q10
        - 0.166666666666667 * q11 * riv
        + 0.283317493741808 * q11
        - 0.166666666666667 * q12 * riv
        + 0.283317493741808 * q12
        - 0.166666666666667 * q13 * riv
        + 0.341569920926274 * q13
        - 0.166666666666667 * q14 * riv
        + 0.341569920926274 * q14
        + 0.333333333333333 * q15 * riv
        - 0.647249190938511 * q15
        + 0.333333333333333 * q16 * riv
        - 0.647249190938511 * q16
        + 0.333333333333333 * q17 * riv
        - 0.647249190938511 * q17
        + 0.333333333333333 * q18 * riv
        - 0.647249190938511 * q18
        - 0.00190324687778156 * q19
        - 0.333333333333333 * q2 * riv
        + 0.676375404530744 * q2
        - 0.00190324687778156 * q20
        - 0.00190324687778156 * q21
        + 0.01213481432227 * q22
        - 0.00190324687778156 * q23
        + 0.01213481432227 * q24
        + 0.01213481432227 * q25
        + 0.01213481432227 * q26
        + 0.166666666666667 * q3 * riv
        - 0.323624595469256 * q3
        + 0.166666666666667 * q4 * riv
        - 0.323624595469256 * q4
        + 0.166666666666667 * q5 * riv
        - 0.323624595469256 * q5
        + 0.166666666666667 * q6 * riv
        - 0.323624595469256 * q6
        - 0.166666666666667 * q7 * riv
        + 0.283317493741808 * q7
        - 0.166666666666667 * q8 * riv
        + 0.283317493741808 * q8
        - 0.166666666666667 * q9 * riv
        + 0.341569920926274 * q9;

    result[2] = 0.333333333333333 * density * riv * ux * ux
        - 0.166666666666667 * density * riv * uy * uy
        - 0.166666666666667 * density * riv * uz * uz
        - 0.211348867110769 * density * ux * ux
        - 4.28652201797698e-18 * density * ux * uy
        - 1.34311023229945e-17 * density * ux * uz
        - 0.458169320552106 * density * ux
        + 0.108548765646749 * density * uy * uy
        - 4.28652201797698e-18 * density * uy * uz
        + 0.108548765646749 * density * uz * uz
        + 0.145300107942581 * density
        - 0.333333333333333 * q1 * riv
        + 0.676375404530744 * q1
        - 0.166666666666667 * q10 * riv
        + 0.283317493741808 * q10
        - 0.166666666666667 * q11 * riv
        + 0.341569920926274 * q11
        - 0.166666666666667 * q12 * riv
        + 0.341569920926274 * q12
        - 0.166666666666667 * q13 * riv
        + 0.283317493741808 * q13
        - 0.166666666666667 * q14 * riv
        + 0.283317493741808 * q14
        + 0.333333333333333 * q15 * riv
        - 0.647249190938511 * q15
        + 0.333333333333333 * q16 * riv
        - 0.647249190938511 * q16
        + 0.333333333333333 * q17 * riv
        - 0.647249190938511 * q17
        + 0.333333333333333 * q18 * riv
        - 0.647249190938511 * q18
        + 0.01213481432227 * q19
        - 0.333333333333333 * q2 * riv
        - 0.323624595469256 * q2
        + 0.01213481432227 * q20
        + 0.01213481432227 * q21
        - 0.00190324687778156 * q22
        + 0.01213481432227 * q23
        - 0.00190324687778156 * q24
        - 0.00190324687778156 * q25
        - 0.00190324687778156 * q26
        + 0.166666666666667 * q3 * riv
        - 0.323624595469256 * q3
        + 0.166666666666667 * q4 * riv
        - 0.323624595469256 * q4
        + 0.166666666666667 * q5 * riv
        - 0.323624595469256 * q5
        + 0.166666666666667 * q6 * riv
        - 0.323624595469256 * q6
        - 0.166666666666667 * q7 * riv
        + 0.341569920926274 * q7
        - 0.166666666666667 * q8 * riv
        + 0.341569920926274 * q8
        - 0.166666666666667 * q9 * riv
        + 0.283317493741808 * q9;

    result[3] = -0.166666666666667 * density * riv * ux * ux
        + 0.333333333333333 * density * riv * uy * uy
        - 0.166666666666667 * density * riv * uz * uz
        + 0.108548765646749 * density * ux * ux
        + 1.14307253812719e-18 * density * ux * uy
        + 1.21451457176014e-18 * density * ux * uz
        - 0.211348867110769 * density * uy * uy
        + 1.21451457176014e-18 * density * uy * uz
        + 0.458169320552106 * density * uy
        + 0.108548765646749 * density * uz * uz
        + 0.145300107942581 * density
        + 0.166666666666667 * q1 * riv
        - 0.323624595469256 * q1
        - 0.166666666666667 * q10 * riv
        + 0.341569920926274 * q10
        + 0.333333333333333 * q11 * riv
        - 0.647249190938511 * q11
        + 0.333333333333333 * q12 * riv
        - 0.647249190938511 * q12
        + 0.333333333333333 * q13 * riv
        - 0.647249190938511 * q13
        + 0.333333333333333 * q14 * riv
        - 0.647249190938511 * q14
        - 0.166666666666667 * q15 * riv
        + 0.283317493741808 * q15
        - 0.166666666666667 * q16 * riv
        + 0.283317493741808 * q16
        - 0.166666666666667 * q17 * riv
        + 0.341569920926274 * q17
        - 0.166666666666667 * q18 * riv
        + 0.341569920926274 * q18
        - 0.00190324687778156 * q19
        + 0.166666666666667 * q2 * riv
        - 0.323624595469256 * q2
        - 0.00190324687778156 * q20
        + 0.01213481432227 * q21
        - 0.00190324687778156 * q22
        + 0.01213481432227 * q23
        + 0.01213481432227 * q24
        - 0.00190324687778156 * q25
        + 0.01213481432227 * q26
        - 0.333333333333333 * q3 * riv
        - 0.323624595469256 * q3
        - 0.333333333333333 * q4 * riv
        + 0.676375404530744 * q4
        + 0.166666666666667 * q5 * riv
        - 0.323624595469256 * q5
        + 0.166666666666667 * q6 * riv
        - 0.323624595469256 * q6
        - 0.166666666666667 * q7 * riv
        + 0.283317493741808 * q7
        - 0.166666666666667 * q8 * riv
        + 0.341569920926274 * q8
        - 0.166666666666667 * q9 * riv
        + 0.283317493741808 * q9;

    result[4] = -0.166666666666667 * density * riv * ux * ux
        + 0.333333333333333 * density * riv * uy * uy
        - 0.166666666666667 * density * riv * uz * uz
        + 0.108548765646749 * density * ux * ux
        - 4.28652201797698e-18 * density * ux * uy
        - 4.28652201797698e-18 * density * ux * uz
        - 0.211348867110769 * density * uy * uy
        - 1.34311023229945e-17 * density * uy * uz
        - 0.458169320552106 * density * uy
        + 0.108548765646749 * density * uz * uz
        + 0.145300107942581 * density
        + 0.166666666666667 * q1 * riv
        - 0.323624595469256 * q1
        - 0.166666666666667 * q10 * riv
        + 0.283317493741808 * q10
        + 0.333333333333333 * q11 * riv
        - 0.647249190938511 * q11
        + 0.333333333333333 * q12 * riv
        - 0.647249190938511 * q12
        + 0.333333333333333 * q13 * riv
        - 0.647249190938511 * q13
        + 0.333333333333333 * q14 * riv
        - 0.647249190938511 * q14
        - 0.166666666666667 * q15 * riv
        + 0.341569920926274 * q15
        - 0.166666666666667 * q16 * riv
        + 0.341569920926274 * q16
        - 0.166666666666667 * q17 * riv
        + 0.283317493741808 * q17
        - 0.166666666666667 * q18 * riv
        + 0.283317493741808 * q18
        + 0.01213481432227 * q19
        + 0.166666666666667 * q2 * riv
        - 0.323624595469256 * q2
        + 0.01213481432227 * q20
        - 0.00190324687778156 * q21
        + 0.01213481432227 * q22
        - 0.00190324687778156 * q23
        - 0.00190324687778156 * q24
        + 0.01213481432227 * q25
        - 0.00190324687778156 * q26
        - 0.333333333333333 * q3 * riv
        + 0.676375404530744 * q3
        - 0.333333333333333 * q4 * riv
        - 0.323624595469256 * q4
        + 0.166666666666667 * q5 * riv
        - 0.323624595469256 * q5
        + 0.166666666666667 * q6 * riv
        - 0.323624595469256 * q6
        - 0.166666666666667 * q7 * riv
        + 0.341569920926274 * q7
        - 0.166666666666667 * q8 * riv
        + 0.283317493741808 * q8
        - 0.166666666666667 * q9 * riv
        + 0.341569920926274 * q9;

    result[5] = -0.166666666666667 * density * riv * ux * ux
        - 0.166666666666667 * density * riv * uy * uy
        + 0.333333333333333 * density * riv * uz * uz
        + 0.108548765646749 * density * ux * ux
        + 1.21451457176014e-18 * density * ux * uy
        + 1.14307253812719e-18 * density * ux * uz
        + 0.108548765646749 * density * uy * uy
        + 1.21451457176014e-18 * density * uy * uz
        - 0.211348867110769 * density * uz * uz
        + 0.458169320552106 * density * uz
        + 0.145300107942581 * density
        + 0.166666666666667 * q1 * riv
        - 0.323624595469256 * q1
        + 0.333333333333333 * q10 * riv
        - 0.647249190938511 * q10
        - 0.166666666666667 * q11 * riv
        + 0.283317493741808 * q11
        - 0.166666666666667 * q12 * riv
        + 0.341569920926274 * q12
        - 0.166666666666667 * q13 * riv
        + 0.283317493741808 * q13
        - 0.166666666666667 * q14 * riv
        + 0.341569920926274 * q14
        - 0.166666666666667 * q15 * riv
        + 0.283317493741808 * q15
        - 0.166666666666667 * q16 * riv
        + 0.341569920926274 * q16
        - 0.166666666666667 * q17 * riv
        + 0.283317493741808 * q17
        - 0.166666666666667 * q18 * riv
        + 0.341569920926274 * q18
        - 0.00190324687778156 * q19
        + 0.166666666666667 * q2 * riv
        - 0.323624595469256 * q2
        + 0.01213481432227 * q20
        - 0.00190324687778156 * q21
        - 0.00190324687778156 * q22
        + 0.01213481432227 * q23
        - 0.00190324687778156 * q24
        + 0.01213481432227 * q25
        + 0.01213481432227 * q26
        + 0.166666666666667 * q3 * riv
        - 0.323624595469256 * q3
        + 0.166666666666667 * q4 * riv
        - 0.323624595469256 * q4
        - 0.333333333333333 * q5 * riv
        - 0.323624595469256 * q5
        - 0.333333333333333 * q6 * riv
        + 0.676375404530744 * q6
        + 0.333333333333333 * q7 * riv
        - 0.647249190938511 * q7
        + 0.333333333333333 * q8 * riv
        - 0.647249190938511 * q8
        + 0.333333333333333 * q9 * riv
        - 0.647249190938511 * q9;

    result[6] = -0.166666666666667 * density * riv * ux * ux
        - 0.166666666666667 * density * riv * uy * uy
        + 0.333333333333333 * density * riv * uz * uz
        + 0.108548765646749 * density * ux * ux
        + 1.40026385920581e-17 * density * ux * uy
        - 1.34311023229945e-17 * density * ux * uz
        + 0.108548765646749 * density * uy * uy
        - 1.34311023229945e-17 * density * uy * uz
        - 0.211348867110769 * density * uz * uz
        - 0.458169320552106 * density * uz
        + 0.145300107942581 * density
        + 0.166666666666667 * q1 * riv
        - 0.323624595469256 * q1
        + 0.333333333333333 * q10 * riv
        - 0.647249190938511 * q10
        - 0.166666666666667 * q11 * riv
        + 0.341569920926274 * q11
        - 0.166666666666667 * q12 * riv
        + 0.283317493741808 * q12
        - 0.166666666666667 * q13 * riv
        + 0.341569920926274 * q13
        - 0.166666666666667 * q14 * riv
        + 0.283317493741808 * q14
        - 0.166666666666667 * q15 * riv
        + 0.341569920926274 * q15
        - 0.166666666666667 * q16 * riv
        + 0.283317493741808 * q16
        - 0.166666666666667 * q17 * riv
        + 0.341569920926274 * q17
        - 0.166666666666667 * q18 * riv
        + 0.283317493741808 * q18
        + 0.01213481432227 * q19
        + 0.166666666666667 * q2 * riv
        - 0.323624595469256 * q2
        - 0.00190324687778156 * q20
        + 0.01213481432227 * q21
        + 0.01213481432227 * q22
        - 0.00190324687778156 * q23
        + 0.01213481432227 * q24
        - 0.00190324687778156 * q25
        - 0.00190324687778156 * q26
        + 0.166666666666667 * q3 * riv
        - 0.323624595469256 * q3
        + 0.166666666666667 * q4 * riv
        - 0.323624595469256 * q4
        - 0.333333333333333 * q5 * riv
        + 0.676375404530744 * q5
        - 0.333333333333333 * q6 * riv
        - 0.323624595469256 * q6
        + 0.333333333333333 * q7 * riv
        - 0.647249190938511 * q7
        + 0.333333333333333 * q8 * riv
        - 0.647249190938511 * q8
        + 0.333333333333333 * q9 * riv
        - 0.647249190938511 * q9;

    result[7] = -1.37168704575263e-17 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uy
        - 1.37168704575263e-17 * density * riv * uy * uy
        + 1.71460880719079e-18 * density * riv * uz * uz
        + 0.107537914911254 * density * ux * ux
        - 0.159948816378759 * density * ux * uy
        + 0.109103041989319 * density * ux
        + 0.107537914911254 * density * uy * uy
        + 0.109103041989319 * density * uy
        - 0.0524109014675052 * density * uz * uz
        + 0.0358459716370845 * density
        - 0.25 * q10 * riv
        + 0.491027337271491 * q10
        - 0.25 * q19 * riv
        + 0.44959093021534 * q19
        - 0.25 * q20 * riv
        + 0.44959093021534 * q20
        + 0.25 * q21 * riv
        - 0.487994785065006 * q21
        + 0.25 * q22 * riv
        - 0.487994785065006 * q22
        + 0.25 * q23 * riv
        - 0.487994785065006 * q23
        - 0.25 * q24 * riv
        + 0.493805296199754 * q24
        + 0.25 * q25 * riv
        - 0.487994785065006 * q25
        - 0.25 * q26 * riv
        + 0.493805296199754 * q26
        - 0.25 * q7 * riv
        - 0.450720235544043 * q7
        + 0.25 * q8 * riv
        - 0.479846449136276 * q8
        + 0.25 * q9 * riv
        - 0.479846449136276 * q9;

    result[8] = -1.20022616503355e-17 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uy
        - 1.20022616503355e-17 * density * riv * uy * uy
        + 1.71460880719079e-18 * density * riv * uz * uz
        + 0.107537914911253 * density * ux * ux
        + 0.159948816378759 * density * ux * uy
        + 3.42921761438158e-18 * density * ux * uz
        + 0.109103041989319 * density * ux
        + 0.107537914911253 * density * uy * uy
        - 0.109103041989319 * density * uy
        - 0.0524109014675052 * density * uz * uz
        + 0.0358459716370845 * density
        + 0.25 * q10 * riv
        - 0.479846449136276 * q10
        + 0.25 * q19 * riv
        - 0.487994785065006 * q19
        + 0.25 * q20 * riv
        - 0.487994785065006 * q20
        - 0.25 * q21 * riv
        + 0.44959093021534 * q21
        - 0.25 * q22 * riv
        + 0.493805296199754 * q22
        - 0.25 * q23 * riv
        + 0.44959093021534 * q23
        + 0.25 * q24 * riv
        - 0.487994785065006 * q24
        - 0.25 * q25 * riv
        + 0.493805296199754 * q25
        + 0.25 * q26 * riv
        - 0.487994785065006 * q26
        + 0.25 * q7 * riv
        - 0.479846449136276 * q7
        - 0.25 * q8 * riv
        - 0.450720235544043 * q8
        - 0.25 * q9 * riv
        + 0.491027337271491 * q9;

    result[9] = -1.20022616503355e-17 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uy
        - 1.20022616503355e-17 * density * riv * uy * uy
        + 1.71460880719079e-18 * density * riv * uz * uz
        + 0.107537914911253 * density * ux * ux
        + 0.159948816378759 * density * ux * uy
        - 0.109103041989319 * density * ux
        + 0.107537914911253 * density * uy * uy
        + 3.42921761438158e-18 * density * uy * uz
        + 0.109103041989319 * density * uy
        - 0.0524109014675052 * density * uz * uz
        + 0.0358459716370845 * density
        + 0.25 * q10 * riv
        - 0.479846449136276 * q10
        + 0.25 * q19 * riv
        - 0.487994785065006 * q19
        + 0.25 * q20 * riv
        - 0.487994785065006 * q20
        - 0.25 * q21 * riv
        + 0.493805296199754 * q21
        - 0.25 * q22 * riv
        + 0.44959093021534 * q22
        - 0.25 * q23 * riv
        + 0.493805296199754 * q23
        + 0.25 * q24 * riv
        - 0.487994785065006 * q24
        - 0.25 * q25 * riv
        + 0.44959093021534 * q25
        + 0.25 * q26 * riv
        - 0.487994785065006 * q26
        + 0.25 * q7 * riv
        - 0.479846449136276 * q7
        - 0.25 * q8 * riv
        + 0.491027337271491 * q8
        - 0.25 * q9 * riv
        - 0.450720235544043 * q9;

    result[10] = -1.37168704575263e-17 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uy
        - 1.37168704575263e-17 * density * riv * uy * uy
        + 1.71460880719079e-18 * density * riv * uz * uz
        + 0.107537914911254 * density * ux * ux
        - 0.159948816378759 * density * ux * uy
        + 3.42921761438158e-18 * density * ux * uz
        - 0.109103041989319 * density * ux
        + 0.107537914911254 * density * uy * uy
        + 3.42921761438158e-18 * density * uy * uz
        - 0.109103041989319 * density * uy
        - 0.0524109014675052 * density * uz * uz
        + 0.0358459716370845 * density
        - 0.25 * q10 * riv
        - 0.450720235544043 * q10
        - 0.25 * q19 * riv
        + 0.493805296199754 * q19
        - 0.25 * q20 * riv
        + 0.493805296199754 * q20
        + 0.25 * q21 * riv
        - 0.487994785065006 * q21
        + 0.25 * q22 * riv
        - 0.487994785065006 * q22
        + 0.25 * q23 * riv
        - 0.487994785065006 * q23
        - 0.25 * q24 * riv
        + 0.44959093021534 * q24
        + 0.25 * q25 * riv
        - 0.487994785065006 * q25
        - 0.25 * q26 * riv
        + 0.44959093021534 * q26
        - 0.25 * q7 * riv
        + 0.491027337271491 * q7
        + 0.25 * q8 * riv
        - 0.479846449136276 * q8
        + 0.25 * q9 * riv
        - 0.479846449136276 * q9;

    result[11] = -1.37168704575263e-17 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uz
        + 1.71460880719079e-18 * density * riv * uy * uy
        - 1.37168704575263e-17 * density * riv * uz * uz
        + 0.107537914911254 * density * ux * ux
        - 0.159948816378759 * density * ux * uz
        + 0.109103041989319 * density * ux
        - 0.0524109014675052 * density * uy * uy
        + 0.107537914911254 * density * uz * uz
        + 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        - 0.25 * q11 * riv
        - 0.450720235544043 * q11
        + 0.25 * q12 * riv
        - 0.479846449136276 * q12
        + 0.25 * q13 * riv
        - 0.479846449136276 * q13
        - 0.25 * q14 * riv
        + 0.491027337271491 * q14
        - 0.25 * q19 * riv
        + 0.44959093021534 * q19
        + 0.25 * q20 * riv
        - 0.487994785065006 * q20
        - 0.25 * q21 * riv
        + 0.44959093021534 * q21
        + 0.25 * q22 * riv
        - 0.487994785065006 * q22
        + 0.25 * q23 * riv
        - 0.487994785065006 * q23
        + 0.25 * q24 * riv
        - 0.487994785065006 * q24
        - 0.25 * q25 * riv
        + 0.493805296199754 * q25
        - 0.25 * q26 * riv
        + 0.493805296199754 * q26;

    result[12] = -1.20022616503355e-17 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uz
        + 1.71460880719079e-18 * density * riv * uy * uy
        - 1.20022616503355e-17 * density * riv * uz * uz
        + 0.107537914911253 * density * ux * ux
        + 3.42921761438158e-18 * density * ux * uy
        + 0.159948816378759 * density * ux * uz
        + 0.109103041989319 * density * ux
        - 0.0524109014675052 * density * uy * uy
        + 0.107537914911253 * density * uz * uz
        - 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        + 0.25 * q11 * riv
        - 0.479846449136276 * q11
        - 0.25 * q12 * riv
        - 0.450720235544043 * q12
        - 0.25 * q13 * riv
        + 0.491027337271491 * q13
        + 0.25 * q14 * riv
        - 0.479846449136276 * q14
        + 0.25 * q19 * riv
        - 0.487994785065006 * q19
        - 0.25 * q20 * riv
        + 0.44959093021534 * q20
        + 0.25 * q21 * riv
        - 0.487994785065006 * q21
        - 0.25 * q22 * riv
        + 0.493805296199754 * q22
        - 0.25 * q23 * riv
        + 0.44959093021534 * q23
        - 0.25 * q24 * riv
        + 0.493805296199754 * q24
        + 0.25 * q25 * riv
        - 0.487994785065006 * q25
        + 0.25 * q26 * riv
        - 0.487994785065006 * q26;

    result[13] = -1.20022616503355e-17 * density * riv * ux * ux
        - 0.25 * density * riv * ux * uz
        + 1.71460880719079e-18 * density * riv * uy * uy
        - 1.20022616503355e-17 * density * riv * uz * uz
        + 0.107537914911253 * density * ux * ux
        + 0.159948816378759 * density * ux * uz
        - 0.109103041989319 * density * ux
        - 0.0524109014675052 * density * uy * uy
        + 3.42921761438158e-18 * density * uy * uz
        + 0.107537914911253 * density * uz * uz
        + 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        + 0.25 * q11 * riv
        - 0.479846449136276 * q11
        - 0.25 * q12 * riv
        + 0.491027337271491 * q12
        - 0.25 * q13 * riv
        - 0.450720235544043 * q13
        + 0.25 * q14 * riv
        - 0.479846449136276 * q14
        + 0.25 * q19 * riv
        - 0.487994785065006 * q19
        - 0.25 * q20 * riv
        + 0.493805296199754 * q20
        + 0.25 * q21 * riv
        - 0.487994785065006 * q21
        - 0.25 * q22 * riv
        + 0.44959093021534 * q22
        - 0.25 * q23 * riv
        + 0.493805296199754 * q23
        - 0.25 * q24 * riv
        + 0.44959093021534 * q24
        + 0.25 * q25 * riv
        - 0.487994785065006 * q25
        + 0.25 * q26 * riv
        - 0.487994785065006 * q26;

    result[14] = -1.37168704575263e-17 * density * riv * ux * ux
        + 0.25 * density * riv * ux * uz
        + 1.71460880719079e-18 * density * riv * uy * uy
        - 1.37168704575263e-17 * density * riv * uz * uz
        + 0.107537914911254 * density * ux * ux
        + 3.42921761438158e-18 * density * ux * uy
        - 0.159948816378759 * density * ux * uz
        - 0.109103041989319 * density * ux
        - 0.0524109014675052 * density * uy * uy
        + 3.42921761438158e-18 * density * uy * uz
        + 0.107537914911254 * density * uz * uz
        - 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        - 0.25 * q11 * riv
        + 0.491027337271491 * q11
        + 0.25 * q12 * riv
        - 0.479846449136276 * q12
        + 0.25 * q13 * riv
        - 0.479846449136276 * q13
        - 0.25 * q14 * riv
        - 0.450720235544043 * q14
        - 0.25 * q19 * riv
        + 0.493805296199754 * q19
        + 0.25 * q20 * riv
        - 0.487994785065006 * q20
        - 0.25 * q21 * riv
        + 0.493805296199754 * q21
        + 0.25 * q22 * riv
        - 0.487994785065006 * q22
        + 0.25 * q23 * riv
        - 0.487994785065006 * q23
        + 0.25 * q24 * riv
        - 0.487994785065006 * q24
        - 0.25 * q25 * riv
        + 0.44959093021534 * q25
        - 0.25 * q26 * riv
        + 0.44959093021534 * q26;

    result[15] = 1.71460880719079e-18 * density * riv * ux * ux
        - 1.37168704575263e-17 * density * riv * uy * uy
        + 0.25 * density * riv * uy * uz
        - 1.37168704575263e-17 * density * riv * uz * uz
        - 0.0524109014675052 * density * ux * ux
        + 0.107537914911254 * density * uy * uy
        - 0.159948816378759 * density * uy * uz
        + 0.109103041989319 * density * uy
        + 0.107537914911254 * density * uz * uz
        + 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        - 0.25 * q15 * riv
        - 0.450720235544043 * q15
        + 0.25 * q16 * riv
        - 0.479846449136276 * q16
        + 0.25 * q17 * riv
        - 0.479846449136276 * q17
        - 0.25 * q18 * riv
        + 0.491027337271491 * q18
        - 0.25 * q19 * riv
        + 0.44959093021534 * q19
        + 0.25 * q20 * riv
        - 0.487994785065006 * q20
        + 0.25 * q21 * riv
        - 0.487994785065006 * q21
        - 0.25 * q22 * riv
        + 0.44959093021534 * q22
        - 0.25 * q23 * riv
        + 0.493805296199754 * q23
        + 0.25 * q24 * riv
        - 0.487994785065006 * q24
        + 0.25 * q25 * riv
        - 0.487994785065006 * q25
        - 0.25 * q26 * riv
        + 0.493805296199754 * q26;

    result[16] = 1.71460880719079e-18 * density * riv * ux * ux
        - 1.20022616503355e-17 * density * riv * uy * uy
        - 0.25 * density * riv * uy * uz
        - 1.20022616503355e-17 * density * riv * uz * uz
        - 0.0524109014675052 * density * ux * ux
        + 3.42921761438158e-18 * density * ux * uy
        + 0.107537914911253 * density * uy * uy
        + 0.159948816378759 * density * uy * uz
        + 0.109103041989319 * density * uy
        + 0.107537914911253 * density * uz * uz
        - 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        + 0.25 * q15 * riv
        - 0.479846449136276 * q15
        - 0.25 * q16 * riv
        - 0.450720235544043 * q16
        - 0.25 * q17 * riv
        + 0.491027337271491 * q17
        + 0.25 * q18 * riv
        - 0.479846449136276 * q18
        + 0.25 * q19 * riv
        - 0.487994785065006 * q19
        - 0.25 * q20 * riv
        + 0.44959093021534 * q20
        - 0.25 * q21 * riv
        + 0.493805296199754 * q21
        + 0.25 * q22 * riv
        - 0.487994785065006 * q22
        + 0.25 * q23 * riv
        - 0.487994785065006 * q23
        - 0.25 * q24 * riv
        + 0.493805296199754 * q24
        - 0.25 * q25 * riv
        + 0.44959093021534 * q25
        + 0.25 * q26 * riv
        - 0.487994785065006 * q26;

    result[17] = 1.71460880719079e-18 * density * riv * ux * ux
        - 1.20022616503355e-17 * density * riv * uy * uy
        - 0.25 * density * riv * uy * uz
        - 1.20022616503355e-17 * density * riv * uz * uz
        - 0.0524109014675052 * density * ux * ux
        + 3.42921761438158e-18 * density * ux * uz
        + 0.107537914911253 * density * uy * uy
        + 0.159948816378759 * density * uy * uz
        - 0.109103041989319 * density * uy
        + 0.107537914911253 * density * uz * uz
        + 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        + 0.25 * q15 * riv
        - 0.479846449136276 * q15
        - 0.25 * q16 * riv
        + 0.491027337271491 * q16
        - 0.25 * q17 * riv
        - 0.450720235544043 * q17
        + 0.25 * q18 * riv
        - 0.479846449136276 * q18
        + 0.25 * q19 * riv
        - 0.487994785065006 * q19
        - 0.25 * q20 * riv
        + 0.493805296199754 * q20
        - 0.25 * q21 * riv
        + 0.44959093021534 * q21
        + 0.25 * q22 * riv
        - 0.487994785065006 * q22
        + 0.25 * q23 * riv
        - 0.487994785065006 * q23
        - 0.25 * q24 * riv
        + 0.44959093021534 * q24
        - 0.25 * q25 * riv
        + 0.493805296199754 * q25
        + 0.25 * q26 * riv
        - 0.487994785065006 * q26;

    result[18] = 1.71460880719079e-18 * density * riv * ux * ux
        - 1.37168704575263e-17 * density * riv * uy * uy
        + 0.25 * density * riv * uy * uz
        - 1.37168704575263e-17 * density * riv * uz * uz
        - 0.0524109014675052 * density * ux * ux
        + 3.42921761438158e-18 * density * ux * uy
        + 3.42921761438158e-18 * density * ux * uz
        + 0.107537914911254 * density * uy * uy
        - 0.159948816378759 * density * uy * uz
        - 0.109103041989319 * density * uy
        + 0.107537914911254 * density * uz * uz
        - 0.109103041989319 * density * uz
        + 0.0358459716370845 * density
        - 0.25 * q15 * riv
        + 0.491027337271491 * q15
        + 0.25 * q16 * riv
        - 0.479846449136276 * q16
        + 0.25 * q17 * riv
        - 0.479846449136276 * q17
        - 0.25 * q18 * riv
        - 0.450720235544043 * q18
        - 0.25 * q19 * riv
        + 0.493805296199754 * q19
        + 0.25 * q20 * riv
        - 0.487994785065006 * q20
        + 0.25 * q21 * riv
        - 0.487994785065006 * q21
        - 0.25 * q22 * riv
        + 0.493805296199754 * q22
        - 0.25 * q23 * riv
        + 0.44959093021534 * q23
        + 0.25 * q24 * riv
        - 0.487994785065006 * q24
        + 0.25 * q25 * riv
        - 0.487994785065006 * q25
        - 0.25 * q26 * riv
        + 0.44959093021534 * q26;

    result[19] = 0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        - 0.9099121294718 * q19
        + 0.00960096371241642 * q20
        + 0.00960096371241642 * q21
        + 0.00960096371241642 * q22
        - 0.00145262778368719 * q23
        - 0.00145262778368719 * q24
        - 0.00145262778368719 * q25
        - 0.0013253311445766 * q26;

    result[20] = 0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        + 0.00960096371241642 * q19
        - 0.9099121294718 * q20
        - 0.00145262778368719 * q21
        - 0.00145262778368719 * q22
        + 0.00960096371241642 * q23
        - 0.0013253311445766 * q24
        + 0.00960096371241642 * q25
        - 0.00145262778368719 * q26;

    result[21] = 0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        + 0.00960096371241642 * q19
        - 0.00145262778368719 * q20
        - 0.9099121294718 * q21
        - 0.00145262778368719 * q22
        + 0.00960096371241642 * q23
        + 0.00960096371241642 * q24
        - 0.0013253311445766 * q25
        - 0.00145262778368719 * q26;

    result[22] = 0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        + 0.00960096371241642 * q19
        - 0.00145262778368719 * q20
        - 0.00145262778368719 * q21
        - 0.9099121294718 * q22
        - 0.0013253311445766 * q23
        + 0.00960096371241642 * q24
        + 0.00960096371241642 * q25
        - 0.00145262778368719 * q26;

    result[23] = 0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        + 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        - 0.00145262778368719 * q19
        + 0.00960096371241642 * q20
        + 0.00960096371241642 * q21
        - 0.0013253311445766 * q22
        - 0.9099121294718 * q23
        - 0.00145262778368719 * q24
        - 0.00145262778368719 * q25
        + 0.00960096371241642 * q26;

    result[24] = 0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        - 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        + 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        - 0.00145262778368719 * q19
        - 0.0013253311445766 * q20
        + 0.00960096371241642 * q21
        + 0.00960096371241642 * q22
        - 0.00145262778368719 * q23
        - 0.9099121294718 * q24
        - 0.00145262778368719 * q25
        + 0.00960096371241642 * q26;

    result[25] = 0.0262054507337526 * density * ux * ux
        - 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        - 0.0799744081893794 * density * uy * uz
        + 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        - 0.00145262778368719 * q19
        + 0.00960096371241642 * q20
        - 0.0013253311445766 * q21
        + 0.00960096371241642 * q22
        - 0.00145262778368719 * q23
        - 0.00145262778368719 * q24
        - 0.9099121294718 * q25
        + 0.00960096371241642 * q26;

    result[26] = 0.0262054507337526 * density * ux * ux
        + 0.0799744081893794 * density * ux * uy
        + 0.0799744081893794 * density * ux * uz
        - 0.0263546278726544 * density * ux
        + 0.0262054507337526 * density * uy * uy
        + 0.0799744081893794 * density * uy * uz
        - 0.0263546278726544 * density * uy
        + 0.0262054507337526 * density * uz * uz
        - 0.0263546278726544 * density * uz
        + 0.00873515024458421 * density
        - 0.0013253311445766 * q19
        - 0.00145262778368719 * q20
        - 0.00145262778368719 * q21
        - 0.00145262778368719 * q22
        + 0.00960096371241642 * q23
        + 0.00960096371241642 * q24
        + 0.00960096371241642 * q25
        - 0.9099121294718 * q26;

    result
}
