import cm_mrt
import sym_gen_util as util
from sympy.matrices import Matrix
from sympy import Symbol
from sympy.simplify.simplify import simplify

def moments_rust_header():
    return '''\
pub fn moments(f: [f32; 27]) -> (f32, [f32;3]) {
'''

def moments_rust_f64_header():
    return '''\
pub fn moments_f64(f: [f64; 27]) -> (f64, [f64;3]) {
'''

def moments_rust_footer():
    return '''\
    (density, [ux/density, uy/density, uz/density]) 
}\n\n
'''

def moments_shader_header():
    return '''\
pub fn wgsl_moments() -> &'static str {
    &"
fn moments(index: i32) {
'''

def moments_shader_footer():
    return '''\
    densities[index] = density;
    set_velocity(index, vec3(ux/density, uy/density, uz/density)); 
}
"
}
'''

def gen_moments_ops(rust_src_dir, shader_src_dir, debug_dir):
    print("Generating moments_op")
    name = "moments"

    f = cm_mrt.f()
    moment_op = cm_mrt.M(Matrix([[0], [0], [0]])) * f

    (source_body, debug_raw) = util.rust_generate_moment_op(simplify(moment_op).evalf())
    util.write_ops_debug(name, debug_raw, debug_dir)

    rust_source = moments_rust_header()
    rust_source += util.rust_fi_def()
    rust_source += source_body
    rust_source += moments_rust_footer()
    util.write_rust_ops(name, rust_source, rust_src_dir)

    f64_rust_source = moments_rust_f64_header()
    f64_rust_source += util.rust_fi_def()
    f64_rust_source += source_body
    f64_rust_source += moments_rust_footer()
    f64_name = f"{name}_f64"
    util.write_rust_ops(f64_name, f64_rust_source, rust_src_dir)

    shader_source = moments_shader_header()
    shader_source += util.shader_fi_def()
    shader_source += source_body
    shader_source += moments_shader_footer()
    util.write_rust_ops(name, shader_source, shader_src_dir)
