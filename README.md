# Einführung in Rust

Übungsaufgaben zum Vortrag "Einführung in Rust" im kraut.space Jena, 16.01.2020.

# Übung 1: Moves

Bringe den folgenden Code zum compilieren. Dabei dürfen die Zeilen mit
`println!()` nicht verändert werden!

```
// https://github.com/rust-lang/rustlings exercise move_semantics2.rs

fn main() {
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
```


# Übung 2: Fehlerbehandlung

Der folgende Code definiert einen neuen Typen `PositiveNonzeroInteger`, der
keine Null und keine negativen Zahlen annimmt. Allerdings fehlt die
entsprechende Fehlererzeugung in `new()` bisher. Ergänze den Code so, dass die
passende Variante von `CreationError` zurückgegeben wird und der Test
erfolgreich ist.

```
// https://github.com/rust-lang/rustlings exercise error_handling/result1.rs

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(Err(CreationError::Negative), PositiveNonzeroInteger::new(-10));
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
```


# Übung 3: Trump-o-Meter

Erweitere Trump-o-Meter aus dem Vortrag um zusätzliche Funktionalität.

## 3a. Timing

Gib für jede Newssite die Zeit zum Laden aus!

Tip: `std::time::Instant`

## 3b. Eingabeformat

Schreibt einen Parser für ein alternatives Eingabeformat. URL und Name werden
mit Komma getrennt:
```
https://www.bild.de/,BILD
http://www.spiegel.de/,Spiegel
https://www.focus.de/,Focus
...
```

## 3c. Free style

Bau etwas weiteres Nettes nach Lust und Laune ein.
