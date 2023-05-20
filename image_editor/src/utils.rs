



pub fn rgb_to_hsl(pixel:&image::Rgb<u8>) -> (f32, f32, f32) {
    let (r, g, b) = (pixel.0[0] as f32 / 255.0, pixel.0[1] as f32 / 255.0, pixel.0[2] as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let mut h = (max + min) / 2.0;
    let s ;
    let l = h;

    if max == min {
        h = 0.0;
        s = 0.0;
    } else {
        let d = max - min;
        s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };

        match max {
            _ if max == r => h = ((g - b) / 6.0)/d,
            _ if max == g => h = (1.0/3.0)+((b-r)/6.0)/d,
            _ if max == b => h = (2.0/3.0)+((r-g)/6.0)/d,
            _ => (),
        }

        if h <0.0{
            h+=1.0;
        }

        if h >1.0{
            h-=1.0;
        }

        //h /= 6.0;
        h=h*360.0;
    }

    (h, s, l)
}
