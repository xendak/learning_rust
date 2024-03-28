use std::io ;

#[derive(Debug)]
enum TemperatureReading {
    Celsius,
    Fahrenheight,
    Kelvin,
}

#[derive(Debug)]
struct Temperature {
    degree: f32,
    sys: TemperatureReading,
}

impl Temperature {
    fn new(degree: f32, sys: TemperatureReading) -> Self {
        Temperature {
            degree,
            sys,
        }
    }
    fn display_all(&self) {
        let mut c = Self::new(0.0, TemperatureReading::Celsius);
        let mut k = Self::new(0.0, TemperatureReading::Kelvin);
        let mut f = Self::new(0.0, TemperatureReading::Fahrenheight);
        match self.sys {
            TemperatureReading::Fahrenheight => {
                k = Self::f_to_k(&self);
                c = Self::f_to_c(&self);
            }
            TemperatureReading::Celsius => {
                k = Self::c_to_k(&self);
                f = Self::c_to_f(&self);
            }
            TemperatureReading::Kelvin => {
                c = Self::k_to_c(&self);
                f = Self::k_to_f(&self);
            }
        };
        println!("Kelvin = {}K", k.degree);
        println!("Fahrenheight = {}F", f.degree);
        println!("Celsius = {}C", c.degree);
    }

    fn c_to_f(&self) -> Temperature {
        Temperature {
            degree: (self.degree * (9.0/5.0)) + 32.0,
            sys: TemperatureReading::Fahrenheight,
        }
    }
    fn f_to_c(&self) -> Temperature {
        Temperature {
            degree: (self.degree - 32.0) * (5.0/9.0),
            sys: TemperatureReading::Celsius,
        }
    }
    fn c_to_k(&self) -> Temperature {
        Temperature {
            degree: self.degree - 273.15,
            sys: TemperatureReading::Kelvin,
        }
    }
    fn k_to_c(&self) -> Temperature {
        Temperature {
            degree: self.degree + 273.15,
            sys: TemperatureReading::Celsius,
        }
    }

    fn k_to_f(&self) -> Temperature {
       Temperature { 
           degree: Self::c_to_f(&Self::k_to_c(&self)).degree,
           sys: TemperatureReading::Fahrenheight,
       }
    }
    fn f_to_k(&self) -> Temperature {
       Temperature { 
           degree: Self::f_to_c(&Self::c_to_k(&self)).degree,
           sys: TemperatureReading::Kelvin,
       }
    }
}

fn get_input() -> Temperature {
    loop {
        println!("Accepted systems are in the Celsius, Fahrenheight, Kelvin");
        println!("Please input a number followed by a temperature system.");

        let mut input = String::new();
        let mut sys: Option<TemperatureReading> = None;
        let mut temp: Option<f32> = None;
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Line");

        for (i, item) in input.trim().chars().enumerate() {
            if !item.is_ascii_digit() && !(item == '-') {
                sys = match item {
                    'c' | 'C' => Some(TemperatureReading::Celsius),
                    'k' | 'K' => Some(TemperatureReading::Kelvin),
                    'f' | 'F' => Some(TemperatureReading::Fahrenheight),
                    _ => continue,
                };
                temp = match input[..i].trim().parse() {
                    Ok(num) => Some(num),
                    Err(_) => continue
                };
                return Temperature::new(temp.expect("Need a valid number"), sys.expect("Needed a valid temperature"))
            }
        }
    }
}

fn main() {
    println!("Welcome to the temperature calculator!!");
    let t: Temperature = get_input();
    t.display_all();
}

