

use dioxus::prelude::*;
use crate::game::Game;

fn obtener_color_fondo(valor: i32) -> &'static str {
    match valor {
        0 => "#cdc1b4", 2 => "#eee4da", 4 => "#ede0c8", 8 => "#f2b179",
        16 => "#f59563", 32 => "#f67c5f", 64 => "#f65e3b", 128 => "#edcf72",
        256 => "#edcc61", 512 => "#edc850", 1024 => "#edc53f", 2048 => "#edc22e",
        _ => "#3c3a32",
    }
}

fn obtener_color_texto(valor: i32) -> &'static str {
    if valor <= 4 { "#776e65" } else { "#f9f6f2" }
}

#[component]
pub fn App() -> Element {
    let mut juego = use_signal(|| Game::new());
    
    let mut ha_ganado = use_signal(|| false);
    
    rsx! {
        div {
            tabindex: "0",
            autofocus: "true",
            onkeydown: move |evento| {
                if !juego.read().is_game_over() && !*ha_ganado.read() {
                    
                    let victoria = match evento.key() {
                        Key::ArrowUp => juego.write().move_up(),
                        Key::ArrowDown => juego.write().move_down(),
                        Key::ArrowLeft => juego.write().move_left(),
                        Key::ArrowRight => juego.write().move_right(),
                        _ => false,
                    };

                    if victoria {
                        ha_ganado.set(true);
                    }
                }
            },
            style: "display: flex; flex-direction: column; align-items: center; font-family: sans-serif; height: 100vh; justify-content: center; background-color: #faf8ef; outline: none;",
            
            div {
                style: "display: flex; justify-content: space-between; width: 430px; align-items: center; margin-bottom: 20px;",
                
                h1 { style: "font-size: 60px; color: #776e65; margin: 0; font-weight: bold;", "2048" }
                
                div {
                    style: "display: flex; gap: 10px;",
                    
                    div {
                        style: "background-color: #bbada0; color: white; padding: 5px 20px; border-radius: 5px; text-align: center; font-weight: bold;",
                        "PUNTOS"
                        br {}
                        span { style: "font-size: 24px;", "{juego.read().points}" }
                    }
                    
                    button {
                        style: "background-color: #8f7a66; color: white; padding: 0 15px; border: none; border-radius: 5px; font-weight: bold; cursor: pointer;",
                        onclick: move |_| {
                            juego.set(Game::new());
                            ha_ganado.set(false);
                        },
                        "Reiniciar"
                    }
                }
            }
            
            div {
                style: "position: relative; display: grid; grid-template-columns: repeat(4, 100px); gap: 10px; background: #bbada0; padding: 10px; border-radius: 10px;",
                
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

                if *ha_ganado.read() {
                    div {
                        style: "position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(237, 194, 46, 0.73); z-index: 10; display: flex; flex-direction: column; justify-content: center; align-items: center; border-radius: 10px;",
                        
                        h2 {
                            style: "font-size: 50px; color: white; margin: 0; margin-bottom: 20px; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);",
                            "¡Ganaste!"
                        }
                        
                        button {
                            style: "padding: 15px 30px; font-size: 20px; background-color: #8f7a66; color: white; border: none; border-radius: 5px; cursor: pointer; font-weight: bold;",
                            onclick: move |_| {
                                juego.set(Game::new());
                                ha_ganado.set(false);
                            },
                            "Jugar de nuevo"
                        }
                    }
                }

                if juego.read().is_game_over() && !*ha_ganado.read() {
                    div {
                        style: "position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(238, 228, 218, 0.73); z-index: 10; display: flex; flex-direction: column; justify-content: center; align-items: center; border-radius: 10px;",
                        
                        h2 {
                            style: "font-size: 50px; color: #776e65; margin: 0; margin-bottom: 20px;",
                            "¡Juego Terminado!"
                        }
                        
                        button {
                            style: "padding: 15px 30px; font-size: 20px; background-color: #8f7a66; color: white; border: none; border-radius: 5px; cursor: pointer; font-weight: bold;",
                            onclick: move |_| {
                                juego.set(Game::new());
                                ha_ganado.set(false);
                            },
                            "Intentar de nuevo"
                        }
                    }
                }
            }

            div {
                style: "margin-top: 30px; color: #776e65; font-weight: bold;",
                "Usa las flechas ⬆️⬇️⬅️➡️ para jugar"
            }
        }
    }
}