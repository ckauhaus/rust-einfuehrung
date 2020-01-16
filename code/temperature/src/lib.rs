use std::ops::Deref;

#[derive(PartialEq, PartialOrd)]
struct Fahrenheit(f64);

#[derive(PartialEq, PartialOrd)]
struct Celsius(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

impl Deref for Celsius {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit((c.0 * 9.0 / 5.0) + 32.0)
    }
}

impl Deref for Fahrenheit {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn c2f() {
        let f = Fahrenheit::from(Celsius(100.0));
        assert_approx_eq!(*f, Fahrenheit(212.0).0);

        let f: Fahrenheit = Celsius(0.0).into();
        assert_approx_eq!(*f, Fahrenheit(32.0).0);
    }

    #[test]
    fn f2c() {
        let c = Celsius::from(Fahrenheit(96.0));
        assert_approx_eq!(*c, 35.555555);

        let c: Celsius = Fahrenheit(0.0).into();
        assert_approx_eq!(*c, -17.777777);
    }
}
