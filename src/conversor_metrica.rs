pub fn conversor_metrica() {
    // Projeto 1: Conversor e Normalizador de Métricas.

    type Celsius = f64;
    type Fahrenheit = f64;
    type Kelvin = f64;
    type Simbolo = char;

    let simbolo_c: Simbolo = 'C';
    let simbolo_f: Simbolo = 'F';
    let simbolo_k: Simbolo = 'K';

    let temp_c: Celsius = 25.0;

    let calculo_temp_f: Fahrenheit = temp_c * (9.0 / 5.0) + 32.0;
    let calculo_temp_k: Kelvin = temp_c + 273.15;

    println!("A temperatura em Celsius: {temp_c:.2} °{simbolo_c}");
    println!("A temperatura em Fahrenheit: {calculo_temp_f:.2} °{simbolo_f}");
    println!("A temperatura em Kelvin: {calculo_temp_k:.2} °{simbolo_k}");
}
