use crossterm::{cursor, QueueableCommand};
use std::f64::consts::PI;
use std::io::{stdout, Write};
use std::{thread, time};

fn main() {
    const SCREEN_WIDTH: f64 = 60.0;
    const SCREEN_HEIGHT: f64 = 60.0;

    const THETA_SPACING: f64 = 0.07;
    const PHI_SPACING: f64 = 0.02;

    const R1: f64 = 1.0;
    const R2: f64 = 2.0;
    const K2: f64 = 5.0;
    const K1: f64 = SCREEN_WIDTH * K2 * 3.0 / (8.0 * (R1 + R2));

    fn render_frame(A: f64, B: f64) {
        let (sinA, cosA) = A.sin_cos();
        let (sinB, cosB) = B.sin_cos();

        let mut output = vec![vec![' ' as u8; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
        let mut zbuffer = vec![vec![0.0; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

        let mut theta = 0.0;
        while theta < 2.0 * PI {
            let (sintheta, costheta) = theta.sin_cos();
            let mut phi = 0.0;
            while phi < 2.0 * PI {
                let (sinphi, cosphi) = phi.sin_cos();

                let circlex = R2 + R1 * costheta;
                let circley = R1 * sintheta;

                let x = circlex * (cosB * cosphi + sinA * sinB * sinphi) - circley * cosA * sinB;
                let y = circlex * (sinB * cosphi - sinA * cosB * sinphi) + circley * cosA * cosB;
                let z = K2 + cosA * circlex * sinphi + circley * sinA;
                let ooz = z.recip();

                let xp = (SCREEN_WIDTH / 2.0 + K1 * ooz * x) as usize;
                let yp = (SCREEN_HEIGHT / 2.0 - K1 * ooz * y) as usize;

                let L = cosphi * costheta * sinB - cosA * costheta * sinphi - sinA * sintheta
                    + cosB * (cosA * sintheta - costheta * sinA * sinphi);

                if L > 0.0 {
                    if ooz > zbuffer[yp][xp] {
                        zbuffer[yp][xp] = ooz;
                        let luminance_index = (L * 8.0) as usize;
                        output[yp][xp] = ".,-~:;=!*#$@".as_bytes()[luminance_index];
                    }
                }
                phi += PHI_SPACING;
            }
            theta += THETA_SPACING;
        }
        let mut stdout = stdout();
        for row in output {
            for val in row {
                stdout.write(&[val as u8]).unwrap();
            }
            stdout.write(&['\n' as u8]).unwrap();
        }
        stdout.queue(cursor::MoveUp(SCREEN_HEIGHT as u16)).unwrap();
        // stdout.write("\x1b[23A".as_bytes()).unwrap();
    }

    // for i in 0..1000 {
    //     render_frame(i as f64 / 20.0, i as f64 / 20.0);
    //     thread::sleep(time::Duration::from_millis(50));
    // }

    let mut rot = 0.0;
    loop {
        render_frame(rot, rot);
        thread::sleep(time::Duration::from_millis(50));
        rot += 0.05
    }
}
