pub fn to_signed(p:u32,x:u32)->i32 {
    let mut y = x as i32;
    let s = 1 << (p - 1);
    if y >= s {
        y - 2*s
    } else {
	y
    }
}

pub fn to_ll(p:u32,sc:f64,x:u32)->f64 {
    let y = to_signed(p,x);
    y as f64/(60.0*sc)
}

pub fn to_range(s:f64,x_bot:u32,x_top:u32,x:u32)->f64 {
    if x == x_top {
        f64::INFINITY
    } else if x < x_bot || x > x_top {
        f64::NAN
    } else {
        x as f64*s
    }
}

pub fn to_range_signed(p:u32,s:f64,y_bot:i32,y_top:i32,x:u32)->f64 {
    let y = to_signed(p,x);
    if y == y_top {
        f64::INFINITY
    } else if y < y_bot || y > y_top {
        f64::NAN
    } else {
        y as f64*s
    }
}

pub const ITU : &[u8] = b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_ !\"#$%&'()*+,-./0123456789:;<=>?";

pub fn itu_to_char(x:u8)->char {
    ITU[x as usize].into()
}
