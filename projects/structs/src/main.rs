#[derive(Debug)]
struct Triangle {
    l: u32,
    b: u32,
    r: u32,
    sym: &'static str,
}

impl Triangle {
    fn create_triangle(l: u32, b: u32, r: u32) ->  Self {  // or Triangle here? why
        Triangle {
            l,
            b,
            r,
            sym: Self::check_type(b, l, r),
        }
    }
    fn check_type(b: u32, r: u32, l: u32) -> &'static str {
        if b == l && l == r {
            return "equilatero";
        } else if b == l || b == r || l == r {
            return "isoceles";
        } else {
            if  b*b == l*l + r*r || 
                l*l == b*b + r*r || 
                r*r == l*l + b*b {
                return "pitagoras"
            }
            "escaleno"
        }
    }

    fn get_area(&self) -> f64 {
        if self.sym == "pitagoras" {
            if self.b > self.l && self.b > self.r {
                return f64::from(self.l * self.r) / 2.0;
            } else if self.l > self.b && self.l > self.r {
                return f64::from(self.b * self.r) / 2.0;
            } else {
                return f64::from(self.b * self.l) / 2.0;
            }
        } else {
            f64::from(self.b) * Self::get_height(&self) / 2.0
        }
    }

    fn get_height(&self) -> f64 {
        let b2: f64 = f64::from(self.b) / 2.0;
        f64::sqrt(
            f64::from(self.r * self.r) - b2 * b2
        )
    }

    // changing each side
    fn set_base(&mut self, b: u32) {
        self.b = b;
        self.sym = Self::check_type(self.b, self.l, self.r);
    }
    fn set_left(&mut self, l: u32) {
        self.l = l;
        self.sym = Self::check_type(self.b, self.l, self.r);
    }
    fn set_right(&mut self, r: u32) {
        self.r = r;
        self.sym = Self::check_type(self.b, self.l, self.r);
    }
}

fn main() {
    let t: Triangle = Triangle::create_triangle(3, 4, 5);
    let mut other_t = Triangle::create_triangle(3, 4, 5);

    dbg!("other_t -> {}", &other_t);
    other_t.set_base(3);
    dbg!("x -> {}", &other_t.b);
    other_t.set_left(3);
    dbg!("y -> {}", &other_t.l);
    other_t.set_right(3);
    dbg!("z -> {}", &other_t.r);

    dbg!("other_t -> {}", &other_t);
    dbg!("area -> {}", &other_t.get_area());

    dbg!("for original t -> {}", &t);
    dbg!("area -> {}", &t.get_area());
}
