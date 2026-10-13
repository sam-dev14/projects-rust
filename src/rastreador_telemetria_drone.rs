pub fn rastreador_drone() {

    // Projeto 2: Rastreador de Telemetria de Drone.

    type Ponto3D = (f64, f64, f64); // Latitude, Longitude e Altitude
    type StatusDrone = (Ponto3D, f32, bool); // Coordenadas, bateria_restante e motor_ligado.

    let coordenadas: Ponto3D = (-22.67439, -45.64839, 174.0);
    let drone: StatusDrone = (coordenadas, 85.0, true);

    let consulta = drone.0.2;

    let (posicao, bateria, ativo) = drone; // Inferencia de tipo, puxa da variável drone.

    println!("A altitude é: {consulta}");
    println!("A posição é: {:?}", posicao);
    println!("A bateria está em: {bateria}%");
    println!("O status do motor é: {ativo}");
}
