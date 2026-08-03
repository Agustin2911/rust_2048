// Archivo: src/gui.rs

use dioxus::prelude::*;
use crate::game::Game;

// Funciones auxiliares para darle los colores exactos del 2048 original
fn obtener_color_fondo(valor: i32) -> &'static str {
    match valor {
        0 => "#cdc1b4",
        2 => "#eee4da",
        4 => "#ede0c8",
        8 => "#f2b179",
        16 => "#f59563",
        32 => "#f67c5f",
        64 => "#f65e3b",
        128 => "#edcf72",
        256 => "#edcc61",
        512 => "#edc850",
        1024 => "#edc53f",
        2048 => "#edc22e",
        _ => "#3c3a32", // Colores para números más grandes (4096+)
    }
}

fn obtener_color_texto(valor: i32) -> &'static str {
    if valor <= 4 {
        "#776e65" // Texto oscuro para 2 y 4
    } else {
        "#f9f6f2" // Texto claro para 8 en adelante
    }
}

#[component]
pub fn App() -> Element {
    let mut juego = use_signal(|| Game::new());
    
    rsx! {
        div {
            // Hacemos que este div pueda recibir eventos de teclado
            tabindex: "0",
            autofocus: "true",
            // onkeydown captura cuando presionás una tecla
            onkeydown: move |evento| {
                // Solo permitimos mover si el juego NO ha terminado
                if !juego.read().is_game_over() {
                    match evento.key() {
                        Key::ArrowUp => juego.write().move_up(),
                        Key::ArrowDown => juego.write().move_down(),
                        Key::ArrowLeft => juego.write().move_left(),
                        Key::ArrowRight => juego.write().move_right(),
                        _ => {} // Si toca cualquier otra tecla, no hacemos nada
                    }
                }
            },
            // outline: none evita que aparezca un recuadro feo al hacerle click
            style: "display: flex; flex-direction: column; align-items: center; font-family: sans-serif; height: 100vh; justify-content: center; background-color: #faf8ef; outline: none;",
            
            // --- ENCABEZADO ---
            div {
                style: "display: flex; justify-content: space-between; width: 430px; align-items: center; margin-bottom: 20px;",
                
                h1 { 
                    style: "font-size: 60px; color: #776e65; margin: 0; font-weight: bold;",
                    "2048" 
                }
                
                // Contenedor para los Puntos y el botón Reiniciar
                div {
                    style: "display: flex; gap: 10px;",
                    
                    // Cuadro de Puntaje
                    div {
                        style: "background-color: #bbada0; color: white; padding: 5px 20px; border-radius: 5px; text-align: center; font-weight: bold;",
                        "PUNTOS"
                        br {}
                        span { 
                            style: "font-size: 24px;", 
                            "{juego.read().points}" 
                        }
                    }
                    
                    // Botón para reiniciar partida en cualquier momento
                    button {
                        style: "background-color: #8f7a66; color: white; padding: 0 15px; border: none; border-radius: 5px; font-weight: bold; cursor: pointer;",
                        onclick: move |_| juego.set(Game::new()),
                        "Reiniciar"
                    }
                }
            }
            
            // --- CONTENEDOR DEL TABLERO ---
            // position: relative es clave para que el cartel de Game Over quede contenido acá adentro
            div {
                style: "position: relative; display: grid; grid-template-columns: repeat(4, 100px); gap: 10px; background: #bbada0; padding: 10px; border-radius: 10px;",
                
                // Iteramos sobre la matriz para dibujar las celdas
                for fila in 0..4 {
                    for col in 0..4 {
                        div {
                            style: "width: 100px; height: 100px; background: {obtener_color_fondo(juego.read().matrix[fila][col])}; display: flex; justify-content: center; align-items: center; font-size: 40px; font-weight: bold; border-radius: 5px; color: {obtener_color_texto(juego.read().matrix[fila][col])};",
                            
                            if juego.read().matrix[fila][col] != 0 {
                                "{juego.read().matrix[fila][col]}"
                            }
                        }
                    }
                }

                // --- CARTEL DE GAME OVER ---
                // Si la función devuelve true, Dioxus dibuja esto superpuesto
                if juego.read().is_game_over() {
                    div {
                        style: "position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(238, 228, 218, 0.73); z-index: 10; display: flex; flex-direction: column; justify-content: center; align-items: center; border-radius: 10px;",
                        
                        h2 {
                            style: "font-size: 50px; color: #776e65; margin: 0; margin-bottom: 20px;",
                            "¡Juego Terminado!"
                        }
                        
                        button {
                            style: "padding: 15px 30px; font-size: 20px; background-color: #8f7a66; color: white; border: none; border-radius: 5px; cursor: pointer; font-weight: bold;",
                            onclick: move |_| juego.set(Game::new()),
                            "Intentar de nuevo"
                        }
                    }
                }
            }

            // Instrucciones de movimiento en la parte inferior
            div {
                style: "margin-top: 30px; color: #776e65; font-weight: bold;",
                "Usa las flechas ⬆️⬇️⬅️➡️ para jugar"
            }
        }
    }
}