
extern crate aha;

fn main() {
    use aha::ga2::*;

    let x0 = -0.5;
    let dx = 0.03;
    let dy = 0.05;
    let ni = 60;
    let nj = 40;
    let nk = 255;

    for j in -nj/2..nj/2 {
        let y = j as f32 * dy;
        print!("{}", "     ");
        for i in -ni/2..ni/2 {
            let x = x0 + i as f32 * dx;
            let z0 = C(x, E12(y));
            let mut z = z0;
            let mut k = 0;
            while (k < nk) && (z.norm() < 10.) {
                z = z*z + z0;
                k += 1;
            }
            print!("{}", if k >= nk {"*"} else {" "});
        }
        println!("{}", "");
    }
}
